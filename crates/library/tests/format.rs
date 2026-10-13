//! The parser against `library/FORMAT.md`.

use rustclass_core::models::BlockKind;
use rustclass_library::{parse_files, LibError, Library};
use serde::Deserialize;
use serde_json::json;

/// The example at the end of FORMAT.md, character for character.
const EXAMPLE: &str = r#"@@part 1 · Ultimate Rust Crash Course
@@section Module 2: Fundamentals
@@chapter 1.2.2 | Lesson 2.2 - Variables | 20
Bindings, immutability, `mut`, constants and shadowing.
@@objectives
- Declare a binding and say why it cannot change by default
- Choose between `let`, `let mut` and `const`
@@video Lesson video | **Ardan Labs** · Ultimate Rust Crash Course · Lesson 2.2 - Variables · 5:07 — attach the MP4 here.
@@callout note | Plant situation
You are commissioning a bottle filler …
@@text Teacher's notes
In a PLC every tag you declare …
@@code mermaid | Diagram: where a binding lives
flowchart LR
  A["let x = 5"] --> B["x is read-only"]
@@code rust | Try it
fn main() {
    let fill_ml = 500;
    println!("{fill_ml}");
}
@@quiz Check yourself
? Which keyword makes a binding changeable?
- const
+ mut
= `mut` is the opt-in.
"#;

fn one(content: &str) -> Result<Library, Vec<LibError>> {
    parse_files(&[("t.md".to_string(), content.to_string())])
}

fn files(list: &[(&str, &str)]) -> Result<Library, Vec<LibError>> {
    let owned: Vec<(String, String)> = list
        .iter()
        .map(|(n, c)| (n.to_string(), c.to_string()))
        .collect();
    parse_files(&owned)
}

/// A valid start of a file: part, section and one chapter. Three lines, so the
/// first line of whatever follows is line 4.
const HEADER: &str = "@@part P\n@@section S\n@@chapter 1.1.1 | Title | 5\n";

fn kinds(lib: &Library) -> Vec<BlockKind> {
    lib.chapters[0].blocks.iter().map(|b| b.kind).collect()
}

