//! Parser for library content files.
//!
//! `library/FORMAT.md` is the contract and this module implements it, no more
//! and no less. A file is a list of `@@` directives, each followed by its
//! body; nothing is parsed inside bodies except quiz questions, so code
//! fences, tables and indentation reach the database byte for byte.
//!
//! Every hard error names the file and the 1-based line. Hard errors are
//! collected across all files and refuse the whole load; warnings ride along
//! in [`Library::warnings`].

use rustclass_core::models::{BlockKind, BlockSeed};
use rustclass_core::CoreError;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

/// Callout tones the frontend knows (`CalloutTone` in `src/lib/types.ts`).
pub const CALLOUT_TONES: [&str; 4] = ["note", "tip", "warning", "danger"];
/// Languages a `@@code` block may declare.
pub const CODE_LANGS: [&str; 6] = ["rust", "toml", "bash", "json", "text", "mermaid"];

/// A hard error. Rendered as `file:line: message`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibError {
    pub file: String,
    pub line: usize,
    pub message: String,
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.file, self.line, self.message)
    }
}

impl std::error::Error for LibError {}

/// A problem that does not stop the load. Rendered as `file:line: message`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibWarning {
    pub file: String,
    pub line: usize,
    pub message: String,
}

impl fmt::Display for LibWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.file, self.line, self.message)
    }
}

/// What `@@course` sets. Absent fields mean "leave the course row alone" for
/// the title and "clear it" for the rest, see `load`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CourseInfo {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub description_md: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibBlock {
    /// Line of the directive.
    pub line: usize,
    /// Line of the first line of the body, or `line + 1` when there is none.
    /// The snippet checker uses it to report rustc errors at file lines.
    pub body_line: usize,
    pub kind: BlockKind,
    pub heading: Option<String>,
    pub caption_md: Option<String>,
    pub body_md: Option<String>,
    pub code_text: Option<String>,
    pub code_lang: Option<String>,
    pub meta_json: Option<String>,
}

