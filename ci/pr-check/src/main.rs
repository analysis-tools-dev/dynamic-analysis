//! PR contribution checker for analysis-tools-dev/dynamic-analysis.
//!
//! Reads new or modified YAML files under `data/tools/` that were introduced
//! by a pull request, fetches metadata from the GitHub API for each tool's
//! source repository, and evaluates each tool against the contributing
//! criteria:
//!
//! - At least 20 stars
//! - More than one human contributor (excluding bots and known automation)
//! - Repository is at least 6 calendar months old
//!
//! The results are either posted as a single comment on the PR (updating an
//! existing bot comment if one already exists) or written to a file when the
//! `COMMENT_OUTPUT_FILE` environment variable is set. The latter mode is used
//! in CI to work around the GitHub Actions restriction that prevents fork PRs
//! from writing to the base repository. A separate `pr-comment` workflow then
//! picks up the file and posts the comment with the necessary permissions.
//!
//! The process exits with a non-zero status code when any hard criterion is
//! not met, causing CI to fail.
//!
//! Expected environment variables:
//!   GITHUB_TOKEN        - a token for GitHub API reads; `pull-requests: write`
//!                         is required only when posting comments directly
//!                         (COMMENT_OUTPUT_FILE is unset)
//!   GITHUB_REPOSITORY   - owner/repo, e.g. "analysis-tools-dev/dynamic-analysis"
//!   PR_NUMBER           - the pull request number
//!   COMMENT_OUTPUT_FILE - (optional) path to write the rendered comment body
//!                         to instead of posting it directly via the API.

use anyhow::{Context, Result, bail};
use askama::Template;
use chrono::{DateTime, Months, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

/// A minimal tool entry parsed from `data/tools/<name>.yml`.
/// Only the fields needed for the contributing criteria check are required.
#[derive(Debug, Deserialize)]
struct ToolEntry {
    name: String,
    source: Option<String>,
}

/// Response from `GET /repos/{owner}/{repo}`.
#[derive(Debug, Deserialize)]
struct RepoInfo {
    stargazers_count: u64,
    created_at: DateTime<Utc>,
}

/// One item from `GET /repos/{owner}/{repo}/contributors`.
#[derive(Debug, Deserialize)]
struct Contributor {
    login: String,
    #[serde(rename = "type")]
    account_type: String,
}

impl Contributor {
    fn counts_as_human(&self) -> bool {
        let login = self.login.to_ascii_lowercase();
        self.account_type.eq_ignore_ascii_case("User")
            && !login.ends_with("[bot]")
            && !AUTOMATION_LOGINS.contains(&login.as_str())
    }
}

// GitHub reports some automation as users. Exact logins avoid excluding humans
// with similar names; keep this list aligned with static-analysis.
const AUTOMATION_LOGINS: &[&str] = &["claude", "dependabot", "renovate-bot"];

/// One PR comment from `GET /repos/{owner}/{repo}/issues/{pr}/comments`.
#[derive(Debug, Deserialize)]
struct IssueComment {
    id: u64,
    body: String,
}

const MIN_STARS: u64 = 20;
const MIN_CONTRIBUTORS: usize = 2;
const MIN_AGE_MONTHS: u32 = 6;

// Marker text embedded in every comment we post so we can find and update it.
const COMMENT_MARKER: &str = "<!-- pr-check-bot -->";

/// The outcome of one criterion check.
#[derive(Debug)]
enum CheckResult {
    Pass(String),
    Fail(String),
    Skip(String),
}

impl CheckResult {
    fn is_fail(&self) -> bool {
        matches!(self, Self::Fail(_))
    }

    fn is_skip(&self) -> bool {
        matches!(self, Self::Skip(_))
    }

    fn symbol(&self) -> &'static str {
        match self {
            Self::Pass(_) => "pass",
            Self::Fail(_) => "fail",
            Self::Skip(_) => "skip",
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::Pass(m) | Self::Fail(m) | Self::Skip(m) => m,
        }
    }
}

/// All checks for a single tool.
#[derive(Debug)]
struct ToolReport {
    name: String,
    source: Option<String>,
    stars: CheckResult,
    contributors: CheckResult,
    age: CheckResult,
    /// Non-GitHub source repositories cannot be checked automatically.
    note: Option<String>,
}

