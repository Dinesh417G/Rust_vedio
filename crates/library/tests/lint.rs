//! Page-structure lint against STYLE.md's topic-page block order.

use rustclass_library::coverage::parse_curriculum;
use rustclass_library::lint::{lint, LintIssue};
use rustclass_library::parse_files;

const CURRICULUM: &str = "\
`@@part 1 · Lint`

### Section 1 — `Module 1`
| display | title | type | video |
|---|---|---|---|
| 1.1.1 | Lesson 1.1 - Variables | Video | 5:07 |
| 1.1.2 | Lesson 1.2 - Exercise A - Variables | Video | 4:56 |
| 1.1.3 | Intro to the module | Text | — |
| 1.1.4 | Module review | PDF | — |
| 1.1.5 | Quiz - Module | Quiz | — |
| 1.1.6 | Final Exam - Module | Quiz | — |
| 1.1.7 | Our own page | Own | — |
| 1.1.8 | Lesson 1.8 - Exercises Overview | Video | 5:07 |
";

fn notes(words: usize) -> String {
    "word ".repeat(words)
}

fn quiz(title: &str, questions: usize) -> String {
    let mut out = format!("@@quiz {title}\n");
    for i in 0..questions {
        out.push_str(&format!("? Question {i}?\n- no\n+ yes\n"));
    }
    out
}

const VIDEO: &str =
    "@@video Lesson video | **Ardan Labs** · Crash Course · T · 5:07 — attach it.\n";
const SITUATION: &str = "@@callout note | Plant situation\nA bottle filler.\n";
const TABLE: &str = "| PLC world | Rust | Stops |\n|---|---|---|\n| TON | timer | no |\n";
const DIAGRAM: &str = "@@code mermaid | Diagram: flow\nflowchart LR\n  A --> B\n";
const TRY_IT: &str = "@@code rust | Try it: print\nfn main() {}\n";
const WATCH_OUT: &str = "@@callout warning | Watch out\nE0382.\n";

/// A correct video lesson, with the pieces the tests break one by one.
fn lesson(header: &str, notes_words: usize, order: &[&str]) -> String {
    let mut out = format!("{header}\nSummary.\n");
    for piece in order {
        match *piece {
            "video" => out.push_str(VIDEO),
            "situation" => out.push_str(SITUATION),
            "notes" => out.push_str(&format!("@@text Teacher's notes\n{}\n", notes(notes_words))),
            "plc" => out.push_str(&format!("@@text PLC ↔ Rust\n{TABLE}")),
            "plc-no-table" => out.push_str("@@text PLC ↔ Rust\nTimers are timers.\n"),
            "diagram" => out.push_str(DIAGRAM),
            "try" => out.push_str(TRY_IT),
            "watch" => out.push_str(WATCH_OUT),
            "exercise" => out.push_str("@@attachment Exercise files | Download.\n"),
            "quiz" => out.push_str(&quiz("Check yourself", 2)),
            "quiz4" => out.push_str(&quiz("Check yourself", 4)),
            other => panic!("unknown piece {other}"),
        }
    }
    out
}

const FULL: [&str; 8] = [
    "video",
    "situation",
    "notes",
    "plc",
    "diagram",
    "try",
    "watch",
    "quiz",
];

fn issues_for(content: &str) -> Vec<LintIssue> {
    let lib = parse_files(&[(
        "c.md".to_string(),
        format!("@@part P\n@@section S\n{content}"),
    )])
    .unwrap_or_else(|e| panic!("library should parse: {e:?}"));
    lint(&parse_curriculum(CURRICULUM).expect("curriculum"), &lib)
}

fn problems(content: &str) -> Vec<String> {
    issues_for(content)
        .iter()
        .map(ToString::to_string)
        .collect()
}

fn one(content: &str) -> String {
    let all = problems(content);
    assert_eq!(all.len(), 1, "{all:#?}");
    all.into_iter().next().unwrap()
}