impl LibBlock {
    /// The seed `rustclass_core::repo::block::insert_in_tx` wants.
    pub fn seed(&self) -> BlockSeed {
        BlockSeed {
            heading: self.heading.clone(),
            caption_md: self.caption_md.clone(),
            body_md: self.body_md.clone(),
            code_text: self.code_text.clone(),
            code_lang: self.code_lang.clone(),
            meta_json: self.meta_json.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibChapter {
    pub file: String,
    /// Line of the `@@chapter` directive.
    pub line: usize,
    pub part: Option<String>,
    pub section: Option<String>,
    pub display_number: Option<String>,
    pub title: String,
    pub summary_md: Option<String>,
    pub objectives_md: Option<String>,
    pub estimated_minutes: Option<i64>,
    pub blocks: Vec<LibBlock>,
}

/// A parsed, validated library: chapters in file order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Library {
    pub course: Option<CourseInfo>,
    /// File names, in the order they were parsed.
    pub files: Vec<String>,
    pub chapters: Vec<LibChapter>,
    pub warnings: Vec<LibWarning>,
}

impl Library {
    pub fn block_count(&self) -> usize {
        self.chapters.iter().map(|c| c.blocks.len()).sum()
    }
}

/// Folds parse errors into the one error type the Tauri layer already knows.
/// Long lists are cut so the message still fits a toast.
pub fn errors_to_core(errors: &[LibError]) -> CoreError {
    const SHOWN: usize = 12;
    let n = errors.len();
    let mut msg = format!(
        "The library has {n} error{}:",
        if n == 1 { "" } else { "s" }
    );
    for e in errors.iter().take(SHOWN) {
        msg.push('\n');
        msg.push_str(&e.to_string());
    }
    if n > SHOWN {
        msg.push_str(&format!("\n... and {} more", n - SHOWN));
    }
    CoreError::invalid(msg)
}

/// Parses `(file name, content)` pairs, in the order given. Choosing which
/// files to pass, and in what order, is the caller's job (`read_dir` does the
/// lexical sort and the `_` rule).
pub fn parse_files(files: &[(String, String)]) -> Result<Library, Vec<LibError>> {
    let mut parser = Parser::default();
    for (name, content) in files {
        parser.parse_file(name, content);
    }
    if parser.errors.is_empty() {
        Ok(parser.lib)
    } else {
        Err(parser.errors)
    }
}

// ---------------------------------------------------------------------------
// Lines -> directives
// ---------------------------------------------------------------------------

/// One `@@` line and the lines below it.
struct Raw<'a> {
    line: usize,
    name: &'a str,
    args: &'a str,
    body: Vec<(usize, &'a str)>,
}

/// Splits a file into directives. Returns the line of the first stray
/// non-blank line that precedes every directive, if there is one.
fn split_directives(content: &str) -> (Option<usize>, Vec<Raw<'_>>) {
    let content = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut raws: Vec<Raw<'_>> = Vec::new();
    let mut stray = None;
    // `lines()` also drops the `\r` of CRLF files, which is what we want: the
    // database stores `\n` whatever the checkout's line endings were.
    for (i, text) in content.lines().enumerate() {
        let line = i + 1;
        if let Some(rest) = text.strip_prefix("@@") {
            let (name, args) = match rest.find(char::is_whitespace) {
                Some(at) => (&rest[..at], rest[at..].trim()),
                None => (rest, ""),
            };
            raws.push(Raw {
                line,
                name,
                args,
                body: Vec::new(),
            });
        } else if let Some(open) = raws.last_mut() {
            open.body.push((line, text));
        } else if stray.is_none() && !text.trim().is_empty() {
            stray = Some(line);
        }
    }
    (stray, raws)
}

/// A body with its leading and trailing blank lines removed.
struct Body {
    /// Line of the first kept line.
    line: usize,
    text: String,
}

fn trim_body(lines: &[(usize, &str)]) -> Option<Body> {
    let has_text = |l: &(usize, &str)| !l.1.trim().is_empty();
    let first = lines.iter().position(has_text)?;
    let last = lines.iter().rposition(has_text)?;
    let kept = &lines[first..=last];
    Some(Body {
        line: kept[0].0,
        text: kept.iter().map(|l| l.1).collect::<Vec<_>>().join("\n"),
    })
}

/// Index of the next ` | ` separator in `s`. The pipe must have whitespace or
/// the end of the arguments on both sides, so `a || b` is not a separator and
/// an empty field (`rust | | caption`) still reads as an empty field.
fn next_separator(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    (0..b.len()).find(|&i| {
        b[i] == b'|'
            && (i == 0 || b[i - 1] == b' ' || b[i - 1] == b'\t')
            && (i + 1 == b.len() || b[i + 1] == b' ' || b[i + 1] == b'\t')
    })
}

fn clean(s: &str) -> Option<String> {
    let s = s.trim();
    (!s.is_empty()).then(|| s.to_string())
}

/// Splits directive arguments into exactly `N` trimmed fields. Missing and
/// empty fields are `None`. The last field keeps any further ` | `, so a
/// caption can contain one and a one-field directive takes the whole line.
fn split_fields<const N: usize>(args: &str) -> [Option<String>; N] {
    let mut out: Vec<Option<String>> = Vec::with_capacity(N);
    let mut rest = args;
    while out.len() + 1 < N {
        let Some(at) = next_separator(rest) else {
            break;
        };
        out.push(clean(&rest[..at]));
        rest = &rest[at + 1..];
    }
    out.push(clean(rest));
    let mut it = out.into_iter();
    std::array::from_fn(|_| it.next().flatten())
}

// ---------------------------------------------------------------------------
// Quiz bodies
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Marker {
    Prompt,
    Wrong,
    Right,
    Explain,
}

/// The marker a quiz line starts with, and the text after it. Markers sit in
/// column 0; anything else is not a quiz line.
fn quiz_line(text: &str) -> Option<(Marker, &str)> {
    for (prefix, marker) in [
        ("? ", Marker::Prompt),
        ("- ", Marker::Wrong),
        ("+ ", Marker::Right),
    ] {
        if let Some(rest) = text.strip_prefix(prefix) {
            return Some((marker, rest.trim()));
        }
    }
    if let Some(rest) = text.strip_prefix("= ") {
        return Some((Marker::Explain, rest.trim_end()));
    }
    // Editors strip the trailing space, so a bare `=` is an empty explanation
    // line: the way to put a paragraph break inside one.
    (text.trim_end() == "=").then_some((Marker::Explain, ""))
}

struct Question {
    line: usize,
    prompt: String,
    choices: Vec<String>,
    right: Vec<usize>,
    explanation: Vec<String>,
}

/// The JSON `src/lib/types.ts` calls `QuizMeta`.
#[derive(Serialize)]
struct QuizMetaOut<'a> {
    questions: &'a [QuizQuestionOut],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct QuizQuestionOut {
    id: String,
    prompt: String,
    choices: Vec<String>,
    answer_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
}

/// Parses a quiz body into the block's `meta_json`, or the list of
/// `(line, message)` problems.
fn parse_quiz(raw: &Raw<'_>) -> Result<String, Vec<(usize, String)>> {
    let mut errors: Vec<(usize, String)> = Vec::new();
    let mut questions: Vec<Question> = Vec::new();

    for &(line, text) in &raw.body {
        if text.trim().is_empty() {
            continue;
        }
        let Some((marker, rest)) = quiz_line(text) else {
            errors.push((
                line,
                format!(
                    "stray line in a quiz body: `{}` (lines start with `? `, `- `, `+ ` or `= `)",
                    text.trim()
                ),
            ));
            continue;
        };
        if marker == Marker::Prompt {
            if rest.is_empty() {
                errors.push((line, "a quiz question has no prompt after `? `".into()));
            }
            questions.push(Question {
                line,
                prompt: rest.to_string(),
                choices: Vec::new(),
                right: Vec::new(),
                explanation: Vec::new(),
            });
            continue;
        }
        let Some(q) = questions.last_mut() else {
            errors.push((
                line,
                format!(
                    "stray line in a quiz body: `{}` comes before any `? ` question",
                    text.trim()
                ),
            ));
            continue;
        };
        match marker {
            Marker::Explain => q.explanation.push(rest.to_string()),
            _ if rest.is_empty() => errors.push((line, "a quiz choice has no text".into())),
            Marker::Right => {
                q.right.push(q.choices.len());
                q.choices.push(rest.to_string());
            }
            _ => q.choices.push(rest.to_string()),
        }
    }

    if questions.is_empty() {
        errors.push((raw.line, "the quiz has no questions".into()));
    }
    for q in &questions {
        if q.choices.len() < 2 {
            errors.push((
                q.line,
                format!(
                    "question `{}` needs at least two choices, found {}",
                    q.prompt,
                    q.choices.len()
                ),
            ));
        }
        if q.right.len() != 1 {
            errors.push((
                q.line,
                format!(
                    "question `{}` needs exactly one `+` choice, found {}",
                    q.prompt,
                    q.right.len()
                ),
            ));
        }
    }
    if !errors.is_empty() {
        errors.sort_by_key(|e| e.0);
        return Err(errors);
    }

    let out: Vec<QuizQuestionOut> = questions
        .into_iter()
        .enumerate()
        .map(|(i, q)| QuizQuestionOut {
            id: format!("q{}", i + 1),
            prompt: q.prompt,
            answer_index: q.right[0],
            choices: q.choices,
            explanation: clean(&q.explanation.join("\n")),
        })
        .collect();
    let meta = serde_json::to_string(&QuizMetaOut { questions: &out })
        .map_err(|e| vec![(raw.line, format!("cannot encode the quiz: {e}"))])?;
    Ok(meta)
}

// ---------------------------------------------------------------------------
// The parser proper
// ---------------------------------------------------------------------------

/// State that resets at every file boundary.
#[derive(Default)]
struct FileState {
    part: Option<String>,
    section: Option<String>,
    /// Index into `Library::chapters` of the chapter blocks attach to.
    chapter: Option<usize>,
    /// Line of the current chapter's `@@objectives`, once seen.
    objectives_at: Option<usize>,
}

#[derive(Default)]
struct Parser {
    lib: Library,
    errors: Vec<LibError>,
    course_at: Option<(String, usize)>,
    /// `(part, display number)` -> where it was first used.
    displays: HashMap<(Option<String>, String), (String, usize)>,
}

impl Parser {
    fn error(&mut self, file: &str, line: usize, message: impl Into<String>) {
        self.errors.push(LibError {
            file: file.to_string(),
            line,
            message: message.into(),
        });
    }

