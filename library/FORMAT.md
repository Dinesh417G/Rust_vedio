# Library content format (`.md` with `@@` directives)

Contract between the content files in `library/<library-name>/` and the
loader in `crates/library`. The loader must accept exactly this, and content
must use nothing else.

## Files

- One library = one folder, e.g. `library/ultimate-rust-bundle/`.
- Every `*.md` file in that folder is loaded in **lexical filename order**.
  Files whose name starts with `_` are ignored (drafts, notes).
- Naming: `<part>-<2-digit section>[letter]-<slug>.md`, e.g.
  `1-02-crash-course-fundamentals.md`, `2-02a-threads.md`, `2-02b-threads.md`.
- Part and section state **resets at every file boundary**: each file must set
  `@@part` and `@@section` before its first `@@chapter`.

## Lines

- A **directive** is a line that starts with `@@` in column 0. Nothing else is
  ever a directive — not inside code fences, not indented.
- Content must never start a line with `@@` for any other purpose.
- Every non-directive line belongs to the **body** of the directive above it.
  Leading and trailing blank lines of a body are trimmed; everything in
  between is kept byte-for-byte (markdown, fences, indentation).
- Directive arguments are split on ` | ` (space, pipe, space). Missing or
  empty fields mean "none". Fields are trimmed.

## Directives

| directive | arguments | body becomes |
|---|---|---|
| `@@course` | `title \| subtitle \| author` | course `description_md` |
| `@@part` | `title` | — (must be empty) |
| `@@section` | `title` | — (must be empty) |
| `@@chapter` | `display number \| title \| estimated minutes` | chapter `summary_md` |
| `@@objectives` | — | current chapter `objectives_md` |
| `@@video` | `heading \| caption` | appended to caption (optional) |
| `@@image` | `heading \| caption` | appended to caption (optional) |
| `@@attachment` | `heading \| caption` | appended to caption (optional) |
| `@@text` | `heading` | block `body_md` |
| `@@callout` | `tone \| heading` | block `body_md`; tone ∈ `note tip warning danger` |
| `@@code` | `lang \| heading \| caption` | block `code_text`; lang ∈ `rust toml bash json text mermaid` |
| `@@quiz` | `heading \| caption` | questions, see below |
| `@@divider` | — | — (must be empty) |

`@@course` may appear once per library, before any `@@part`. Every block
directive (`video` … `divider`) must come after a `@@chapter`; it is added to
that chapter in file order. `@@objectives` may appear once per chapter.

Asset blocks (`video`, `image`, `attachment`) are created **empty** — the
"not uploaded yet" placeholder. Their caption says what file belongs there.

## Quiz body

```
? Which keyword makes a binding changeable?
- const
+ mut
- static
= `mut` opts one binding into change; everything else stays fixed.
? Next question …
```

- `? ` starts a question (the prompt is the rest of the line).
- `- ` is a wrong choice, `+ ` is the right one. **Exactly one `+`** and at
  least two choices per question.
- `= ` lines are the explanation; several `=` lines are joined with a newline.
- Blank lines between questions are allowed. Any other line is an error.
- Stored as the quiz block's `meta_json`:
  `{"questions":[{"id":"q<n>","prompt":…,"choices":[…],"answerIndex":i,"explanation":…}]}`

## Errors and warnings

Hard errors (the whole load is refused, nothing is written): unknown
directive, block before any chapter, non-empty body on `part`/`section`/
`divider`, unknown callout tone, unknown code lang, chapter with no title,
quiz with no questions, a question without exactly one `+` or with fewer than
two choices, a stray line in a quiz body, a second `@@objectives` in one
chapter, a second `@@course`.

Every error names the file and line number.

Warnings (loaded anyway, reported): a chapter with no part or no section,
a duplicate display number inside one part, a chapter with no blocks.

## Example

```
@@part 1 · Ultimate Rust Crash Course
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
```
