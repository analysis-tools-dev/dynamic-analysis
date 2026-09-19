use anyhow::{Context, Result};
use askama::Template;
use pico_args::Arguments;
use render::types::{Entry, ParsedEntry, Tag, Tags, Type};
use render::{check_deprecated, create_api, create_catalog};
use slug::slugify;
use std::collections::BTreeMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

struct Args {
    tags: PathBuf,
    tools: PathBuf,
    md_out: PathBuf,
    json_out: PathBuf,
    skip_deprecated: bool,
}

fn parse_path(s: &OsStr) -> Result<PathBuf> {
    Ok(s.into())
}

fn read_tags(path: PathBuf) -> Result<Tags> {
    let f = fs::File::open(&path).with_context(|| format!("Cannot open {}", path.display()))?;
    serde_saphyr::from_reader(f).with_context(|| format!("Cannot parse {}", path.display()))
}

fn read_tool(path: &Path, tags: &[Tag]) -> Result<Entry> {
    let file = fs::File::open(path).with_context(|| format!("Cannot open {}", path.display()))?;
    let parsed: ParsedEntry = serde_saphyr::from_reader(file)
        .with_context(|| format!("Cannot parse {}", path.display()))?;
    Entry::from_parsed(parsed, tags).with_context(|| format!("Invalid tool in {}", path.display()))
}

fn read_tools(path: PathBuf, tags: &[Tag]) -> Result<Vec<Entry>> {
    let dir =
        fs::read_dir(&path).with_context(|| format!("Cannot read directory {}", path.display()))?;
    let mut tools = Vec::new();
    for entry in dir {
        let entry =
            entry.with_context(|| format!("Cannot read directory entry in {}", path.display()))?;
        let path = entry.path();
        if matches!(
            path.extension().and_then(OsStr::to_str),
            Some("yml" | "yaml")
        ) {
            println!("Checking {}", path.display());
            tools.push(read_tool(&path, tags)?);
        }
    }
    Ok(tools)
}