impl ToolReport {
    fn any_fail(&self) -> bool {
        self.stars.is_fail() || self.contributors.is_fail() || self.age.is_fail()
    }

    fn needs_review(&self) -> bool {
        self.stars.is_skip() || self.contributors.is_skip() || self.age.is_skip()
    }

    fn status(&self) -> &'static str {
        if self.any_fail() {
            "FAIL"
        } else if self.needs_review() {
            "REVIEW"
        } else {
            "PASS"
        }
    }
}

#[derive(Template)]
#[template(path = "comment.md")]
struct CommentTemplate<'a> {
    marker: &'a str,
    reports: &'a [ToolReport],
    any_failures: bool,
    any_incomplete: bool,
}

struct GithubClient {
    client: reqwest::Client,
    token: String,
}

impl GithubClient {
    /// Creates a new client.
    ///
    /// # Errors
    ///
    /// Returns an error if the `reqwest` client cannot be constructed.
    fn new(token: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("pr-check-bot/1.0 (analysis-tools-dev)")
            .build()
            .context("Failed to build HTTP client")?;
        Ok(Self { client, token })
    }

    /// Sends an authenticated GET request and deserialises the JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the response cannot be
    /// deserialised as `T`.
    async fn get<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<Option<T>> {
        let resp = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .with_context(|| format!("GET {url} failed"))?;

        let status = resp.status();
        if status == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GET {url} returned {status}: {body}");
        }

        resp.json::<T>()
            .await
            .with_context(|| format!("Failed to deserialise response from {url}"))
            .map(Some)
    }

    /// Fetches repository metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn repo_info(&self, owner: &str, repo: &str) -> Result<Option<RepoInfo>> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}");
        self.get::<RepoInfo>(&url).await
    }

    /// Fetches the contributor list (up to 100, which is enough to confirm
    /// whether there is more than one human contributor).
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn contributor_count(&self, owner: &str, repo: &str) -> Result<Option<usize>> {
        let url =
            format!("https://api.github.com/repos/{owner}/{repo}/contributors?per_page=100&anon=0");
        let Some(contributors) = self.get::<Vec<Contributor>>(&url).await? else {
            return Ok(None);
        };
        // Exclude bot accounts from the contributor count.
        let human_count = contributors.iter().filter(|c| c.counts_as_human()).count();
        Ok(Some(human_count))
    }

    /// Lists all comments on a PR/issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn list_pr_comments(&self, repo: &str, pr: u64) -> Result<Vec<IssueComment>> {
        let url = format!("https://api.github.com/repos/{repo}/issues/{pr}/comments?per_page=100");
        self.get::<Vec<IssueComment>>(&url)
            .await?
            .with_context(|| format!("PR {pr} not found in {repo}"))
    }

    /// Creates a new PR comment.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn create_pr_comment(&self, repo: &str, pr: u64, body: &str) -> Result<()> {
        let url = format!("https://api.github.com/repos/{repo}/issues/{pr}/comments");
        let mut payload = HashMap::new();
        payload.insert("body", body);

        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&payload)
            .send()
            .await
            .with_context(|| format!("POST {url} failed"))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST {url} returned {status}: {body}");
        }
        Ok(())
    }

    /// Updates an existing PR comment.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn update_pr_comment(&self, repo: &str, comment_id: u64, body: &str) -> Result<()> {
        let url = format!("https://api.github.com/repos/{repo}/issues/comments/{comment_id}");
        let mut payload = HashMap::new();
        payload.insert("body", body);

        let resp = self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&payload)
            .send()
            .await
            .with_context(|| format!("PATCH {url} failed"))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("PATCH {url} returned {status}: {body}");
        }
        Ok(())
    }
}

