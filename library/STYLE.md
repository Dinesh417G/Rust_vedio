# Teaching style guide — Ultimate Rust Bundle library

Every content file follows this guide. The structure comes from
`CURRICULUM.md`, the syntax from `FORMAT.md`, the exercise links from
`EXERCISES.md`.

## Who is reading

One learner: an automation engineer who builds PLC systems every day —
CODESYS and Structured Text, ladder, HMIs, Modbus/PROFINET/EtherCAT, OPC UA,
SCADA, CNC/DNC file transfer, leak testers and conveyors. New to Rust. Reads
English well, prefers short, plain sentences.

You are a teacher whose only job is that he *understands*. Start from what he
already knows on the plant floor, then move to Rust. Put him in a realistic
situation on every page. No hype, no filler, no "in this lesson we will".

## Topic page — fixed block order

Approved by the owner. Every **video lesson** page has these blocks, in this
order, with these exact headings:

| # | directive and heading | what goes in | size |
|---|---|---|---|
| 1 | `@@video Lesson video \| <caption>` | caption format below | 1 line |
| 2 | `@@callout note \| Plant situation` | a concrete scene on a machine or line where this exact Rust idea is the answer | 60–120 words |
| 3 | `@@text Teacher's notes` | the concept in plain words, built up step by step; `###` sub-headings when long; inline `code` | 250–450 words (long lessons up to 700) |
| 4 | `@@text PLC ↔ Rust` | a 4–7 row table: `PLC world` \| `Rust` \| `Where the comparison stops` | table only |
| 5 | `@@code mermaid \| Diagram: <what it shows>` | one flowchart / sequence / state / class diagram | ≤ 15 nodes |
| 6 | `@@code rust \| Try it: <what it does>` | runnable example with plant names and an `// Output:` comment | ≤ 40 lines |
| 7 | `@@callout warning \| Watch out` | the mistake people make, ideally with the real compiler error code (E0382 …) and the fix | 40–100 words |
| 8 | exercise block(s) — see "Exercises" | from `EXERCISES.md` | — |
| 9 | `@@quiz Check yourself` | 2–3 original questions | — |

Chapter header for every page:

```
@@chapter <display> | <exact Ardan title> | <minutes>
<1–2 sentence summary: what this topic is and why it matters on a machine>
@@objectives
- <2–4 bullets, each starting with a verb: Explain / Write / Choose / Fix / Read>
```

Minutes = video minutes rounded up + 1 min per 200 words of notes, + 15 for
an exercise lesson.

### Other page types

- **Exercise lesson** (title contains "Exercise"): blocks 1, 2, then
  `@@text Exercise brief` (goal in our own words, numbered steps),
  the exercise blocks, `@@callout tip | Hints` (2–4 progressive hints),
  `@@code rust | Solution — try it yourself first` (our own solution),
  `@@callout warning | Watch out`, `@@quiz Check yourself`.
  Diagram and PLC table are optional here.
- **Ardan text page** (type `Text`): no video block. Start with
  `@@callout note | On Ardan this is a text page` (one line: read it there),
  then blocks 2–9 as for a lesson, scaled to the topic (a course intro gets a
  roadmap diagram of the whole course instead of a code sample).
- **Review** (type `PDF`):
  `@@attachment Ardan review sheet (PDF) | <title> — download it from the Ardan course page and attach it here.`
  then `@@text Cheat sheet` (one table row per topic in the module:
  topic · one-line meaning · PLC equivalent), `@@code mermaid | Module map`,
  `@@callout tip | Before you take the quiz` (3 self-check prompts).
- **Quiz** (type `Quiz`): `@@text How this quiz works` (2 lines) and
  `@@quiz <title>` with 8–10 questions. A **Final Exam** has 15–20 questions
  split over 2–3 `@@quiz` blocks by module.
- **Own** pages (part 0): free form, same voice.

## Video caption

```
@@video Lesson video | **Ardan Labs** · <course short name> · <exact title> · <m:ss> — attach the MP4 here when you have it.
```