#[test]
fn lint_flags_missing_and_misordered_blocks() {
    let h = "@@chapter 1.1.1 | Lesson 1.1 - Variables | 12";

    // (a) a correct lesson, with exercise blocks before the quiz, is clean.
    assert_eq!(problems(&lesson(h, 210, &FULL)), Vec::<String>::new());
    let mut with_exercise = FULL.to_vec();
    with_exercise.insert(7, "exercise");
    assert_eq!(
        problems(&lesson(h, 210, &with_exercise)),
        Vec::<String>::new()
    );

    // Each missing piece is named, as `display title: problem`.
    let mut no_diagram = FULL.to_vec();
    no_diagram.remove(4);
    assert_eq!(
        one(&lesson(h, 210, &no_diagram)),
        "1.1.1 Lesson 1.1 - Variables: missing code(mermaid) starting \"Diagram:\""
    );
    let mut no_watch = FULL.to_vec();
    no_watch.remove(6);
    assert_eq!(
        one(&lesson(h, 210, &no_watch)),
        "1.1.1 Lesson 1.1 - Variables: missing callout(warning) \"Watch out\""
    );
    let mut no_quiz = FULL.to_vec();
    no_quiz.pop();
    assert_eq!(
        one(&lesson(h, 210, &no_quiz)),
        "1.1.1 Lesson 1.1 - Variables: missing quiz \"Check yourself\""
    );

    // Misordered: the situation callout after the notes.
    let swapped = [
        "video",
        "notes",
        "situation",
        "plc",
        "diagram",
        "try",
        "watch",
        "quiz",
    ];
    assert_eq!(
        one(&lesson(h, 210, &swapped)),
        "1.1.1 Lesson 1.1 - Variables: text \"Teacher's notes\" comes before \
         callout(note) \"Plant situation\", expected after it"
    );
    // Nothing before the video: the video is block 2, not block 1.
    let mut late_video = vec!["situation", "video"];
    late_video.extend(&FULL[2..]);
    assert!(
        problems(&lesson(h, 210, &late_video))
            .iter()
            .any(|p| p.contains("comes before")),
        "{late_video:?}"
    );
    // An extra block between the fixed ones pushes the rest out of place.
    let mut shoved = FULL.to_vec();
    shoved.insert(1, "exercise");
    assert_eq!(
        one(&lesson(h, 210, &shoved)),
        "1.1.1 Lesson 1.1 - Variables: callout(note) \"Plant situation\" is block 3, expected block 2"
    );
    // The quiz has to be last.
    let mut quiz_early = FULL.to_vec();
    quiz_early.push("exercise");
    assert_eq!(
        one(&lesson(h, 210, &quiz_early)),
        "1.1.1 Lesson 1.1 - Variables: quiz \"Check yourself\" must be the last block"
    );

    // Size and content rules.
    assert_eq!(
        one(&lesson(h, 150, &FULL)),
        "1.1.1 Lesson 1.1 - Variables: text \"Teacher's notes\" has 150 words, needs at least 200"
    );
    let mut no_table = FULL.to_vec();
    no_table[3] = "plc-no-table";
    assert_eq!(
        one(&lesson(h, 210, &no_table)),
        "1.1.1 Lesson 1.1 - Variables: text \"PLC ↔ Rust\" has no table"
    );
    let mut four = FULL.to_vec();
    four[7] = "quiz4";
    assert_eq!(
        one(&lesson(h, 210, &four)),
        "1.1.1 Lesson 1.1 - Variables: quiz \"Check yourself\" has 4 questions, needs 2 or 3"
    );

    // (f) the caption must carry the curriculum's video length.
    let wrong_length = lesson(h, 210, &FULL).replace("5:07", "9:99");
    assert_eq!(
        one(&wrong_length),
        "1.1.1 Lesson 1.1 - Variables: the video caption does not contain the length 5:07"
    );

    // Wrong tone on the warning callout counts as missing.
    let wrong_tone = lesson(h, 210, &FULL).replace("callout warning", "callout tip");
    assert_eq!(
        one(&wrong_tone),
        "1.1.1 Lesson 1.1 - Variables: missing callout(warning) \"Watch out\""
    );
}

