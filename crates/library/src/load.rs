//! Writes a parsed [`Library`] into the course database.
//!
//! Mirrors `rustclass_core::apply_outline`: one transaction, and Replace
//! soft-deletes every live chapter first (so it stays recoverable from the
//! trash) and drops the chapter rows from the search index the same way.

use crate::format::Library;
use rusqlite::Connection;
use rustclass_core::models::{CoursePatch, NewChapter};
use rustclass_core::repo;
use rustclass_core::{time, CoreError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoadMode {
    /// Soft-delete every existing chapter, then add the library.
    Replace,
    /// Add the library after whatever already exists.
    Append,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadReport {
    pub chapters_created: u32,
    pub blocks_created: u32,
    pub chapters_removed: u32,
    /// How many library files were loaded.
    pub files: u32,
    /// `file:line: message`, one per library warning.
    pub warnings: Vec<String>,
}

pub fn load(conn: &mut Connection, lib: &Library, mode: LoadMode) -> Result<LoadReport> {
    // An empty built-in folder must never turn "Replace" into "delete my course".
    if mode == LoadMode::Replace && lib.chapters.is_empty() {
        return Err(CoreError::invalid(
            "The library has no chapters, so replacing the course with it would only delete it.",
        ));
    }

    let course = repo::course::get_or_seed(conn)?;
    let mut report = LoadReport {
        chapters_created: 0,
        blocks_created: 0,
        chapters_removed: 0,
        files: lib.files.len() as u32,
        warnings: lib.warnings.iter().map(ToString::to_string).collect(),
    };

    let tx = conn.transaction()?;

    if mode == LoadMode::Replace {
        let now = time::now_ms();
        report.chapters_removed = tx.execute(
            "UPDATE chapter SET deleted_at = ?1, updated_at = ?1 WHERE deleted_at IS NULL",
            [now],
        )? as u32;
        tx.execute("DELETE FROM search_idx WHERE entity_type = 'chapter'", [])?;
    }

    if let Some(info) = &lib.course {
        repo::course::update(
            &tx,
            CoursePatch {
                title: info.title.clone(),
                subtitle: Some(info.subtitle.clone()),
                author: Some(info.author.clone()),
                description_md: Some(info.description_md.clone()),
                cover_asset_id: None,
            },
        )?;
    }

    // Each chapter goes right after the previous one, so file order is the
    // course order no matter what was there before (Append) or how many there are.
    let mut after: Option<String> = None;
    for chapter in &lib.chapters {
        let req = NewChapter {
            title: chapter.title.clone(),
            display_number: chapter.display_number.clone(),
            summary_md: chapter.summary_md.clone(),
            objectives_md: chapter.objectives_md.clone(),
            estimated_minutes: chapter.estimated_minutes,
            template: None,
            after_id: after.clone(),
            part: chapter.part.clone(),
            section: chapter.section.clone(),
        };
        let chapter_id = repo::chapter::insert_in_tx(&tx, &course.id, &req)?;
        report.chapters_created += 1;

        let mut prev_block: Option<String> = None;
        for block in &chapter.blocks {
            let block_id = repo::block::insert_in_tx(
                &tx,
                &chapter_id,
                block.kind,
                prev_block.as_deref(),
                Some(block.seed()),
            )?;
            report.blocks_created += 1;
            prev_block = Some(block_id);
        }
        after = Some(chapter_id);
    }

    tx.commit()?;
    Ok(report)
}