    fn warn(&mut self, file: &str, line: usize, message: impl Into<String>) {
        self.lib.warnings.push(LibWarning {
            file: file.to_string(),
            line,
            message: message.into(),
        });
    }

    fn parse_file(&mut self, file: &str, content: &str) {
        self.lib.files.push(file.to_string());
        let (stray, raws) = split_directives(content);
        if let Some(line) = stray {
            self.warn(
                file,
                line,
                "text before the first directive belongs to nothing and is ignored",
            );
        }
        let mut st = FileState::default();
        for raw in &raws {
            self.directive(file, &mut st, raw);
        }
        self.close_chapter(file, st.chapter);
    }

    fn directive(&mut self, file: &str, st: &mut FileState, raw: &Raw<'_>) {
        match raw.name {
            "course" => self.course(file, raw),
            "part" | "section" => {
                self.require_no_body(file, raw);
                let [title] = split_fields::<1>(raw.args);
                if raw.name == "part" {
                    st.part = title;
                } else {
                    st.section = title;
                }
            }
            "chapter" => self.chapter(file, st, raw),
            "objectives" => self.objectives(file, st, raw),
            "video" | "image" | "attachment" | "text" | "callout" | "code" | "quiz" | "divider" => {
                self.block(file, st, raw);
            }
            other => self.error(file, raw.line, format!("unknown directive `@@{other}`")),
        }
    }