Course short names: `Crash Course`, `Rust Foundations`, `Foundations: Next
Steps`, `Best Practices`, `Ultimate Rust 2`, `Rust as a Service`, `Webinar`.

## PLC analogy bank — use these, consistently

| Rust | PLC side | where it stops |
|---|---|---|
| Cargo project / crate | CODESYS project or `.library` | Rust compiles to native code ahead of time |
| `Cargo.toml` | project settings + Library Manager list | |
| `Cargo.lock` | exact library versions frozen in the project archive | |
| crates.io | CODESYS Store / vendor library repository | anyone can publish |
| `cargo build` / `cargo run` | Build (F11) / build + login + start | |
| binary vs library crate | application (`PLC_PRG`) vs library POUs | |
| `mod` | folder / namespace in the POU tree | |
| `pub` | public in the library, otherwise internal | |
| `let` | a value fixed after first write (like `VAR CONSTANT`, but per binding) | |
| `let mut` | ordinary `VAR` | |
| `const` | `VAR_GLOBAL CONSTANT` | |
| `static` | `VAR_GLOBAL` | `static mut` is `unsafe` |
| scope / block | `VAR_TEMP`: gone when the call ends | |
| `fn` | `FUNCTION`: inputs in, return value out, no memory | |
| `struct` + `impl` | `FUNCTION_BLOCK` type with its variables and `METHOD`s | FB instances are static; Rust values live on stack or heap |
| `&self` / `&mut self` | a METHOD that reads / writes the FB's own variables | |
| trait | `INTERFACE` (e.g. `I_Motor`) implemented by several FBs | |
| generics | one FB written for `ANY_NUM` | Rust stamps out a copy per type |
| `enum` | state `ENUM` — but every state can carry its own data | |
| `match` | `CASE state OF` — and the compiler forces every state to be handled | |
| `Option<T>` | value + "signal valid" bit (wire break → no value) | |
| `Result<T, E>` | FB outputs `xDone` / `xError` / `udiErrorId` | |
| `panic!` | the runtime stops on an exception (array bound, divide by zero) | |
| ownership | single-writer rule: one rung owns one coil (no double coil) | |
| move | handing a pallet to the next station — you no longer have it | |
| `clone()` | copying a recipe data set | costs time and memory |
| `Copy` types | `BOOL`/`INT` passed to `VAR_INPUT` by value | |
| `&T` | read-only `REFERENCE TO` / `VAR_IN_OUT CONSTANT` | |
| `&mut T` | `VAR_IN_OUT` with write access — only one writer at a time | |
| borrow checker | static analysis that refuses two writers, or reading while writing | runs at compile time, costs nothing at run time |
| lifetimes | a reference must not outlive the instance or `VAR_TEMP` it points at | |
| stack / heap | `VAR_TEMP` area / dynamic memory (`__NEW`) | heap is normal in Rust, rare in PLCs |
| `[T; N]` / `Vec<T>` | `ARRAY[0..N-1] OF T` / an array that can grow (FIFO buffer) | PLC arrays never grow |
| `HashMap` | lookup table: recipe name → recipe | |
| `VecDeque` | FIFO / shift register | |
| `String` / `&str` | a `STRING(80)` buffer you own / a read-only view of text | UTF-8, length in bytes |
| iterator chain | a conveyor: `filter` = reject gate, `map` = machining station, `sum` = counter at the end | lazy: nothing moves until the end pulls |
| closure | a small rule handed over as a parameter (callback) | can capture variables |
| thread | a PLC task on its own core | the OS can pre-empt it any time |
| `Mutex` | semaphore guarding shared GVL data between tasks | |
| atomic | a write that can never be torn (DINT on a 32-bit bus) | |
| `RwLock` | many HMI readers, one writer | |
| channel | mailbox / message queue between tasks | |
| thread pool | a bank of identical workstations | |
| async / Tokio | one scan serving many step chains: a step that is waiting just returns | the executor is the runtime |
| blocking inside async | a `WHILE` wait loop inside the scan → watchdog | |
| `select!` | whichever comes first: part sensor or `TON` timeout | |
| `Drop` / RAII | "on stop, de-energise outputs" — cleanup runs by itself | |
| `Rc` / `Arc` | one shared recipe with a user counter; the last user releases it | |
| `unsafe` | forcing I/O / bridging an interlock: allowed, you sign for it | |
| `#[repr(C)]` / packed | `pack_mode` attribute on fieldbus structs | |
| FFI | calling a C library or vendor DLL | |
| tracing / logging | PLC trace + event logger / alarm history | |
| tests | FB unit tests in simulation (CfUnit/TcUnit), FAT | |
| benchmarks | measuring scan time | |
| axum service | the machine's REST / OPC UA endpoint for MES | |
| middleware / layer | the signal-conditioning chain in front of the handler | |
| configuration | machine parameters loaded at start-up | |
| gRPC + protobuf | a typed fieldbus with a device description file (GSDML/EDS) | |
| WebSocket | live HMI push instead of polling | |
| Docker | a pre-wired standard cabinet you can ship anywhere | |
| clippy / rustfmt | static code analysis / one formatting rule for the team | |
| cargo audit / deny | checking libraries for CVEs and licences | |
| game loop (Rusty Engine) | the PLC scan: read inputs → logic → write outputs, every frame | |