#[test]
fn parses_the_format_md_example() {
    let lib = files(&[("1-02-example.md", EXAMPLE)]).expect("the FORMAT.md example parses");

    assert_eq!(lib.files, ["1-02-example.md"]);
    assert!(lib.course.is_none());
    assert!(lib.warnings.is_empty(), "{:?}", lib.warnings);
    assert_eq!(lib.chapters.len(), 1);

    let c = &lib.chapters[0];
    assert_eq!((c.file.as_str(), c.line), ("1-02-example.md", 3));
    assert_eq!(c.part.as_deref(), Some("1 · Ultimate Rust Crash Course"));
    assert_eq!(c.section.as_deref(), Some("Module 2: Fundamentals"));
    assert_eq!(c.display_number.as_deref(), Some("1.2.2"));
    assert_eq!(c.title, "Lesson 2.2 - Variables");
    assert_eq!(c.estimated_minutes, Some(20));
    assert_eq!(
        c.summary_md.as_deref(),
        Some("Bindings, immutability, `mut`, constants and shadowing.")
    );
    assert_eq!(
        c.objectives_md.as_deref(),
        Some(
            "- Declare a binding and say why it cannot change by default\n\
             - Choose between `let`, `let mut` and `const`"
        )
    );
    assert_eq!(
        kinds(&lib),
        [
            BlockKind::Video,
            BlockKind::Callout,
            BlockKind::Text,
            BlockKind::Code,
            BlockKind::Code,
            BlockKind::Quiz
        ]
    );

    let [video, callout, text, mermaid, rust, quiz] = &c.blocks[..] else {
        panic!("six blocks");
    };
    assert_eq!(video.heading.as_deref(), Some("Lesson video"));
    assert_eq!(
        video.caption_md.as_deref(),
        Some("**Ardan Labs** · Ultimate Rust Crash Course · Lesson 2.2 - Variables · 5:07 — attach the MP4 here.")
    );
    assert_eq!(video.line, 8);
    assert!(video.body_md.is_none() && video.code_text.is_none());

    assert_eq!(callout.heading.as_deref(), Some("Plant situation"));
    assert_eq!(
        callout.body_md.as_deref(),
        Some("You are commissioning a bottle filler …")
    );
    assert_eq!(callout.meta_json.as_deref(), Some(r#"{"tone":"note"}"#));

    assert_eq!(text.heading.as_deref(), Some("Teacher's notes"));
    assert_eq!(
        text.body_md.as_deref(),
        Some("In a PLC every tag you declare …")
    );

    assert_eq!(mermaid.code_lang.as_deref(), Some("mermaid"));
    assert_eq!(
        mermaid.heading.as_deref(),
        Some("Diagram: where a binding lives")
    );
    assert_eq!(
        mermaid.code_text.as_deref(),
        Some("flowchart LR\n  A[\"let x = 5\"] --> B[\"x is read-only\"]")
    );

    assert_eq!(rust.code_lang.as_deref(), Some("rust"));
    assert_eq!(rust.heading.as_deref(), Some("Try it"));
    assert_eq!(
        rust.code_text.as_deref(),
        Some("fn main() {\n    let fill_ml = 500;\n    println!(\"{fill_ml}\");\n}")
    );
    assert_eq!(rust.line, 16);
    assert_eq!(rust.body_line, 17);

    assert_eq!(quiz.heading.as_deref(), Some("Check yourself"));
    assert_eq!(
        quiz.meta_json.as_deref(),
        Some(
            r#"{"questions":[{"id":"q1","prompt":"Which keyword makes a binding changeable?","choices":["const","mut"],"answerIndex":1,"explanation":"`mut` is the opt-in."}]}"#
        )
    );
}

#[test]
fn format_md_example_in_the_live_file_still_parses() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../library/FORMAT.md");
    let Ok(text) = std::fs::read_to_string(&path) else {
        println!("skipping: {} is not there", path.display());
        return;
    };
    let after = text
        .split_once("## Example")
        .expect("FORMAT.md has an Example section")
        .1;
    let fenced = after
        .split("```")
        .nth(1)
        .expect("the example is in a code fence");
    let fenced = fenced.strip_prefix('\n').unwrap_or(fenced);
    let lib = files(&[("example.md", fenced)]).expect("the live FORMAT.md example parses");
    assert_eq!(lib.chapters.len(), 1);
    assert!(lib.chapters[0].blocks.len() >= 5);
    assert!(lib.warnings.is_empty(), "{:?}", lib.warnings);
}

#[test]
fn every_hard_error_names_file_and_line() {
    struct Case {
        label: &'static str,
        content: String,
        line: usize,
        says: &'static str,
    }
    let case = |label, content: &str, line, says| Case {
        label,
        content: content.to_string(),
        line,
        says,
    };
    let with_header = |rest: &str| format!("{HEADER}{rest}");

    let cases = vec![
        case(
            "unknown directive",
            &with_header("@@bogus x\n"),
            4,
            "unknown directive `@@bogus`",
        ),
        case(
            "block before any chapter",
            "@@part P\n@@section S\n@@text Notes\nbody\n",
            3,
            "before any `@@chapter`",
        ),
        case(
            "objectives before any chapter",
            "@@part P\n@@section S\n@@objectives\n- a\n",
            3,
            "before any `@@chapter`",
        ),
        case("body under @@part", "@@part P\nstray\n", 2, "takes no body"),
        case(
            "body under @@section",
            "@@part P\n@@section S\n\nstray\n",
            4,
            "takes no body",
        ),
        case(
            "body under @@divider",
            &with_header("@@divider\n\nstray\n"),
            6,
            "takes no body",
        ),
        case(
            "unknown callout tone",
            &with_header("@@callout loud | H\nbody\n"),
            4,
            "unknown callout tone `loud`",
        ),
        case(
            "callout with no tone",
            &with_header("@@callout\nbody\n"),
            4,
            "unknown callout tone ``",
        ),
        case(
            "unknown code lang",
            &with_header("@@code cobol | H\nx\n"),
            4,
            "unknown code language `cobol`",
        ),
        case(
            "code with no lang",
            &with_header("@@code | H\nx\n"),
            4,
            "unknown code language ``",
        ),
        case(
            "chapter with an empty title",
            "@@part P\n@@section S\n@@chapter 1.1.1 |  | 5\n",
            3,
            "no title",
        ),
        case(
            "chapter with no arguments",
            "@@part P\n@@section S\n@@chapter\n",
            3,
            "no title",
        ),
        case(
            "quiz with no questions",
            &with_header("@@quiz Check\n\n\n"),
            4,
            "no questions",
        ),
        case(
            "question with no right choice",
            &with_header("@@quiz Q\n? What?\n- a\n- b\n"),
            5,
            "exactly one `+`",
        ),
        case(
            "question with two right choices",
            &with_header("@@quiz Q\n? What?\n+ a\n+ b\n"),
            5,
            "exactly one `+`",
        ),
        case(
            "question with one choice",
            &with_header("@@quiz Q\n? What?\n+ a\n"),
            5,
            "at least two choices",
        ),
        case(
            "stray line in a quiz body",
            &with_header("@@quiz Q\n? What?\n- a\n+ b\nstray text\n"),
            8,
            "stray line in a quiz body",
        ),
        case(
            "indented quiz line",
            &with_header("@@quiz Q\n? What?\n- a\n+ b\n  - c\n"),
            8,
            "stray line in a quiz body",
        ),
        case(
            "quiz choice before any question",
            &with_header("@@quiz Q\n- a\n? What?\n+ b\n- c\n"),
            5,
            "before any `? ` question",
        ),
        case(
            "quiz choice with no text",
            &with_header("@@quiz Q\n? What?\n- a\n+ b\n- \n"),
            8,
            "has no text",
        ),
        case(
            "second @@objectives in one chapter",
            &with_header("@@objectives\n- a\n@@objectives\n- b\n"),
            6,
            "second `@@objectives`",
        ),
        case(
            "second @@course",
            "@@course A | B | C\nabout\n@@course D\n",
            3,
            "second `@@course`",
        ),
    ];

    for c in cases {
        let errs = one(&c.content).expect_err(c.label);
        assert_eq!(errs.len(), 1, "{}: {errs:?}", c.label);
        let e = &errs[0];
        assert_eq!(e.file, "t.md", "{}", c.label);
        assert_eq!(e.line, c.line, "{}: {e}", c.label);
        assert!(e.message.contains(c.says), "{}: {e}", c.label);
        assert!(
            e.to_string().starts_with(&format!("t.md:{}: ", c.line)),
            "{}: {e}",
            c.label
        );
    }
}

#[test]
fn errors_name_the_right_file_and_all_of_them_are_reported() {
    // The second @@course is in another file than the first.
    let errs = files(&[
        ("a.md", "@@course A\n"),
        ("b.md", &format!("{HEADER}@@text x\nbody\n@@course B\n")),
    ])
    .unwrap_err();
    assert_eq!(errs.len(), 1);
    assert_eq!((errs[0].file.as_str(), errs[0].line), ("b.md", 6));
    assert!(errs[0].message.contains("a.md:1"), "{}", errs[0]);

    // Nothing stops at the first problem, and each is filed under its own file.
    let errs = files(&[
        ("a.md", &format!("{HEADER}@@nope\n")),
        ("b.md", "@@text x\nbody\n"),
        ("c.md", &format!("{HEADER}@@callout loud\n")),
    ])
    .unwrap_err();
    let places: Vec<(&str, usize)> = errs.iter().map(|e| (e.file.as_str(), e.line)).collect();
    assert_eq!(places, [("a.md", 4), ("b.md", 1), ("c.md", 4)]);

    // A block at the top of a file never joins the previous file's chapter.
    let errs = files(&[
        ("a.md", &format!("{HEADER}@@text x\nbody\n")),
        ("b.md", "@@text y\nbody\n"),
    ])
    .unwrap_err();
    assert_eq!((errs[0].file.as_str(), errs[0].line), ("b.md", 1));
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct FrontendQuestion {
    id: String,
    prompt: String,
    choices: Vec<String>,
    answer_index: usize,
    explanation: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrontendQuiz {
    questions: Vec<FrontendQuestion>,
}

#[test]
fn quiz_meta_matches_frontend_shape() {
    let lib = one(&format!(
        "{HEADER}@@quiz Check yourself | A caption\n\
         ? Which keyword makes a binding changeable?\n\
         - const\n\
         + mut\n\
         - static\n\
         = `mut` opts one binding into change.\n\
         =\n\
         = Everything else stays fixed.\n\
         \n\
         ? Second question\n\
         + yes\n\
         - no\n"
    ))
    .expect("valid quiz");
    let block = &lib.chapters[0].blocks[0];
    assert_eq!(block.kind, BlockKind::Quiz);
    assert_eq!(block.heading.as_deref(), Some("Check yourself"));
    assert_eq!(block.caption_md.as_deref(), Some("A caption"));
    assert!(block.body_md.is_none() && block.code_text.is_none());
    let meta = block.meta_json.as_deref().expect("quiz meta");

    // The exact bytes FORMAT.md documents: key order, and no `explanation`
    // key at all when there is none.
    assert_eq!(
        meta,
        concat!(
            r#"{"questions":["#,
            r#"{"id":"q1","prompt":"Which keyword makes a binding changeable?","#,
            r#""choices":["const","mut","static"],"answerIndex":1,"#,
            r#""explanation":"`mut` opts one binding into change.\n\nEverything else stays fixed."},"#,
            r#"{"id":"q2","prompt":"Second question","choices":["yes","no"],"answerIndex":0}"#,
            r#"]}"#
        )
    );

    // And it deserialises into exactly what `QuizMeta` in src/lib/types.ts reads.
    let quiz: FrontendQuiz = serde_json::from_str(meta).expect("QuizMeta shape");
    assert_eq!(quiz.questions.len(), 2);
    let mut ids: Vec<&str> = quiz.questions.iter().map(|q| q.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids, ["q1", "q2"], "ids are unique within the block");
    for q in &quiz.questions {
        assert!(!q.prompt.is_empty());
        assert!(q.choices.len() >= 2);
        assert!(q.answer_index < q.choices.len());
    }
    assert_eq!(
        quiz.questions[0].choices[quiz.questions[0].answer_index],
        "mut"
    );
    assert_eq!(quiz.questions[1].explanation, None);

    // The core treats it as a ready quiz.
    assert!(rustclass_core::state::quiz_has_questions(meta));
}

#[test]
fn callout_meta_is_the_tone_and_nothing_else() {
    for tone in ["note", "tip", "warning", "danger"] {
        let lib = one(&format!("{HEADER}@@callout {tone} | H\nbody\n")).expect(tone);
        let meta: serde_json::Value =
            serde_json::from_str(lib.chapters[0].blocks[0].meta_json.as_deref().unwrap()).unwrap();
        assert_eq!(meta, json!({ "tone": tone }));
    }
}

#[test]
fn part_and_section_reset_per_file() {
    let lib = files(&[
        (
            "a.md",
            "@@part Part A\n@@section Section A\n@@chapter 1.1.1 | One | 5\n@@text x\nbody\n",
        ),
        // Nothing set in this file: must not inherit from a.md.
        ("b.md", "@@chapter 2.1.1 | Two | 5\n@@text x\nbody\n"),
        // Only a section: the part from a.md must be gone.
        (
            "c.md",
            "@@section Section C\n@@chapter 3.1.1 | Three | 5\n@@text x\nbody\n",
        ),
        // Only a part: the section from c.md must be gone.
        (
            "d.md",
            "@@part Part D\n@@chapter 4.1.1 | Four | 5\n@@text x\nbody\n",
        ),
        // Both again, two chapters sharing them.
        (
            "e.md",
            "@@part Part E\n@@section Section E\n@@chapter 5.1.1 | Five | 5\n@@text x\nbody\n\
             @@chapter 5.1.2 | Six | 5\n@@text x\nbody\n",
        ),
    ])
    .expect("valid");

    let grouping: Vec<(&str, Option<&str>, Option<&str>)> = lib
        .chapters
        .iter()
        .map(|c| (c.title.as_str(), c.part.as_deref(), c.section.as_deref()))
        .collect();
    assert_eq!(
        grouping,
        [
            ("One", Some("Part A"), Some("Section A")),
            ("Two", None, None),
            ("Three", None, Some("Section C")),
            ("Four", Some("Part D"), None),
            ("Five", Some("Part E"), Some("Section E")),
            ("Six", Some("Part E"), Some("Section E")),
        ]
    );

    let warnings: Vec<(&str, usize, &str)> = lib
        .warnings
        .iter()
        .map(|w| (w.file.as_str(), w.line, w.message.as_str()))
        .collect();
    assert_eq!(warnings.len(), 3, "{warnings:?}");
    assert_eq!((warnings[0].0, warnings[0].1), ("b.md", 1));
    assert!(warnings[0].2.contains("no `@@part` and no `@@section`"));
    assert_eq!((warnings[1].0, warnings[1].1), ("c.md", 2));
    assert!(warnings[1].2.contains("no `@@part`") && !warnings[1].2.contains("section"));
    assert_eq!((warnings[2].0, warnings[2].1), ("d.md", 2));
    assert!(warnings[2].2.contains("no `@@section`") && !warnings[2].2.contains("part"));
}

#[test]
fn warnings_are_the_three_in_format_md_and_do_not_stop_the_load() {
    // A chapter with no blocks, in the middle and at the end of a file.
    let lib = one("@@part P\n@@section S\n\
         @@chapter 1.1.1 | Empty middle | 5\n\
         @@chapter 1.1.2 | Fine | 5\n@@text x\nbody\n\
         @@chapter 1.1.3 | Empty end | 5\n")
    .expect("warnings are not errors");
    let said: Vec<(usize, &str)> = lib
        .warnings
        .iter()
        .map(|w| (w.line, w.message.as_str()))
        .collect();
    assert_eq!(said.len(), 2, "{said:?}");
    assert_eq!(said[0].0, 3);
    assert!(said[0].1.contains("1.1.1") && said[0].1.contains("no blocks"));
    assert_eq!(said[1].0, 7);
    assert!(said[1].1.contains("1.1.3") && said[1].1.contains("no blocks"));
    assert_eq!(lib.chapters.len(), 3);

    // A duplicate display number inside one part, across files too, but not
    // between two different parts.
    let lib = files(&[
        (
            "a.md",
            "@@part P\n@@section S\n@@chapter 1.1.1 | A | 5\n@@text x\nbody\n\
             @@chapter 1.1.1 | B | 5\n@@text x\nbody\n",
        ),
        (
            "b.md",
            "@@part P\n@@section T\n@@chapter 1.1.1 | C | 5\n@@text x\nbody\n",
        ),
        (
            "c.md",
            "@@part Q\n@@section S\n@@chapter 1.1.1 | D | 5\n@@text x\nbody\n",
        ),
    ])
    .expect("warnings are not errors");
    let dupes: Vec<(&str, usize)> = lib
        .warnings
        .iter()
        .map(|w| (w.file.as_str(), w.line))
        .collect();
    assert_eq!(dupes, [("a.md", 6), ("b.md", 3)]);
    assert!(lib.warnings[0].message.contains("1.1.1"));
    assert!(
        lib.warnings[1].message.contains("a.md:3"),
        "names the first use"
    );
    assert_eq!(lib.chapters.len(), 4);

    // A clean file has none, and a chapter with no display number never
    // counts as a duplicate.
    let lib = one(
        "@@part P\n@@section S\n@@chapter | A | 5\n@@text x\nbody\n@@chapter | B | 5\n@@text x\nbody\n",
    )
    .expect("valid");
    assert!(lib.warnings.is_empty(), "{:?}", lib.warnings);
}

#[test]
fn directives_start_in_column_zero_only() {
    let lib = one(&format!(
        "{HEADER}@@text Notes\n\
         An inline @@ sign and an email a@@b are text.\n\
         \x20   @@chapter 9.9.9 | Indented, so not a directive | 1\n\
         \t@@video also text\n"
    ))
    .expect("valid");
    assert_eq!(lib.chapters.len(), 1);
    assert_eq!(kinds(&lib), [BlockKind::Text]);
    let body = lib.chapters[0].blocks[0].body_md.as_deref().unwrap();
    assert!(body.contains("an email a@@b are text."));
    assert!(body.contains("    @@chapter 9.9.9"));
    assert!(body.ends_with("\t@@video also text"));
}

#[test]
fn bodies_keep_everything_between_the_blank_edges() {
    let content = format!(
        "{HEADER}@@text Notes\n\n\nIntro  \n\n```rust\nfn main() {{\n    println!(\"hi\");\n}}\n```\n\n    indented\n\ttabbed  \n\n\n\
         @@code rust | Try\n   \n\nfn main() {{}}\n\n\n"
    );
    let lib = one(&content).expect("valid");
    let [text, code] = &lib.chapters[0].blocks[..] else {
        panic!("two blocks");
    };
    assert_eq!(
        text.body_md.as_deref(),
        Some("Intro  \n\n```rust\nfn main() {\n    println!(\"hi\");\n}\n```\n\n    indented\n\ttabbed  ")
    );
    assert_eq!(code.code_text.as_deref(), Some("fn main() {}"));
    // `body_line` is where the code really starts in the file.
    let at = content.lines().position(|l| l == "fn main() {}").unwrap() + 1;
    assert_eq!(code.body_line, at);
}

#[test]
fn crlf_files_and_a_byte_order_mark_parse_like_plain_ones() {
    let plain = files(&[("x.md", EXAMPLE)]).expect("valid");
    let crlf = EXAMPLE.replace('\n', "\r\n");
    let windows = files(&[("x.md", &format!("\u{feff}{crlf}"))]).expect("valid");
    assert_eq!(plain, windows);
}

#[test]
fn block_directives_fill_the_documented_fields() {
    let lib = one(&format!(
        "{HEADER}\
         @@video Lesson video | Caption one\nmore caption text\n\
         @@image | Only a caption\n\
         @@attachment Sheet\nthe caption is in the body\n\
         @@text\nbody only\n\
         @@callout danger | Careful\n\
         @@code json | Payload | The payload\n{{\"a\": 1}}\n\
         @@code text\nplain\n\
         @@divider\n"
    ))
    .expect("valid");
    let b = &lib.chapters[0].blocks;
    assert_eq!(
        kinds(&lib),
        [
            BlockKind::Video,
            BlockKind::Image,
            BlockKind::Attachment,
            BlockKind::Text,
            BlockKind::Callout,
            BlockKind::Code,
            BlockKind::Code,
            BlockKind::Divider
        ]
    );

    assert_eq!(b[0].heading.as_deref(), Some("Lesson video"));
    assert_eq!(
        b[0].caption_md.as_deref(),
        Some("Caption one\nmore caption text")
    );

    assert_eq!(b[1].heading, None);
    assert_eq!(b[1].caption_md.as_deref(), Some("Only a caption"));

    assert_eq!(b[2].heading.as_deref(), Some("Sheet"));
    assert_eq!(
        b[2].caption_md.as_deref(),
        Some("the caption is in the body")
    );

    assert_eq!(b[3].heading, None);
    assert_eq!(b[3].body_md.as_deref(), Some("body only"));

    // A callout with no body is allowed: it is the empty placeholder.
    assert_eq!(b[4].heading.as_deref(), Some("Careful"));
    assert_eq!(b[4].body_md, None);

    assert_eq!(b[5].code_lang.as_deref(), Some("json"));
    assert_eq!(b[5].heading.as_deref(), Some("Payload"));
    assert_eq!(b[5].caption_md.as_deref(), Some("The payload"));
    assert_eq!(b[5].code_text.as_deref(), Some("{\"a\": 1}"));

    assert_eq!(b[6].heading, None);
    assert_eq!(b[6].code_text.as_deref(), Some("plain"));

    assert!(b[7].heading.is_none() && b[7].body_md.is_none() && b[7].meta_json.is_none());
    assert!(lib.warnings.is_empty(), "{:?}", lib.warnings);
}

#[test]
fn every_code_lang_in_format_md_is_accepted() {
    for lang in ["rust", "toml", "bash", "json", "text", "mermaid"] {
        let lib = one(&format!("{HEADER}@@code {lang} | H\nx\n")).expect(lang);
        assert_eq!(lib.chapters[0].blocks[0].code_lang.as_deref(), Some(lang));
    }
}

#[test]
fn course_directive_carries_title_subtitle_author_and_description() {
    let lib = files(&[(
        "0-00.md",
        "@@course Ultimate Rust Bundle | Learn Rust for the plant floor | Dinesh\n\
         Line one.\n\nLine two.\n\n@@part P\n",
    )])
    .expect("valid");
    let course = lib.course.expect("course");
    assert_eq!(course.title.as_deref(), Some("Ultimate Rust Bundle"));
    assert_eq!(
        course.subtitle.as_deref(),
        Some("Learn Rust for the plant floor")
    );
    assert_eq!(course.author.as_deref(), Some("Dinesh"));
    assert_eq!(
        course.description_md.as_deref(),
        Some("Line one.\n\nLine two.")
    );

    let lib = one("@@course Only a title\n").expect("valid");
    let course = lib.course.expect("course");
    assert_eq!(course.title.as_deref(), Some("Only a title"));
    assert_eq!(
        (course.subtitle, course.author, course.description_md),
        (None, None, None)
    );
}

#[test]
fn text_before_the_first_directive_and_odd_arguments_only_warn() {
    let lib = one(&format!(
        "# A stray title\n\n{HEADER}@@objectives - inline bullet\n- real bullet\n@@text x\nbody\n\
         @@chapter 1.1.2 | Minutes | twenty\n@@divider now\n"
    ))
    .expect("only warnings");
    let said: Vec<(usize, &str)> = lib
        .warnings
        .iter()
        .map(|w| (w.line, w.message.as_str()))
        .collect();
    assert_eq!(said.len(), 4, "{said:?}");
    assert_eq!(said[0].0, 1);
    assert!(said[0].1.contains("before the first directive"));
    assert_eq!(said[1].0, 6);
    assert!(said[1].1.contains("`@@objectives` takes no arguments"));
    assert_eq!(said[2].0, 10);
    assert!(said[2].1.contains("`twenty`") && said[2].1.contains("not a whole number"));
    assert_eq!(said[3].0, 11);
    assert!(said[3].1.contains("`@@divider` takes no arguments"));

    // Nothing the warnings talk about leaks into the content.
    assert_eq!(
        lib.chapters[0].objectives_md.as_deref(),
        Some("- real bullet")
    );
    assert_eq!(lib.chapters[1].estimated_minutes, None);
    assert_eq!(kinds_of(&lib, 1), [BlockKind::Divider]);
}

fn kinds_of(lib: &Library, chapter: usize) -> Vec<BlockKind> {
    lib.chapters[chapter]
        .blocks
        .iter()
        .map(|b| b.kind)
        .collect()
}