#[test]
fn lint_checks_exercise_text_pdf_and_quiz_pages() {
    // (b) exercise lesson.
    let h = "@@chapter 1.1.2 | Lesson 1.2 - Exercise A - Variables | 20";
    let video_4_56 = VIDEO.replace("5:07", "4:56");
    let exercise = format!(
        "{h}\nS.\n{video_4_56}{SITUATION}@@text Exercise brief\n1. Do it.\n\
         @@attachment Starter | Files.\n@@callout tip | Hints\nHint.\n\
         @@code rust | Solution — try it yourself first\nfn main() {{}}\n{WATCH_OUT}{}",
        quiz("Check yourself", 3)
    );
    assert_eq!(problems(&exercise), Vec::<String>::new());
    let no_hints = exercise.replace("@@callout tip | Hints\nHint.\n", "");
    assert_eq!(
        one(&no_hints),
        "1.1.2 Lesson 1.2 - Exercise A - Variables: missing callout(tip) \"Hints\""
    );
    let late_brief = format!(
        "{h}\nS.\n{video_4_56}{SITUATION}@@callout tip | Hints\nHint.\n@@text Exercise brief\nDo.\n\
         @@code rust | Solution\nfn main() {{}}\n{WATCH_OUT}{}",
        quiz("Check yourself", 2)
    );
    assert_eq!(
        one(&late_brief),
        "1.1.2 Lesson 1.2 - Exercise A - Variables: callout(tip) \"Hints\" comes before \
         text \"Exercise brief\", expected after it"
    );

    // (c) text page.
    let h = "@@chapter 1.1.3 | Intro to the module | 5";
    let text_page = format!(
        "{h}\nS.\n@@callout note | On Ardan this is a text page\nRead it there.\n\
         @@text Teacher's notes\nNotes.\n{}",
        quiz("Check yourself", 2)
    );
    assert_eq!(problems(&text_page), Vec::<String>::new());
    let bad_text = format!("{h}\nS.\n@@text Teacher's notes\nNotes.\n");
    let got = problems(&bad_text);
    assert_eq!(got.len(), 2, "{got:#?}");
    assert!(
        got[0].ends_with("the first block must be callout(note) \"On Ardan this is a text page\"")
    );
    assert!(got[1].ends_with("missing quiz"));

    // (d) PDF page.
    let h = "@@chapter 1.1.4 | Module review | 5";
    let pdf = format!(
        "{h}\nS.\n@@attachment Ardan review sheet (PDF) | Download.\n\
         @@text Cheat sheet\n{TABLE}@@code mermaid | Module map\nflowchart LR\n  A --> B\n"
    );
    assert_eq!(problems(&pdf), Vec::<String>::new());
    let bad_pdf = format!(
        "{h}\nS.\n@@text Cheat sheet\nno table here\n@@code mermaid | Module map\nflowchart LR\n  A --> B\n"
    );
    let got = problems(&bad_pdf);
    assert_eq!(got.len(), 2, "{got:#?}");
    assert!(got[0].ends_with("the first block must be attachment"));
    assert!(got[1].ends_with("text \"Cheat sheet\" has no table"));

    // (e) quiz pages: 8 questions, 15 for a final exam.
    let q = |h: &str, n: usize| {
        format!(
            "{h}\nS.\n@@text How this quiz works\nTwo lines.\n{}",
            quiz("Quiz", n)
        )
    };
    assert_eq!(
        problems(&q("@@chapter 1.1.5 | Quiz - Module | 5", 8)),
        Vec::<String>::new()
    );
    assert_eq!(
        one(&q("@@chapter 1.1.5 | Quiz - Module | 5", 7)),
        "1.1.5 Quiz - Module: the page has 7 quiz questions, needs at least 8"
    );
    assert_eq!(
        one(&q("@@chapter 1.1.6 | Final Exam - Module | 5", 14)),
        "1.1.6 Final Exam - Module: the final exam has 14 quiz questions, needs at least 15"
    );
    assert_eq!(
        problems(&q("@@chapter 1.1.6 | Final Exam - Module | 5", 15)),
        Vec::<String>::new()
    );
    assert_eq!(
        one("@@chapter 1.1.5 | Quiz - Module | 5\nS.\n@@text Only text\nx\n"),
        "1.1.5 Quiz - Module: missing a quiz block"
    );
}

#[test]
fn lint_skips_own_pages_and_flags_unknown_chapters() {
    assert_eq!(
        problems("@@chapter 1.1.7 | Our own page | 5\nS.\n@@text Anything\nx\n"),
        Vec::<String>::new()
    );
    assert_eq!(
        one("@@chapter 9.9.9 | Stray | 5\nS.\n@@text Anything\nx\n"),
        "9.9.9 Stray: not in the curriculum, so its page type is unknown"
    );
    let issues = issues_for("@@chapter | No number | 5\nS.\n@@text Anything\nx\n");
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].display, "(no number)");
    assert_eq!((issues[0].file.as_str(), issues[0].line), ("c.md", 3));
}

#[test]
fn exercises_overview_is_linted_as_a_normal_lesson() {
    let h = "@@chapter 1.1.8 | Lesson 1.8 - Exercises Overview | 3";
    assert_eq!(problems(&lesson(h, 210, &FULL)), Vec::<String>::new());
    // The exercise-page shape is not accepted for it.
    let exercise_shape = format!(
        "{h}
Summary.
{VIDEO}{SITUATION}@@text Exercise brief
Do it.
         @@callout tip | Hints
Look.
@@code rust | Solution - mine
fn main() {{}}
         {WATCH_OUT}{}",
        quiz("Check yourself", 2)
    );
    assert!(!problems(&exercise_shape).is_empty());
}
