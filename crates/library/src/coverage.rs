//! Compares a parsed library with `library/CURRICULUM.md`, the single source
//! of truth for parts, sections, display numbers and titles.
//!
//! The curriculum is read, not trusted: a table row is only counted when it
//! sits under a `@@part` line and either a `### Section N — `name`` heading
//! or a `section` column (part 7 has one table and no section headings).

use crate::format::{LibChapter, Library};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// One curriculum row: a chapter the library is expected to contain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expected {
    pub display: String,
    pub part: String,
    pub section: String,
    pub title: String,
    /// `Video`, `Text`, `PDF`, `Quiz` or `Own`. Own rows are expected too.
    pub kind: String,
    /// The video length (`5:07`) from the `video` column; `None` when the
    /// table has no such column or the cell is a dash.
    pub video: Option<String>,
}

struct Columns {
    display: usize,
    title: usize,
    kind: usize,
    video: Option<usize>,
    section: Option<usize>,
}

impl Columns {
    /// `None` when the row is not the header of a curriculum table.
    fn from_header(header: &[String]) -> Option<Self> {
        let find = |name: &str| header.iter().position(|h| h.eq_ignore_ascii_case(name));
        Some(Self {
            display: find("display")?,
            title: find("title")?,
            kind: find("type")?,
            video: find("video"),
            section: header
                .iter()
                .position(|h| h.to_ascii_lowercase().starts_with("section")),
        })
    }
}

fn cells(line: &str) -> Vec<String> {
    let line = line.trim();
    let inner = line.strip_prefix('|').unwrap_or(line);
    let inner = inner.strip_suffix('|').unwrap_or(inner);
    inner.split('|').map(|c| c.trim().to_string()).collect()
}

fn is_separator(cells: &[String]) -> bool {
    cells
        .iter()
        .all(|c| !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':'))
}

/// The text between the first pair of backticks on a line.
fn backticked(line: &str) -> Option<&str> {
    let rest = &line[line.find('`')? + 1..];
    Some(&rest[..rest.find('`')?])
}

/// Reads every table row that has a `display` and a `title` column.
pub fn parse_curriculum(md: &str) -> Result<Vec<Expected>, String> {
    let mut rows = Vec::new();
    let mut part: Option<String> = None;
    let mut section: Option<String> = None;
    let mut columns: Option<Columns> = None;

    for (i, line) in md.lines().enumerate() {
        let n = i + 1;
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("`@@part ") {
            part = rest.split('`').next().map(|p| p.trim().to_string());
            section = None;
            columns = None;
        } else if t.starts_with("### Section") {
            section = backticked(t).map(|s| s.trim().to_string());
            columns = None;
        } else if t.starts_with('|') {
            let row = cells(t);
            if is_separator(&row) {
                continue;
            }
            let Some(c) = &columns else {
                columns = Columns::from_header(&row);
                continue;
            };
            let get = |idx: usize| -> Result<String, String> {
                row.get(idx)
                    .cloned()
                    .ok_or_else(|| format!("CURRICULUM.md line {n}: the table row is too short"))
            };
            let row_section = match c.section {
                Some(idx) => get(idx)?,
                None => section.clone().ok_or_else(|| {
                    format!(
                        "CURRICULUM.md line {n}: a table row has no `### Section` heading above it"
                    )
                })?,
            };
            rows.push(Expected {
                display: get(c.display)?,
                part: part.clone().ok_or_else(|| {
                    format!("CURRICULUM.md line {n}: a table row has no `@@part` line above it")
                })?,
                section: row_section,
                title: get(c.title)?,
                kind: get(c.kind)?,
                video: match c.video {
                    Some(idx) => Some(get(idx)?).filter(|v| v.contains(':')),
                    None => None,
                },
            });
        } else {
            columns = None;
        }
    }

    if rows.is_empty() {
        return Err("CURRICULUM.md has no table rows with display and title columns".into());
    }
    Ok(rows)
}

/// A chapter in the content that the curriculum does not account for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unexpected {
    pub display: Option<String>,
    pub title: String,
    pub file: String,
    pub line: usize,
    pub reason: &'static str,
}

/// One field that differs between the curriculum and the content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    /// `title`, `section` or `part`.
    pub field: &'static str,
    pub expected: String,
    /// `(none)` when the content has no value.
    pub found: String,
}