/// Parses `owner` and `repo` out of a GitHub URL like
/// `https://github.com/owner/repo` or `https://github.com/owner/repo/`.
/// Returns `None` for non-GitHub URLs or malformed paths.
fn parse_github_repo(url: &str) -> Option<(String, String)> {
    let url = url.trim_end_matches('/');
    let without_scheme = url
        .strip_prefix("https://github.com/")
        .or_else(|| url.strip_prefix("http://github.com/"))?;

    let parts: Vec<&str> = without_scheme.splitn(3, '/').collect();
    if parts.len() < 2 || parts[0].is_empty() || parts[1].is_empty() {
        return None;
    }
    // Reject sub-paths inside a repo (e.g. /tree/main/…).
    if parts.len() == 3 && !parts[2].is_empty() {
        return None;
    }
    Some((parts[0].to_owned(), parts[1].to_owned()))
}

/// Reads and deserialises a single tool YAML file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
fn read_tool(path: &Path) -> Result<ToolEntry> {
    let f = std::fs::File::open(path).with_context(|| format!("Cannot open {}", path.display()))?;
    serde_saphyr::from_reader(f).with_context(|| format!("Cannot parse {}", path.display()))
}

/// Runs all contributing-criteria checks for one tool.
///
/// # Errors
///
/// Returns an error if calendar arithmetic fails. Unavailable metadata is
/// skipped for manual review; verified unmet criteria produce failures.
async fn check_tool(client: &GithubClient, tool: &ToolEntry) -> Result<ToolReport> {
    let source = tool.source.clone();

    let gh_coords = source.as_deref().and_then(parse_github_repo);

    if let Some((owner, repo)) = gh_coords {
        let repo_result = client.repo_info(&owner, &repo).await;
        let contributors_result = client.contributor_count(&owner, &repo).await;

        repository_report(tool, &repo_result, &contributors_result, Utc::now())
    } else {
        let note = "No GitHub source URL found. Automated checks for stars, contributor count, \
                    and age are not possible. Please verify the contributing criteria manually.";

        Ok(ToolReport {
            name: tool.name.to_string(),
            source,
            stars: CheckResult::Skip("N/A".into()),
            contributors: CheckResult::Skip("N/A".into()),
            age: CheckResult::Skip("N/A".into()),
            note: Some(note.into()),
        })
    }
}

/// Evaluates fetched metadata without I/O so policy boundaries are testable.
fn repository_report(
    tool: &ToolEntry,
    repo_result: &Result<Option<RepoInfo>>,
    contributors_result: &Result<Option<usize>>,
    now: DateTime<Utc>,
) -> Result<ToolReport> {
    let stars_check = match repo_result {
        Ok(Some(info)) => {
            let s = info.stargazers_count;
            if s >= MIN_STARS {
                CheckResult::Pass(format!("{s} stars"))
            } else {
                CheckResult::Fail(format!("{s} stars (minimum is {MIN_STARS})"))
            }
        }
        Ok(None) => CheckResult::Skip("repository not found".into()),
        Err(e) => CheckResult::Skip(format!("Could not fetch repo info: {e}")),
    };

    let age_check = match repo_result {
        Ok(Some(info)) => {
            let minimum_created_at = now
                .checked_sub_months(Months::new(MIN_AGE_MONTHS))
                .context("Current date cannot be shifted back by six months")?;
            let days = now.signed_duration_since(info.created_at).num_days();
            if info.created_at <= minimum_created_at {
                CheckResult::Pass(format!("created {days} days ago (at least 6 months)"))
            } else {
                CheckResult::Fail(format!(
                    "created {days} days ago (minimum is 6 calendar months)"
                ))
            }
        }
        Ok(None) => CheckResult::Skip("repository not found".into()),
        Err(_) => CheckResult::Skip("Could not determine age (repo info unavailable)".into()),
    };

    let contributors_check = match contributors_result {
        Ok(Some(count)) => {
            if *count >= MIN_CONTRIBUTORS {
                CheckResult::Pass(format!("{count} human contributors"))
            } else {
                CheckResult::Fail(format!(
                    "{count} human contributor(s) (minimum is {MIN_CONTRIBUTORS})"
                ))
            }
        }
        Ok(None) => CheckResult::Skip("repository not found".into()),
        Err(e) => CheckResult::Skip(format!("Could not fetch contributors: {e}")),
    };

    let repo_not_found = matches!(repo_result, Ok(None));
    let note = repo_not_found.then_some(
        "The source URL returned a 404. Please check that the repository exists and is public.",
    );

    Ok(ToolReport {
        name: tool.name.to_string(),
        source: tool.source.clone(),
        stars: stars_check,
        contributors: contributors_check,
        age: age_check,
        note: note.map(str::to_owned),
    })
}

