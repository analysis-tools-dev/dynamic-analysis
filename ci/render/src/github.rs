use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::time::Duration;

use crate::types::Entry;

#[derive(Deserialize)]
struct Commit {
    commit: CommitDetails,
}

#[derive(Deserialize)]
struct CommitDetails {
    author: Author,
}

#[derive(Deserialize)]
struct Author {
    date: DateTime<Utc>,
}

fn client() -> Result<Client> {
    Client::builder()
        .user_agent("analysis-tools-bot")
        .timeout(Duration::from_secs(30))
        .build()
        .context("Cannot build GitHub HTTP client")
}

async fn last_commit(client: &Client, token: &str, url: &str) -> Result<Option<DateTime<Utc>>> {
    let response = client
        .get(url)
        .bearer_auth(token)
        .query(&[("per_page", "1")])
        .send()
        .await
        .context("Cannot request GitHub commits")?;
    match response.status() {
        // Missing/private repositories and empty repositories provide no activity evidence.
        StatusCode::NOT_FOUND | StatusCode::CONFLICT => return Ok(None),
        _ => {}
    }
    let commits: Vec<Commit> = response
        .error_for_status()
        .context("GitHub commits request failed")?
        .json()
        .await
        .context("Cannot parse GitHub commits")?;
    Ok(commits.first().map(|commit| commit.commit.author.date))
}

pub async fn check_deprecated(token: String, entries: &mut Vec<Entry>) -> Result<()> {
    println!("Checking for deprecated entries on Github. This might take a while...");
    let client = client()?;
    for entry in entries {
        let Some(source) = entry.source.as_ref() else {
            continue;
        };
        let components: Vec<&str> = source.trim_end_matches('/').split('/').collect();
        if !(components.contains(&"github.com") && components.len() == 5) {
            continue;
        }
        let owner = components[3];
        let repo = components[4];
        let url = format!("https://api.github.com/repos/{owner}/{repo}/commits");
        if let Some(date) = last_commit(&client, &token, &url)
            .await
            .with_context(|| format!("Cannot check deprecation for {owner}/{repo}"))?
        {
            let duration = Local::now()
                .date_naive()
                .signed_duration_since(date.date_naive());
            entry.deprecated = (duration.num_days() > 365).then_some(true);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    async fn request(status: &str, body: &str) -> Result<Option<DateTime<Utc>>> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let url = format!("http://{}/commits", listener.local_addr()?);
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let server = std::thread::spawn(move || -> std::io::Result<()> {
            let (mut stream, _) = listener.accept()?;
            stream.set_read_timeout(Some(Duration::from_secs(5)))?;
            let mut headers = Vec::new();
            while !headers.ends_with(b"\r\n\r\n") {
                let mut byte = [0];
                stream.read_exact(&mut byte)?;
                headers.push(byte[0]);
            }
            let headers = String::from_utf8_lossy(&headers);
            assert!(headers.starts_with("GET /commits?per_page=1 "));
            assert!(headers.contains("authorization: Bearer test-token"));
            stream.write_all(response.as_bytes())
        });
        let result = last_commit(
            &Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(5))
                .build()?,
            "test-token",
            &url,
        )
        .await;
        server
            .join()
            .map_err(|_| anyhow::anyhow!("Mock server panicked"))??;
        result
    }

    #[tokio::test]
    async fn parses_rfc3339_timestamps() -> Result<()> {
        let date = request(
            "200 OK",
            r#"[{"commit":{"author":{"date":"2025-01-02T03:04:05+02:00"}}}]"#,
        )
        .await?;
        assert_eq!(date, Some("2025-01-02T01:04:05Z".parse()?));
        Ok(())
    }

    #[tokio::test]
    async fn missing_empty_and_conflicting_repositories_provide_no_evidence() -> Result<()> {
        for (status, body) in [
            ("404 Not Found", "{}"),
            ("409 Conflict", "{}"),
            ("200 OK", "[]"),
        ] {
            assert_eq!(request(status, body).await?, None);
        }
        Ok(())
    }

    #[tokio::test]
    async fn reports_api_and_deserialization_failures() {
        for (status, body) in [
            ("401 Unauthorized", "{}"),
            ("403 Forbidden", "{}"),
            ("429 Too Many Requests", "{}"),
            ("500 Internal Server Error", "{}"),
            ("200 OK", "not json"),
            ("200 OK", r#"[{"commit":{"author":{"date":"invalid"}}}]"#),
        ] {
            assert!(request(status, body).await.is_err());
        }
    }
}