    fn require_no_body(&mut self, file: &str, raw: &Raw<'_>) {
        if let Some(body) = trim_body(&raw.body) {
            self.error(
                file,
                body.line,
                format!(
                    "`@@{}` (line {}) takes no body, but there is text under it",
                    raw.name, raw.line
                ),
            );
        }
    }

    fn course(&mut self, file: &str, raw: &Raw<'_>) {
        if let Some((first_file, first_line)) = self.course_at.clone() {
            self.error(
                file,
                raw.line,
                format!("a second `@@course` (the first is at {first_file}:{first_line})"),
            );
            return;
        }
        self.course_at = Some((file.to_string(), raw.line));
        let [title, subtitle, author] = split_fields::<3>(raw.args);
        self.lib.course = Some(CourseInfo {
            title,
            subtitle,
            author,
            description_md: trim_body(&raw.body).map(|b| b.text),
        });
    }

    fn chapter(&mut self, file: &str, st: &mut FileState, raw: &Raw<'_>) {
        self.close_chapter(file, st.chapter);
        let [display, title, minutes] = split_fields::<3>(raw.args);
        if title.is_none() {
            self.error(file, raw.line, "`@@chapter` has no title");
        }
        let label = display
            .clone()
            .or_else(|| title.clone())
            .unwrap_or_else(|| "(untitled)".to_string());

        let estimated_minutes = minutes.and_then(|m| match m.parse::<u32>() {
            Ok(n) => Some(i64::from(n)),
            Err(_) => {
                self.warn(
                    file,
                    raw.line,
                    format!("chapter `{label}`: estimated minutes `{m}` is not a whole number and is ignored"),
                );
                None
            }
        });

        if let Some(d) = &display {
            let key = (st.part.clone(), d.clone());
            if let Some((first_file, first_line)) = self.displays.get(&key).cloned() {
                self.warn(
                    file,
                    raw.line,
                    format!(
                        "display number `{d}` is used twice in {} (first at {first_file}:{first_line})",
                        match &st.part {
                            Some(p) => format!("part `{p}`"),
                            None => "the chapters that have no part".to_string(),
                        }
                    ),
                );
            } else {
                self.displays.insert(key, (file.to_string(), raw.line));
            }
        }

        let missing = match (st.part.is_none(), st.section.is_none()) {
            (true, true) => Some("no `@@part` and no `@@section`"),
            (true, false) => Some("no `@@part`"),
            (false, true) => Some("no `@@section`"),
            (false, false) => None,
        };
        if let Some(what) = missing {
            self.warn(
                file,
                raw.line,
                format!("chapter `{label}` has {what} above it in this file"),
            );
        }

        st.chapter = Some(self.lib.chapters.len());
        st.objectives_at = None;
        self.lib.chapters.push(LibChapter {
            file: file.to_string(),
            line: raw.line,
            part: st.part.clone(),
            section: st.section.clone(),
            display_number: display,
            title: title.unwrap_or_default(),
            summary_md: trim_body(&raw.body).map(|b| b.text),
            objectives_md: None,
            estimated_minutes,
            blocks: Vec::new(),
        });
    }