/// Backfills the deprecated field in the tools data from the old tools data.
fn backfill_deprecated(tools: &mut Vec<Entry>, path: &Path) -> Result<()> {
    let tools_raw = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Ok(()), // No old data to backfill from. Skip silently.
    };

    let old_tools_data: BTreeMap<String, serde_json::Value> = serde_json::from_str(&tools_raw)
        .with_context(|| format!("Cannot parse {}", path.display()))?;

    for tool in tools {
        let id = slugify(&tool.name);
        if let Some(old_tool) = old_tools_data.get(&id) {
            // Only backfill deprecated if it's not already set
            if tool.deprecated.is_none() {
                tool.deprecated = old_tool.get("deprecated").and_then(|d| d.as_bool());
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    let args = Args {
        tags: args.value_from_os_str("--tags", parse_path)?,
        tools: args.value_from_os_str("--tools", parse_path)?,
        md_out: args.value_from_os_str("--md-out", parse_path)?,
        json_out: args.value_from_os_str("--json-out", parse_path)?,
        skip_deprecated: args.contains("--skip-deprecated"),
    };

    let tags = read_tags(args.tags)?;

    let mut tools = read_tools(args.tools, &tags)?;
    tools.sort();

    let should_check_deprecation = !args.skip_deprecated;
    let github_token = env::var("GITHUB_TOKEN");
    let old_tools_path = args.json_out.join("tools.json");

    match (should_check_deprecation, github_token) {
        (true, Ok(token)) => check_deprecated(token, &mut tools)?,
        (true, Err(_)) => {
            eprintln!("No GITHUB_TOKEN environment variable found. Reusing old deprecation data.");
            backfill_deprecated(&mut tools, &old_tools_path)?;
        }
        (false, _) => backfill_deprecated(&mut tools, &old_tools_path)?,
    }

    let languages: Vec<Tag> = tags
        .clone()
        .into_iter()
        .filter(|t| t.tag_type == Type::Language)
        .collect();

    let other_tags: Vec<Tag> = tags
        .clone()
        .into_iter()
        .filter(|t| t.tag_type == Type::Other)
        .collect();

    let catalog = create_catalog(&tools, &languages, &other_tags)?;
    fs::write(&args.md_out, catalog.render()?).context(format!(
        "Cannot write Markdown output to {}",
        args.md_out.display()
    ))?;

    let api = create_api(catalog, &languages, &other_tags)?;

    let json = serde_json::to_string_pretty(&api)?;
    let tools_out = args.json_out.join("tools.json");
    fs::write(&tools_out, json).context(format!(
        "Cannot write tools JSON output to {}",
        args.json_out.display()
    ))?;

    let mut tags_json = BTreeMap::new();
    tags_json.insert("languages", languages);
    tags_json.insert("other", other_tags);
    let json = serde_json::to_string_pretty(&tags_json)?;

    let tags_out = args.json_out.join("tags.json");
    fs::write(&tags_out, json).context(format!(
        "Cannot write tags JSON output to {}",
        args.json_out.display()
    ))?;

    // let stats_raw = fs::read_to_string("data/api/stats_raw.json")?;
    // let stats: StatsRaw = serde_json::from_str(&stats_raw)?;

    // let stats = format_stats(stats);
    // let json = serde_json::to_string(&stats)?;

    // let stats_out = args.json_out.join("stats.json");
    // fs::write(&stats_out, json).context(format!(
    //     "Cannot write stats JSON output to {}",
    //     args.json_out.display()
    // ))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use render::types::{Category, ToolType};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    fn tags() -> Tags {
        vec![Tag {
            name: "Rust".into(),
            value: "rust".into(),
            tag_type: Type::Language,
        }]
    }

    #[test]
    fn reads_both_yaml_extensions_and_preserves_dynamic_categories() -> Result<()> {
        let tags = tags();
        let tools = read_tools(fixture("tools"), &tags)?;
        assert_eq!(tools.len(), 2);
        let catalog = create_catalog(&tools, &tags, &[])?;
        let markdown = catalog.render()?;
        assert!(markdown.contains("Example Fuzzer"));
        assert!(markdown.contains("Example Profiler"));
        let api = create_api(catalog, &tags, &[])?;
        assert!(api["example-fuzzer"].categories.contains(&Category::Fuzzer));
        assert!(
            api["example-profiler"]
                .categories
                .contains(&Category::Profiler)
        );
        assert!(api["example-profiler"].types.contains(&ToolType::Gui));
        let json = serde_json::to_value(api)?;
        assert_eq!(
            json["example-fuzzer"]["categories"],
            serde_json::json!(["fuzzer"])
        );
        assert_eq!(
            json["example-profiler"]["categories"],
            serde_json::json!(["profiler"])
        );
        Ok(())
    }

    #[test]
    fn malformed_yaml_reports_actual_path() {
        let path = fixture("malformed.yaml");
        let error = read_tool(&path, &tags()).unwrap_err();
        assert!(
            error
                .to_string()
                .contains(&format!("Cannot parse {}", path.display()))
        );
        let error = read_tags(path.clone()).unwrap_err();
        assert!(
            error
                .to_string()
                .contains(&format!("Cannot parse {}", path.display()))
        );
    }

    #[test]
    fn invalid_tags_report_actual_path_and_all_errors() {
        let path = fixture("unexpected-filename.yaml");
        let error = format!("{:#}", read_tool(&path, &tags()).unwrap_err());
        assert!(error.contains(&path.display().to_string()));
        assert!(error.contains("Different Tool Name"));
        assert!(error.contains("Invalid tag: unknown-tag"));
        assert!(error.contains("Invalid tag: another-unknown-tag"));
        assert!(!error.contains("different-tool-name.yml"));
    }

    #[test]
    fn missing_inputs_report_actual_path() {
        let path = fixture("missing.yaml");
        assert!(
            read_tool(&path, &tags())
                .unwrap_err()
                .to_string()
                .contains(&path.display().to_string())
        );
        assert!(
            read_tags(path.clone())
                .unwrap_err()
                .to_string()
                .contains(&path.display().to_string())
        );
        assert!(
            read_tools(path.clone(), &tags())
                .unwrap_err()
                .to_string()
                .contains(&path.display().to_string())
        );
    }

    #[test]
    fn backfills_from_selected_output_without_overriding_explicit_flags() -> Result<()> {
        let mut tools = read_tools(fixture("tools"), &tags())?;
        for tool in &mut tools {
            if tool.name == "Example Profiler" {
                tool.deprecated = Some(false);
            }
        }
        backfill_deprecated(&mut tools, &fixture("tools.json"))?;
        for tool in &tools {
            assert_eq!(tool.deprecated, Some(tool.name == "Example Fuzzer"));
        }
        let previous = tools.clone();
        backfill_deprecated(&mut tools, &fixture("missing.json"))?;
        assert_eq!(tools, previous);
        Ok(())
    }

    #[test]
    fn parses_catalog() -> Result<()> {
        let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
        let tags = read_tags(data.join("tags.yml"))?;
        let tools = read_tools(data.join("tools"), &tags)?;
        assert!(!tags.is_empty());
        assert!(!tools.is_empty());
        assert!(tools.iter().any(|tool| tool.name == "BAP"));
        Ok(())
    }
}