/// Renders all tool reports into a Markdown comment body.
///
/// # Errors
///
/// Returns an error if the template fails to render.
fn render_comment(reports: &[ToolReport]) -> Result<String> {
    let any_failures = reports.iter().any(|r| r.any_fail());
    CommentTemplate {
        marker: COMMENT_MARKER,
        reports,
        any_failures,
        any_incomplete: reports.iter().any(|r| r.needs_review()),
    }
    .render()
    .context("Failed to render comment template")
}

/// Posts or updates the bot comment on the PR.
///
/// # Errors
///
/// Returns an error if the GitHub API calls fail.
async fn upsert_comment(client: &GithubClient, repo: &str, pr: u64, body: &str) -> Result<()> {
    let comments = client.list_pr_comments(repo, pr).await?;

    let existing = comments.iter().find(|c| c.body.contains(COMMENT_MARKER));

    match existing {
        Some(c) => client.update_pr_comment(repo, c.id, body).await,
        None => client.create_pr_comment(repo, pr, body).await,
    }
}

/// Parses a PR number from a string.
///
/// # Errors
///
/// Returns an error if the string is not a valid integer.
fn parse_pr_number(s: &str) -> Result<u64> {
    s.trim()
        .parse::<u64>()
        .with_context(|| format!("Invalid PR number: {s}"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;
    let gh_repo = env::var("GITHUB_REPOSITORY").context("GITHUB_REPOSITORY not set")?;
    let pr_number_str = env::var("PR_NUMBER").context("PR_NUMBER not set")?;
    let pr_number = parse_pr_number(&pr_number_str)?;

    // Remaining CLI arguments are the paths to check.
    // Usage: pr-check data/tools/foo.yml data/tools/bar.yml
    let pico = pico_args::Arguments::from_env();
    let tool_paths: Vec<PathBuf> = pico.finish().into_iter().map(PathBuf::from).collect();

    let tool_paths: Vec<PathBuf> = tool_paths
        .into_iter()
        .filter(|p| {
            p.starts_with("data/tools")
                && matches!(
                    p.extension().and_then(|e| e.to_str()),
                    Some("yml") | Some("yaml")
                )
        })
        .collect();

    let client = GithubClient::new(token)?;

    let mut reports = Vec::new();
    for path in &tool_paths {
        let tool = read_tool(path).with_context(|| format!("Failed to read {}", path.display()))?;
        eprintln!("Checking '{}'...", tool.name);
        let report = check_tool(&client, &tool).await?;
        reports.push(report);
    }

    let comment_body = render_comment(&reports)?;

    // If COMMENT_OUTPUT_FILE is set, write the comment to that file instead of
    // posting it via the API. This is used by the `pull_request` CI workflow to
    // avoid the 403 that GitHub returns when a fork PR tries to write comments.
    // A separate `pr-comment` workflow picks up the file and posts the comment
    // with the write permissions it has as a `workflow_run` job.
    if let Ok(output_file) = env::var("COMMENT_OUTPUT_FILE") {
        if let Some(parent) = std::path::Path::new(&output_file).parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory for {output_file}"))?;
        }
        std::fs::write(&output_file, &comment_body)
            .with_context(|| format!("Failed to write comment to {output_file}"))?;
        eprintln!("Comment written to {output_file}");
    } else {
        upsert_comment(&client, &gh_repo, pr_number, &comment_body).await?;
    }

    let any_failures = reports.iter().any(|r| r.any_fail());
    if any_failures {
        eprintln!("One or more tools failed the contributing criteria check.");
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_tool() -> ToolEntry {
        ToolEntry {
            name: "Example".into(),
            source: Some("https://github.com/example/tool".into()),
        }
    }

    #[test]
    fn excludes_bots_and_known_automation() {
        for login in [
            "claude",
            "Claude",
            "dependabot",
            "Dependabot",
            "renovate-bot",
            "RENOVATE-BOT",
            "github-actions[bot]",
            "some-new-app[BOT]",
        ] {
            let contributor = Contributor {
                login: login.into(),
                account_type: "User".into(),
            };
            assert!(!contributor.counts_as_human(), "{login}");
        }
        for account_type in ["Bot", "bot", "Organization", "unknown"] {
            let contributor = Contributor {
                login: "alice".into(),
                account_type: account_type.into(),
            };
            assert!(!contributor.counts_as_human(), "{account_type}");
        }
        for login in [
            "alice",
            "claude-smith",
            "dependabot-maintainer",
            "robotics-researcher",
            "human-bot",
        ] {
            for account_type in ["User", "user"] {
                let contributor = Contributor {
                    login: login.into(),
                    account_type: account_type.into(),
                };
                assert!(contributor.counts_as_human(), "{login}");
            }
        }
    }

    #[test]
    fn automation_does_not_satisfy_human_minimum() -> Result<()> {
        let contributors: Vec<Contributor> = serde_saphyr::from_str(
            "- {login: alice, type: User}\n- {login: claude, type: User}\n- {login: 'dependabot[bot]', type: Bot}\n- {login: renovate-bot, type: User}\n- {login: bob, type: User}",
        )?;
        let count =
            |accounts: &[Contributor]| accounts.iter().filter(|c| c.counts_as_human()).count();
        assert_eq!(count(&contributors[..4]), 1);
        assert!(count(&contributors[..4]) < MIN_CONTRIBUTORS);
        assert_eq!(count(&contributors), MIN_CONTRIBUTORS);
        Ok(())
    }

    #[test]
    fn repository_thresholds_and_calendar_age_boundary() -> Result<()> {
        let created_at = "2026-03-01T12:00:00Z".parse::<DateTime<Utc>>()?;
        let boundary = "2026-09-01T12:00:00Z".parse::<DateTime<Utc>>()?;
        for stars in [19, 20, 21] {
            for contributors in [0, 1, 2, 3] {
                for seconds in [-1, 0, 1] {
                    let report = repository_report(
                        &example_tool(),
                        &Ok(Some(RepoInfo {
                            stargazers_count: stars,
                            created_at,
                        })),
                        &Ok(Some(contributors)),
                        boundary + chrono::Duration::seconds(seconds),
                    )?;
                    assert_eq!(report.stars.is_fail(), stars < 20);
                    assert_eq!(report.contributors.is_fail(), contributors < 2);
                    assert_eq!(report.age.is_fail(), seconds < 0);
                    assert_eq!(
                        report.any_fail(),
                        stars < 20 || contributors < 2 || seconds < 0
                    );
                }
            }
        }
        Ok(())
    }

    #[test]
    fn calendar_age_clamps_month_end() -> Result<()> {
        let now = "2026-08-31T12:00:00Z".parse::<DateTime<Utc>>()?;
        for (created, passes) in [
            ("2026-02-28T12:00:00Z", true),
            ("2026-02-28T12:00:01Z", false),
        ] {
            let report = repository_report(
                &example_tool(),
                &Ok(Some(RepoInfo {
                    stargazers_count: 20,
                    created_at: created.parse()?,
                })),
                &Ok(Some(2)),
                now,
            )?;
            assert_eq!(!report.age.is_fail(), passes);
        }
        Ok(())
    }

    #[test]
    fn missing_metadata_requires_review_but_verified_failures_still_fail() -> Result<()> {
        let now = "2026-09-01T12:00:00Z".parse::<DateTime<Utc>>()?;
        let missing = repository_report(&example_tool(), &Ok(None), &Ok(None), now)?;
        assert_eq!(missing.status(), "REVIEW");
        assert!(
            missing
                .note
                .as_deref()
                .is_some_and(|note| note.contains("404"))
        );
        let unavailable = repository_report(
            &example_tool(),
            &Err(anyhow::anyhow!("rate limited")),
            &Err(anyhow::anyhow!("connection failed")),
            now,
        )?;
        assert_eq!(unavailable.status(), "REVIEW");
        let failed = repository_report(&example_tool(), &Ok(None), &Ok(Some(1)), now)?;
        assert_eq!(failed.status(), "FAIL");
        assert!(failed.any_fail());
        Ok(())
    }

    #[test]
    fn report_status_and_summary_require_complete_evidence() -> Result<()> {
        let result = |kind| match kind {
            0 => CheckResult::Pass("verified".into()),
            1 => CheckResult::Fail("below minimum".into()),
            _ => CheckResult::Skip("unavailable".into()),
        };
        for stars in 0..3 {
            for contributors in 0..3 {
                for age in 0..3 {
                    let results = [stars, contributors, age];
                    let fails = results.contains(&1);
                    let incomplete = results.contains(&2);
                    let report = ToolReport {
                        name: "Example".into(),
                        source: None,
                        note: None,
                        stars: result(stars),
                        contributors: result(contributors),
                        age: result(age),
                    };
                    assert_eq!(report.any_fail(), fails);
                    assert_eq!(
                        report.status(),
                        if fails {
                            "FAIL"
                        } else if incomplete {
                            "REVIEW"
                        } else {
                            "PASS"
                        }
                    );
                    let comment = render_comment(&[report])?;
                    assert_eq!(
                        comment.contains("All criteria passed"),
                        !fails && !incomplete
                    );
                    assert_eq!(comment.contains("Manual review required"), incomplete);
                }
            }
        }
        let passed = ToolReport {
            name: "Passed".into(),
            source: None,
            note: None,
            stars: result(0),
            contributors: result(0),
            age: result(0),
        };
        let incomplete = ToolReport {
            name: "Incomplete".into(),
            source: None,
            note: None,
            stars: result(2),
            contributors: result(2),
            age: result(2),
        };
        let comment = render_comment(&[passed, incomplete])?;
        assert!(comment.contains("Manual review required"));
        assert!(!comment.contains("All criteria passed"));
        Ok(())
    }

    #[tokio::test]
    async fn absent_or_non_github_source_requires_review() -> Result<()> {
        let client = GithubClient::new(String::new())?;
        for source in [None, Some("https://gitlab.com/example/tool".into())] {
            let report = check_tool(
                &client,
                &ToolEntry {
                    name: "Example".into(),
                    source,
                },
            )
            .await?;
            assert_eq!(report.status(), "REVIEW");
            assert!(!report.any_fail());
            let comment = render_comment(&[report])?;
            assert!(comment.contains("Manual review required"));
            assert!(!comment.contains("All criteria passed"));
        }
        Ok(())
    }

    #[test]
    fn parses_catalog() -> Result<()> {
        let tools = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/tools");
        let mut count = 0;
        let mut found_bap = false;
        for entry in std::fs::read_dir(tools)? {
            let path = entry?.path();
            if path
                .extension()
                .is_some_and(|ext| ext == "yml" || ext == "yaml")
            {
                let tool = read_tool(&path)?;
                assert!(!tool.name.is_empty(), "{}", path.display());
                found_bap |= tool.name == "BAP";
                count += 1;
            }
        }
        assert!(count > 0);
        assert!(found_bap);
        Ok(())
    }

    #[test]
    fn parses_plain_github_url() {
        let result = parse_github_repo("https://github.com/owner/repo");
        assert_eq!(result, Some(("owner".into(), "repo".into())));
    }

    #[test]
    fn parses_trailing_slash() {
        let result = parse_github_repo("https://github.com/owner/repo/");
        assert_eq!(result, Some(("owner".into(), "repo".into())));
    }

    #[test]
    fn rejects_subpath() {
        let result = parse_github_repo("https://github.com/owner/repo/tree/main/subdir");
        assert!(result.is_none());
    }

    #[test]
    fn rejects_gitlab() {
        let result = parse_github_repo("https://gitlab.com/owner/repo");
        assert!(result.is_none());
    }

    #[test]
    fn rejects_missing_repo() {
        let result = parse_github_repo("https://github.com/owner");
        assert!(result.is_none());
    }

    #[test]
    fn render_comment_no_files() {
        let comment = render_comment(&[]).unwrap();
        assert!(comment.contains("No new tool files detected"));
    }

    #[test]
    fn render_comment_contains_marker() {
        let comment = render_comment(&[]).unwrap();
        assert!(comment.contains(COMMENT_MARKER));
    }
}
