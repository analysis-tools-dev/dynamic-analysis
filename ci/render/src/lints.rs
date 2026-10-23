use anyhow::{Result, anyhow};

use crate::types::ParsedEntry;
use crate::types::Tag;

pub fn name(entry: &ParsedEntry, _: &[Tag]) -> Result<()> {
    if entry.name.trim().is_empty() {
        Err(anyhow!("Name of entry must not be blank"))
    } else if entry.name.len() <= 50 {
        Ok(())
    } else {
        Err(anyhow!(
            "Name of entry may be at most 50 UTF-8 bytes long, but {} is {} bytes long",
            entry.name,
            entry.name.len()
        ))
    }
}

pub fn min_one_tag(entry: &ParsedEntry, _: &[Tag]) -> Result<()> {
    if entry.tags.is_empty() {
        Err(anyhow!(
            "{} must have at least one tag from `tags.yml`.",
            entry.name
        ))
    } else {
        Ok(())
    }
}