    /// Warns about a chapter that ended up with no blocks. Called when the
    /// chapter is closed, by the next `@@chapter` or the end of the file.
    fn close_chapter(&mut self, file: &str, chapter: Option<usize>) {
        let Some(idx) = chapter else { return };
        let c = &self.lib.chapters[idx];
        if c.blocks.is_empty() {
            let (line, label) = (
                c.line,
                c.display_number.clone().unwrap_or_else(|| c.title.clone()),
            );
            self.warn(file, line, format!("chapter `{label}` has no blocks"));
        }
    }

    fn objectives(&mut self, file: &str, st: &mut FileState, raw: &Raw<'_>) {
        let Some(idx) = st.chapter else {
            self.error(
                file,
                raw.line,
                "`@@objectives` comes before any `@@chapter`",
            );
            return;
        };
        if let Some(first) = st.objectives_at {
            self.error(
                file,
                raw.line,
                format!("a second `@@objectives` in this chapter (the first is at line {first})"),
            );
            return;
        }
        st.objectives_at = Some(raw.line);
        if !raw.args.is_empty() {
            self.warn(
                file,
                raw.line,
                "`@@objectives` takes no arguments; the text after it is ignored (put the bullets on the lines below)",
            );
        }
        self.lib.chapters[idx].objectives_md = trim_body(&raw.body).map(|b| b.text);
    }