Scenes to draw from: bottle filler, conveyor sorter, palletiser, leak tester
(pressure-decay test), CNC DNC program transfer, packaging line HMI, boiler
alarm, batch recipe. Keep numbers realistic (ms, bar, mm, rpm).

## Diagrams (`@@code mermaid`)

- Allowed: `flowchart LR|TD`, `sequenceDiagram`, `stateDiagram-v2`,
  `classDiagram`.
- Quote every node label: `A["Read inputs"]`. Inside labels use only letters,
  digits, spaces and `. , : ' - / + = ( ) ?` — never `< > & " ; #` or braces.
- Edge labels: `A -->|plain words| B`, same character rule.
- State diagrams: single-word state names, transitions `Idle --> Running: start`.
- At most 15 nodes. One idea per diagram.

## Code (`@@code rust`)

- Compiles on stable Rust, edition 2021, **std only** — unless the topic is
  about a crate; then the first line is a comment naming it:
  `// Cargo.toml: tokio = { version = "1", features = ["full"] }`.
- Has `fn main()`, prints something, ends with an `// Output:` comment.
- Plant names: `fill_volume_ml`, `conveyor_speed_mm_s`, `MotorState`,
  `leak_rate_pa_s`, `recipe`, `station`.
- Showing a compile error: keep the program compiling and put the failing line
  in a comment, e.g. `// let y = x; // ❌ E0382: borrow of moved value`.
- A block whose first line is `// no-check` is skipped by the snippet checker —
  use it only for fragments that cannot stand alone (e.g. a single `impl`).
- Course code needing a crate version: match the course repo (axum 0.7 uses
  `/:id`, axum 0.8 uses `/{id}` — say which).

## Exercises

From `EXERCISES.md`:

- **MIT repos** (Nathan Stocks): embed the starter verbatim in
  `@@code rust | Starter code — <repo path> (MIT © Nathan Stocks)` and give
  the local path and the run command in `@@code bash | Run it`.
- **Unlicensed repos** (Herbert Wolverson): never copy their code or text.
  Use `@@callout note | Course code on GitHub` with the GitHub URL, the local
  path under `D:\RustroverProjects\learn_rust\ardan\`, which folder or file to
  open, and what to run and look for — in our own words.
- Topics with no repo code get our own short exercise in `@@text Exercise`.

## Quizzes

Original questions only — never Ardan's. Plausible wrong choices, one right
one, and an `=` explanation for every question. Test understanding, not
trivia.

## Hard rules

- Never copy Ardan's videos, slides, PDFs, transcripts or quiz questions.
- Never write Ardan file URLs (PDF CDN links) — this repo is public.
- Never invent APIs. If unsure a function exists in the version you name,
  leave it out.
- Every `@@code rust` block without `// no-check` or a `// Cargo.toml:` line
  must pass the snippet checker (`cargo run -p rustclass-library -- check`).