/// Same display number, but not the same chapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch {
    pub display: String,
    pub file: String,
    pub line: usize,
    pub differences: Vec<Difference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartCoverage {
    pub part: String,
    pub expected: usize,
    pub present: usize,
    pub mismatch: usize,
    pub missing: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coverage {
    pub expected: usize,
    /// Display numbers whose title, section and part all match.
    pub present: Vec<String>,
    /// Expected but absent.
    pub missing: Vec<Expected>,
    /// In the content, not in the curriculum.
    pub unexpected: Vec<Unexpected>,
    pub mismatch: Vec<Mismatch>,
    pub by_part: Vec<PartCoverage>,
}

fn differences(want: &Expected, got: &LibChapter) -> Vec<Difference> {
    let none = || "(none)".to_string();
    let mut out = Vec::new();
    if want.title != got.title {
        out.push(Difference {
            field: "title",
            expected: want.title.clone(),
            found: got.title.clone(),
        });
    }
    if got.section.as_deref() != Some(want.section.as_str()) {
        out.push(Difference {
            field: "section",
            expected: want.section.clone(),
            found: got.section.clone().unwrap_or_else(none),
        });
    }
    if got.part.as_deref() != Some(want.part.as_str()) {
        out.push(Difference {
            field: "part",
            expected: want.part.clone(),
            found: got.part.clone().unwrap_or_else(none),
        });
    }
    out
}

pub fn compare(expected: &[Expected], lib: &Library) -> Coverage {
    let known: HashSet<&str> = expected.iter().map(|e| e.display.as_str()).collect();
    let mut found: HashMap<&str, &LibChapter> = HashMap::new();
    let mut unexpected = Vec::new();
    for chapter in &lib.chapters {
        let mut reject = |reason: &'static str| {
            unexpected.push(Unexpected {
                display: chapter.display_number.clone(),
                title: chapter.title.clone(),
                file: chapter.file.clone(),
                line: chapter.line,
                reason,
            });
        };
        match chapter.display_number.as_deref() {
            None => reject("has no display number"),
            Some(d) if !known.contains(d) => reject("is not in the curriculum"),
            Some(d) if found.contains_key(d) => reject("repeats an earlier display number"),
            Some(d) => {
                found.insert(d, chapter);
            }
        }
    }

    let mut cov = Coverage {
        expected: expected.len(),
        present: Vec::new(),
        missing: Vec::new(),
        unexpected,
        mismatch: Vec::new(),
        by_part: Vec::new(),
    };
    for want in expected {
        if !cov.by_part.iter().any(|p| p.part == want.part) {
            cov.by_part.push(PartCoverage {
                part: want.part.clone(),
                expected: 0,
                present: 0,
                mismatch: 0,
                missing: 0,
            });
        }
        let Some(part) = cov.by_part.iter_mut().find(|p| p.part == want.part) else {
            continue;
        };
        part.expected += 1;
        match found.get(want.display.as_str()) {
            None => {
                part.missing += 1;
                cov.missing.push(want.clone());
            }
            Some(got) => {
                let diff = differences(want, got);
                if diff.is_empty() {
                    part.present += 1;
                    cov.present.push(want.display.clone());
                } else {
                    part.mismatch += 1;
                    cov.mismatch.push(Mismatch {
                        display: want.display.clone(),
                        file: got.file.clone(),
                        line: got.line,
                        differences: diff,
                    });
                }
            }
        }
    }
    cov
}

/// Is `b` the item right after `a` in the same section (`1.2.4` -> `1.2.5`)?
fn follows(a: &str, b: &str) -> bool {
    match (a.rsplit_once('.'), b.rsplit_once('.')) {
        (Some((pa, na)), Some((pb, nb))) => {
            pa == pb
                && matches!((na.parse::<u32>(), nb.parse::<u32>()), (Ok(x), Ok(y)) if y == x + 1)
        }
        _ => false,
    }
}

/// `1.2.1 1.2.2 1.2.3 1.2.5` -> `1.2.1-1.2.3 1.2.5`.
fn compress(displays: &[&str]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < displays.len() {
        let mut j = i;
        while j + 1 < displays.len() && follows(displays[j], displays[j + 1]) {
            j += 1;
        }
        if j >= i + 2 {
            out.push(format!("{}-{}", displays[i], displays[j]));
        } else {
            out.extend(displays[i..=j].iter().map(ToString::to_string));
        }
        i = j + 1;
    }
    out.join(" ")
}

impl fmt::Display for Coverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} present, {} missing, {} mismatch, {} unexpected (of {} expected)",
            self.present.len(),
            self.missing.len(),
            self.mismatch.len(),
            self.unexpected.len(),
            self.expected
        )?;

        writeln!(f, "\nBy part")?;
        for p in &self.by_part {
            writeln!(
                f,
                "  {:<44} {:>3} of {:<3} present, {} missing, {} mismatch",
                p.part, p.present, p.expected, p.missing, p.mismatch
            )?;
        }

        if !self.mismatch.is_empty() {
            writeln!(f, "\nMismatch ({})", self.mismatch.len())?;
            for m in &self.mismatch {
                writeln!(f, "  {} ({}:{})", m.display, m.file, m.line)?;
                for d in &m.differences {
                    writeln!(f, "    {:<8} expected: {}", d.field, d.expected)?;
                    writeln!(f, "    {:<8} found:    {}", "", d.found)?;
                }
            }
        }

        if !self.unexpected.is_empty() {
            writeln!(f, "\nUnexpected ({})", self.unexpected.len())?;
            for u in &self.unexpected {
                writeln!(
                    f,
                    "  {} `{}` {} ({}:{})",
                    u.display.as_deref().unwrap_or("(no number)"),
                    u.title,
                    u.reason,
                    u.file,
                    u.line
                )?;
            }
        }

        if !self.missing.is_empty() {
            writeln!(f, "\nMissing ({})", self.missing.len())?;
            let mut i = 0;
            while i < self.missing.len() {
                let key = (&self.missing[i].part, &self.missing[i].section);
                let mut j = i;
                while j + 1 < self.missing.len()
                    && (&self.missing[j + 1].part, &self.missing[j + 1].section) == key
                {
                    j += 1;
                }
                let displays: Vec<&str> = self.missing[i..=j]
                    .iter()
                    .map(|e| e.display.as_str())
                    .collect();
                writeln!(
                    f,
                    "  {} / {} ({}): {}",
                    key.0,
                    key.1,
                    displays.len(),
                    compress(&displays)
                )?;
                i = j + 1;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_of_consecutive_items_collapse() {
        let d = ["1.2.1", "1.2.2", "1.2.3", "1.2.5", "1.2.6", "1.3.1"];
        assert_eq!(compress(&d), "1.2.1-1.2.3 1.2.5 1.2.6 1.3.1");
        assert_eq!(compress(&[]), "");
    }

    #[test]
    fn backticked_takes_the_first_pair() {
        assert_eq!(
            backticked("### Section 1 — `Teacher's Orientation` (our additions)"),
            Some("Teacher's Orientation")
        );
        assert_eq!(backticked("no ticks"), None);
    }
}