    fn block(&mut self, file: &str, st: &FileState, raw: &Raw<'_>) {
        let Some(chapter) = st.chapter else {
            self.error(
                file,
                raw.line,
                format!("`@@{}` comes before any `@@chapter`", raw.name),
            );
            return;
        };
        let Some(kind) = BlockKind::parse(raw.name) else {
            return;
        };
        let body = trim_body(&raw.body);
        let mut block = LibBlock {
            line: raw.line,
            body_line: body.as_ref().map_or(raw.line + 1, |b| b.line),
            kind,
            heading: None,
            caption_md: None,
            body_md: None,
            code_text: None,
            code_lang: None,
            meta_json: None,
        };

        match kind {
            BlockKind::Video | BlockKind::Image | BlockKind::Attachment => {
                let [heading, caption] = split_fields::<2>(raw.args);
                block.heading = heading;
                block.caption_md = match (caption, body) {
                    (Some(c), Some(b)) => Some(format!("{c}\n{}", b.text)),
                    (Some(c), None) => Some(c),
                    (None, Some(b)) => Some(b.text),
                    (None, None) => None,
                };
            }
            BlockKind::Text => {
                let [heading] = split_fields::<1>(raw.args);
                block.heading = heading;
                block.body_md = body.map(|b| b.text);
            }
            BlockKind::Callout => {
                let [tone, heading] = split_fields::<2>(raw.args);
                match tone.as_deref() {
                    Some(t) if CALLOUT_TONES.contains(&t) => {
                        block.meta_json = Some(serde_json::json!({ "tone": t }).to_string());
                    }
                    other => self.error(
                        file,
                        raw.line,
                        format!(
                            "unknown callout tone `{}` (use {})",
                            other.unwrap_or(""),
                            CALLOUT_TONES.join(", ")
                        ),
                    ),
                }
                block.heading = heading;
                block.body_md = body.map(|b| b.text);
            }
            BlockKind::Code => {
                let [lang, heading, caption] = split_fields::<3>(raw.args);
                match lang.as_deref() {
                    Some(l) if CODE_LANGS.contains(&l) => block.code_lang = Some(l.to_string()),
                    other => self.error(
                        file,
                        raw.line,
                        format!(
                            "unknown code language `{}` (use {})",
                            other.unwrap_or(""),
                            CODE_LANGS.join(", ")
                        ),
                    ),
                }
                block.heading = heading;
                block.caption_md = caption;
                block.code_text = body.map(|b| b.text);
            }
            BlockKind::Quiz => {
                let [heading, caption] = split_fields::<2>(raw.args);
                block.heading = heading;
                block.caption_md = caption;
                match parse_quiz(raw) {
                    Ok(meta) => block.meta_json = Some(meta),
                    Err(problems) => {
                        for (line, message) in problems {
                            self.error(file, line, message);
                        }
                    }
                }
            }
            BlockKind::Divider => {
                self.require_no_body(file, raw);
                if !raw.args.is_empty() {
                    self.warn(
                        file,
                        raw.line,
                        "`@@divider` takes no arguments; they are ignored",
                    );
                }
            }
        }
        self.lib.chapters[chapter].blocks.push(block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_split_on_space_pipe_space_only() {
        let [a, b, c] = split_fields::<3>("1.2.2 | Lesson 2.2 - Variables | 20");
        assert_eq!(a.as_deref(), Some("1.2.2"));
        assert_eq!(b.as_deref(), Some("Lesson 2.2 - Variables"));
        assert_eq!(c.as_deref(), Some("20"));

        // `||` and a pipe glued to a word are text, not separators.
        let [x, y] = split_fields::<2>("a || b | c|d");
        assert_eq!(x.as_deref(), Some("a || b"));
        assert_eq!(y.as_deref(), Some("c|d"));
    }

    #[test]
    fn missing_and_empty_fields_are_none() {
        let [a, b, c] = split_fields::<3>("rust | | a caption");
        assert_eq!(a.as_deref(), Some("rust"));
        assert_eq!(b, None);
        assert_eq!(c.as_deref(), Some("a caption"));

        let [a, b, c] = split_fields::<3>("only");
        assert_eq!(a.as_deref(), Some("only"));
        assert_eq!((b, c), (None, None));

        let [a, b] = split_fields::<2>("");
        assert_eq!((a, b), (None, None));
    }

    #[test]
    fn the_last_field_keeps_further_separators() {
        let [h, c] = split_fields::<2>("Lesson video | Ardan | Crash Course | 5:07");
        assert_eq!(h.as_deref(), Some("Lesson video"));
        assert_eq!(c.as_deref(), Some("Ardan | Crash Course | 5:07"));

        let [t] = split_fields::<1>("Plan | Do | Check");
        assert_eq!(t.as_deref(), Some("Plan | Do | Check"));
    }

    #[test]
    fn body_trimming_removes_only_blank_edges() {
        let lines = [
            (1, ""),
            (2, "  "),
            (3, "    indented"),
            (4, ""),
            (5, "tail  "),
            (6, ""),
        ];
        let body = trim_body(&lines).expect("body");
        assert_eq!(body.line, 3);
        assert_eq!(body.text, "    indented\n\ntail  ");
        assert!(trim_body(&[(1, ""), (2, "\t")]).is_none());
    }

    #[test]
    fn bare_equals_is_an_empty_explanation_line() {
        assert_eq!(quiz_line("="), Some((Marker::Explain, "")));
        assert_eq!(quiz_line("= "), Some((Marker::Explain, "")));
        assert_eq!(quiz_line("= why"), Some((Marker::Explain, "why")));
        assert!(quiz_line("=text").is_none());
        assert!(quiz_line("-").is_none());
        assert!(quiz_line("  - indented").is_none());
    }
}
