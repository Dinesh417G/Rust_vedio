@@part 1 · Ultimate Rust Crash Course
@@section Ultimate Rust Crash Course
@@chapter 1.0.1 | Ultimate Rust Crash Course | 7
The map of Part 1: five short modules and eleven exercises that take you from `cargo new` to structs, traits, collections and enums. Taught by Nathan Stocks.
@@objectives
- Describe what each of the five modules covers, in PLC terms
- Explain how the exercises are organised and why you open one folder at a time
- Clone the exercise repository and run an exercise with `cargo run`
@@callout note | On Ardan this is a text page
Read the course description on Ardan too; this page is our version of the same map, in plant terms.
@@callout note | Plant situation
The bottle filler on line 2 rejects about 3 percent of its bottles, and nobody can say at which hour it gets worse. The shift leader asks for a small program on the line PC: read the hourly reject counts, flag every hour above 2 percent, and never fall over at three in the morning because a file has a blank line in it. For a program that lives on the PC next to the PLC, Rust is a good fit: native speed, and a compiler that forces you to decide what happens with the blank line. Part 1 teaches the small language you need to write that program.
@@text Teacher's notes
### What this course is
The Ultimate Rust Crash Course is Nathan Stocks's introduction to Rust, and he calls it a crash course for a reason: it moves fast and expects you to practise. Every module is a handful of short videos, each followed by an exercise you do on your own computer. You learn by changing code and watching the compiler answer.

### The five modules
1. **Module 1: Introduction.** What the course is, how the exercises work, how to install Rust, which editor to use. Under ten minutes of video in total. This is your commissioning check: tools in place before the real work.
2. **Module 2: Fundamentals.** Cargo, variables, scope, memory safety, functions and the module system. After it you can create a project, declare values and split code over several files.
3. **Module 3: Primitive Types and Control Flow.** The number types, tuples and arrays, `if`, `loop`, `while`, `for`, and text strings.
4. **Module 4: The Heart of Rust.** Ownership and references. This is the one idea that Rust adds to everything you know, so take it slowly.
5. **Module 5: The Meat of Rust.** Structs, traits, collections and enums: your function blocks, interfaces, arrays and state machines.

A review sheet and a quiz close every module, and a final review closes the course.

### The exercises
There are eleven exercises, labelled **A to K**, each in its own folder of the course repository. Exercise A is only a list of tasks in a README file. The others are small Cargo projects whose `src/main.rs` holds numbered instructions in comments. You open the folder of one exercise, run `cargo run`, read the compiler's complaint, and fix the code step by step. There is also a free-form final project, `exercise/z_final_project`, which builds a command-line image tool with the `image` crate. It is optional for us, and fun when you have an evening.

The repository tells you to open **each exercise folder on its own** in your editor. If you open the whole repository, the editor sees several Rust projects at once and complains.

### How we teach it
Each lesson page in this library has the same shape: the Ardan video, then a plant situation, our notes, a PLC table, a diagram, code you can run, the mistake to avoid, the exercise and a short quiz. The Rust you meet in Part 1 is all you need for the first useful program. Part 2 and the rest take it further.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| A commissioning checklist of small tests | Exercises A to K, one concept each | Rust's compiler is the examiner: it fails your code on the spot and tells you why |
| New project from a template | `cargo new` | Cargo also fetches libraries and builds for you |
| Data types, tags and constants | Module 2 and 3: variables, `const`, numbers, strings | Rust variables are fixed after their first write unless you opt in |
| `IF`, `CASE`, `FOR`, `WHILE` | Module 3: control flow | `if` and `match` give back values |
| The single-writer rule, no double coil | Module 4: ownership and borrowing | A PLC checks nothing here; Rust checks it before the program runs |
| `FUNCTION_BLOCK`, `INTERFACE`, `ARRAY`, state `ENUM` | Module 5: structs, traits, collections, enums | Rust values are not static instances; they live on the stack or the heap |
@@code mermaid | Diagram: the roadmap of the whole course
flowchart LR
  M1["Module 1: Introduction"] --> M2["Module 2: Fundamentals"]
  M2 --> M3["Module 3: Types and control flow"]
  M3 --> M4["Module 4: Ownership and references"]
  M4 --> M5["Module 5: Structs, traits, collections, enums"]
  M5 --> R["Full course review and final exam"]
  R --> F["Final project: image tool"]
  M2 --> EA["Exercises A, B, C"]
  M3 --> ED["Exercises D, E, F"]
  M4 --> EG["Exercise G"]
  M5 --> EH["Exercises H, I, J, K"]
@@text Exercises repository
The course exercises are in the repository `CleanCut/ultimate_rust_crash_course` by Nathan Stocks (MIT licence). It is already cloned on this PC at `D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course`. The folders are:

| Folder | Topic | Lesson page |
|---|---|---|
| `exercise/a_variables` | Variables and constants | 1.2.5 |
| `exercise/b_functions` | Functions, shadowing, scope | 1.2.7 |
| `exercise/c_modules` | Library crate and modules | 1.2.9 |
| `exercise/d_simple_types` | Tuples, arrays, number types | 1.3.3 |
| `exercise/e_control_flow` | `loop`, `for`, `while`, `if` | 1.3.5 |
| `exercise/f_strings` | Strings | 1.3.8 |
| `exercise/g_ownership_references` | Ownership and references | 1.4.3 |
| `exercise/h_structs` | Structs | 1.5.2 |
| `exercise/i_traits` | Traits | 1.5.4 |
| `exercise/j_collections` | Collections | 1.5.6 |
| `exercise/k_enums` | Enums | 1.5.8 |
@@code bash | Clone and run an exercise (PowerShell)
# Once, on a new PC:
cd D:\RustroverProjects\learn_rust\ardan
git clone https://github.com/CleanCut/ultimate_rust_crash_course

# Every exercise: open its own folder, then run it
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\b_functions
cargo run
@@callout warning | Watch out
Do not start by opening the whole repository in your editor. It holds a dozen separate Rust projects, and the editor's Rust support gets confused: red marks everywhere, no hints. Open the single exercise folder, for example `exercise\b_functions`, as its own project. Also remember that Exercise A has no Cargo project at all. You create it yourself with `cargo new`. And do not worry when the first `cargo run` prints errors: some starter files are unfinished on purpose, and Exercise B fails until you finish its first step.
@@quiz Check yourself
? You want to do Exercise B. What do you open in your editor?
- The whole `ultimate_rust_crash_course` folder
- The `examples` folder
+ The folder `exercise\b_functions` on its own
= Each exercise is a separate Cargo project. Opening one project at a time keeps the editor's Rust support working.
? Which module introduces ownership and references?
- Module 2: Fundamentals
- Module 3: Primitive Types and Control Flow
+ Module 4: The Heart of Rust
= Ownership is the one idea Rust adds to what you know from other languages, so the course gives it a module of its own.
? What is the first thing to do when an exercise's first `cargo run` shows errors?
- Delete the project and clone it again
+ Read the first error and the numbered instruction in `src/main.rs`; some starter files are unfinished on purpose
- Switch to a different Rust version
= The exercises are built so that you finish them step by step, and the compiler's message tells you what to do next.
