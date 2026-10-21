use anyhow::Result;
use askama::Template;
use render::types::{Catalog, Entry, Tag, Type};
use std::collections::BTreeMap;

fn tag(name: &str, value: &str, tag_type: Type) -> Tag {
    Tag {
        name: name.into(),
        value: value.into(),
        tag_type,
    }
}

fn tool(name: &str, deprecated: Option<bool>) -> Result<Entry> {
    Ok(serde_json::from_value(serde_json::json!({
        "name": name,
        "categories": ["fuzzer"],
        "tags": [],
        "license": "proprietary",
        "types": ["cli"],
        "homepage": format!("https://example.com/{name}"),
        "description": "Finds runtime errors.",
        "discussion": "https://example.com/discussion",
        "deprecated": deprecated
    }))?)
}

fn catalog(entries: Vec<Entry>, section: &str) -> Catalog {
    let mut catalog = Catalog {
        linters: BTreeMap::new(),
        others: BTreeMap::new(),
        multi: vec![],
    };
    match section {
        "language" => {
            catalog
                .linters
                .insert(tag("C++", "cpp", Type::Language), entries);
        }
        "other" => {
            catalog
                .others
                .insert(tag("Runtime", "runtime", Type::Other), entries);
        }
        _ => catalog.multi = entries,
    }
    catalog
}

#[test]
fn empty_catalog_has_no_deprecated_disclosure_or_tools() -> Result<()> {
    let markdown = catalog(vec![], "multi").render()?;
    assert!(!markdown.contains("Show Deprecated"));
    assert!(!markdown.contains("- ["));
    assert!(markdown.contains("Edit files in `data/tools/`"));
    assert!(!markdown.contains("linters and formatters"));
    for workflow in ["ci", "links"] {
        assert!(markdown.contains(&format!(
            "](https://github.com/analysis-tools-dev/dynamic-analysis/actions/workflows/{workflow}.yml)"
        )));
    }
    Ok(())
}

#[test]
fn sections_group_only_true_deprecation_and_preserve_links_and_symbols() -> Result<()> {
    for section in ["language", "multi", "other"] {
        for flags in [
            vec![None, Some(false)],
            vec![Some(true), None, Some(false), Some(true)],
            vec![Some(true), Some(true)],
        ] {
            let entries = flags
                .iter()
                .enumerate()
                .map(|(i, flag)| tool(&format!("Tool{i}"), *flag))
                .collect::<Result<Vec<_>>>()?;
            let markdown = catalog(entries, section).render()?;
            let disclosure = "<details>\n<summary>Show Deprecated</summary>\n\n";
            assert_eq!(
                markdown.matches(disclosure).count(),
                usize::from(flags.contains(&Some(true)))
            );
            let (active, deprecated) = markdown.split_once(disclosure).unwrap_or((&markdown, ""));
            for (i, flag) in flags.iter().enumerate() {
                let warning = if *flag == Some(true) {
                    " :warning:"
                } else {
                    ""
                };
                let line = format!(
                    "- [Tool{i}](https://example.com/Tool{i}) [:information_source:](<https://example.com/discussion>){warning} :copyright: — Finds runtime errors."
                );
                assert_eq!(markdown.matches(&line).count(), 1);
                if *flag == Some(true) {
                    assert!(!active.contains(&line));
                    assert!(
                        deprecated
                            .split_once("</details>")
                            .is_some_and(|(body, _)| body.contains(&line))
                    );
                } else {
                    assert!(active.contains(&line));
                }
            }
            if flags.contains(&Some(true)) {
                assert!(deprecated.contains("\n\n</details>"));
            }
        }
    }
    Ok(())
}

#[test]
fn open_source_tools_without_discussions_have_no_symbols() -> Result<()> {
    let mut entry = tool("Open", Some(false))?;
    entry.license = "MIT".into();
    entry.discussion = None;
    assert!(!entry.is_deprecated());
    let markdown = catalog(vec![entry], "multi").render()?;
    let line = markdown.lines().find(|line| line.starts_with("- [Open]"));
    assert_eq!(
        line,
        Some("- [Open](https://example.com/Open) — Finds runtime errors.")
    );
    Ok(())
}

#[test]
fn navigation_has_three_columns_and_matching_section_anchors() -> Result<()> {
    for count in 0..=7 {
        let mut catalog = catalog(vec![], "multi");
        for i in 0..count {
            catalog.linters.insert(
                tag(
                    &format!("Language {i}"),
                    &format!("lang-{i}"),
                    Type::Language,
                ),
                vec![],
            );
            catalog.others.insert(
                tag(&format!("Other {i}"), &format!("topic-{i}"), Type::Other),
                vec![],
            );
        }
        let markdown = catalog.render()?;
        let (navigation, body) = markdown
            .split_once("\n---\n")
            .ok_or_else(|| anyhow::anyhow!("Missing navigation separator"))?;
        for prefix in ["lang", "topic"] {
            let rows: Vec<_> = navigation
                .lines()
                .filter(|line| line.starts_with('|') && line.contains(&format!("](#{prefix}-")))
                .collect();
            assert_eq!(rows.len(), (count as usize).div_ceil(3));
            for (i, row) in rows.iter().enumerate() {
                assert_eq!(row.matches("](#").count(), (count as usize - i * 3).min(3));
            }
            for i in 0..count {
                assert_eq!(navigation.matches(&format!("](#{prefix}-{i})")).count(), 1);
                assert!(body.contains(&format!("<a id=\"{prefix}-{i}\"></a>")));
            }
        }
        for anchor in ["programming-languages-1", "multiple-languages-1", "other-1"] {
            assert!(navigation.contains(&format!("](#{anchor})")));
        }
    }
    Ok(())
}
