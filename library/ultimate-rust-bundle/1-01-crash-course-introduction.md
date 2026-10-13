@@part 1 · Ultimate Rust Crash Course
@@section Module 1: Introduction
@@chapter 1.1.1 | Lesson 1.1 - Introduction | 7
What Rust is, what it promises and what it is not. The one-page answer to "why would a PLC engineer bother?"
@@objectives
- Explain in one sentence what the Rust compiler produces
- Name the class of bugs Rust is built to prevent
- Say what Rust does not replace on your plant
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 1.1 - Introduction · 4:08 — attach the MP4 here when you have it.
@@callout note | Plant situation
Every few weeks the DNC transfer tool on the CNC cell dies at night. It is a ten-year-old C++ program that sends G-code programs from a share to four machines, and when it stops, the 06:00 shift finds a machine waiting with no program. Nobody can reproduce the crash and the log ends in the middle of a line. In the end someone finds a buffer that is written past its end whenever a program name is 64 characters long. In Rust that mistake cannot become a silent crash: the compiler refuses it, or the program stops at that exact line with a clear message.
@@text Teacher's notes
### What Rust is
Rust is a **compiled** language. You write source text in files that end in `.rs`. A program called the compiler, `rustc`, reads them and produces a native executable. On Windows that is a normal `.exe` that the operating system starts like any other program. There is no virtual machine underneath, and no garbage collector that pauses your program from time to time to tidy memory.

### What it promises
A large share of crashes and security holes in C and C++ come from a few memory mistakes: using memory after it was released, writing past the end of a buffer, two threads changing the same data at the same moment. In the normal ("safe") part of Rust, the compiler refuses to build a program that can do these things. You pay for this with a strict compiler. The strictness is the product, and Module 4 is where you meet it face to face.

### What it is not
Rust is not an IEC 61131-3 language and it does not replace the PLC. The PLC keeps running your ladder and structured text on its scan. Rust goes to the places around the PLC where you would otherwise write C#, Python or C++: the line PC, the gateway, the test station, the DNC transfer tool, the protocol converter.

### How the course runs
Nathan Stocks teaches five modules. Each module is a few short videos, usually with exercises on your own computer, and it ends with a review sheet and a quiz. You will type code in almost every module.

### Three things to keep from this lesson
1. Rust compiles to native code. Nothing else has to be installed on the machine that runs the result.
2. There is no garbage collector. The rules of ownership, checked while compiling, decide when memory is released.
3. The compiler is strict, and its messages are written to help you fix the code.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| An application downloaded to the runtime, which executes it | Source built into a standalone `.exe` | No PLC runtime and no scan cycle come with it. The operating system schedules the program |
| Compiler messages in the message window | `rustc` errors with a code and often a suggested fix | Rust also checks who owns and who may change each value |
| An array-bound or division exception stops the runtime | `panic!` stops the program with a message | A panic is a controlled stop, not memory damage |
| A `.library` with POUs | A crate | A crate is compiled together with your code, not loaded at run time |
| Deterministic scan time | Nothing by default | A normal Rust program on Windows is not hard real-time |
@@code mermaid | Diagram: from source text to a running program
flowchart LR
  A["Source code: main.rs"] --> B["rustc checks types and ownership"]
  B -->|errors| C["Read the message and fix the code"]
  C --> A
  B -->|all checks pass| D["Native program: .exe"]
  D --> E["Windows starts it"]
@@code rust | Try it: the smallest program
fn main() {
    let station = "Filler 2";
    let fill_volume_ml = 500;

    println!("Hello from {station}");
    println!("Target fill volume: {fill_volume_ml} ml");
    println!("Two bottles need {} ml", fill_volume_ml * 2);
}
// Output:
// Hello from Filler 2
// Target fill volume: 500 ml
// Two bottles need 1000 ml
@@callout warning | Watch out
`println!` ends with an exclamation mark because it is a **macro**, not a function. If you forget it, the compiler says `error[E0423]: expected function, found macro println` and suggests adding the `!`. The other beginner's classic is the missing semicolon at the end of a line, which gives the message *expected `;`*. Both are one-character fixes, and in both cases the error message already says what to do. Read it before you change anything.
@@text Exercise
Create the program on your own PC and break it twice.

1. Run `cargo new hello_plant` inside `D:\RustroverProjects\learn_rust`, open `src\main.rs` and replace its contents with the Try it code. Change the station name and the volume.
2. Run it with `cargo run`.
3. Delete the `!` after the first `println`, run again and read the message. Put it back.
4. Delete one semicolon, run again and read the message. Put it back.
@@quiz Check yourself
? What does `rustc` produce from your source files?
- Bytecode that a Rust virtual machine interprets
+ A native executable for the operating system
- A package you download into the PLC runtime
= Rust compiles ahead of time to machine code. The result is an ordinary program, with no virtual machine and no PLC runtime under it.
? Which statement about memory in Rust is true?
- A garbage collector runs every second and frees unused values
+ Ownership rules checked at compile time decide when memory is released
- You must call a free function for every value you create
= Rust has no garbage collector and, in safe code, no manual freeing. The compiler tracks who owns each value and releases it at the right moment.
? Where would Rust fit best on your plant?
- Replacing the ladder program inside the PLC
+ In a line-PC program, such as a DNC transfer tool or a gateway
- As a drop-in language for CODESYS function blocks
= Rust does not replace IEC 61131-3 in the PLC runtime. It suits the programs around the PLC that are now written in C#, Python or C++.
@@chapter 1.1.2 | Lesson 1.2 - Exercises Overview | 4
How the course exercises work: one folder per exercise, instructions as numbered comments in `src/main.rs`, and the habit of running `cargo run` after every small change.
@@objectives
- Describe the loop you follow in every exercise
- Find the numbered instructions in an exercise's `src/main.rs`
- Say why you change one step at a time
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 1.2 - Exercises Overview · 1:03 — attach the MP4 here when you have it.
@@callout note | Plant situation
Palletiser 3 changes hands. The engineer who started the project left a note: "Compile errors: 14. Do them in order. Step 1 is the layer counter. Step 2 is the stacker sequence." You do not read all fourteen errors at once. You fix step 1, press Build, see that the list got shorter, and carry on. The Rust exercises work the same way: a half-finished project, a numbered list in the comments, and a compiler that tells you after every step how far you have got.
@@text Teacher's notes
### What an exercise looks like
Every exercise lives in its own folder under `exercise\` in the course repository. Exercises B to K are small Cargo projects. Each has a `src\main.rs` where the instructions are written as **numbered comments** next to the code they are about. Exercise A is the exception: its folder holds only a `README.md` with a task list, and you create the Cargo project yourself.

### The loop
1. Open **one** exercise folder in your editor (not the whole repository).
2. In a terminal inside that folder, run `cargo run`.
3. Read the first error or the output. Find the matching numbered comment.
4. Do step 1: uncomment the code, or write the function, or fix the line.
5. Run again. Repeat with the next step.

Changing one step at a time is the whole trick. When you change five things and the compiler complains, you do not know which change caused it. When you change one thing, you do.

### Try before you peek
Some exercise folders have a challenge at the end of the file for when you finish early. Exercise A also has an `ANSWERS.md`. Try the exercise first, then compare. Reading the answer before you struggle teaches you what the answer looks like; struggling teaches you why.

### How this library helps
Each exercise lesson here repeats the goal in plain words, prints the starter code (with the author's name next to it), gives hints in order, and ends with our own solution. The Ardan video for the exercise is a walk-through. Watch it after you have tried.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| FAT checklist with numbered tests | Numbered comments in `main.rs` | The compiler checks the code for you, a checklist does not |
| A half-finished project to complete | Starter code that does not compile yet | In an exercise the error is deliberate |
| Download and start in simulation | `cargo run` | It builds and runs a normal program, no target needed |
| Expected value on the test sheet | The output printed in the instructions | Nothing compares them for you; you look |
| Master solution kept by the lead | `ANSWERS.md` (Exercise A) and the solutions on our pages | |
@@code mermaid | Diagram: the exercise loop
flowchart LR
  A["Open one exercise folder"] --> B["cargo run"]
  B --> C["Read the first error and the numbered comment"]
  C --> D["Change one step"]
  D --> E["Output as described?"]
  E -->|no| B
  E -->|yes| F["Next step or finished"]
@@code rust | Try it: what an exercise file looks like
// Silence warnings about unused variables so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    // 1. Declare the conveyor speed in mm per second.
    let conveyor_speed_mm_s = 300;

    // 2. Declare the distance between bottle centres in mm.
    let bottle_pitch_mm = 100;

    // 3. Work out how many bottles pass per minute and print the result.
    let bottles_per_s = conveyor_speed_mm_s / bottle_pitch_mm;
    println!("Bottles per minute: {}", bottles_per_s * 60);
}
// Output:
// Bottles per minute: 180
@@callout warning | Watch out
Do not uncomment all the steps in one go. The first `cargo run` of the Exercise B starter already stops with `error[E0308]: mismatched types`, because a decimal number is passed where a whole number is expected. If you also switch on steps 2, 3 and 4, you get a pile of messages and cannot tell which belongs to which. One step, one run. And remember that the first error is the one to read first: later ones are often a result of it.
@@text Exercise
A look-before-you-leap exercise for now.

1. Open a terminal in `D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\b_functions` and run `cargo run`.
2. You do not fix anything yet. Find the line number in the message and look at that line in `src\main.rs`.
3. In My notes, write what the compiler says is wrong, in your own words. You will fix it in chapter 1.2.7.
@@quiz Check yourself
? Where are the instructions for Exercise C?
- In a separate PDF on the Ardan course page
+ In numbered comments inside `src/main.rs` of the exercise folder
- In the Cargo.toml file
= Exercises B to K carry their instructions in the code comments, right next to the lines you will change.
? Why does the course tell you to change one step at a time?
- The compiler only accepts one change per run
+ If something goes wrong, you know which change caused it
- Because `cargo run` is slow
= A small step gives a small error. A big step gives a pile of errors with no clear cause.
@@chapter 1.1.3 | Lesson 1.3 - Installation | 4
Install the Rust toolchain on Windows with rustup, check that `rustc` and `cargo` answer, and keep them current.
@@objectives
- Install Rust with rustup and verify it with `rustc --version` and `cargo --version`
- Explain what rustup, rustc and cargo each do
- Fix the "command not found" problem after an install
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 1.3 - Installation · 1:36 — attach the MP4 here when you have it.
@@callout note | Plant situation
A colleague has to build the DNC tool on the build PC at the plant this afternoon, and the PC has no Rust. In CODESYS you would know the drill: install the exact version the project needs, open the project, build. It is the same drill here, and it takes fifteen minutes. The thing you check at the end is the same as in CODESYS too: the version number. When two people build the same project with different compiler versions, "it works on my PC" starts to appear.
@@text Teacher's notes
### Three programs, one installer
- **`rustup`** is the installer and updater. It downloads the compiler and keeps it current.
- **`rustc`** is the compiler. You rarely call it yourself.
- **`cargo`** is the tool you use all day: it creates projects, builds, runs and fetches libraries. Cargo calls `rustc` for you.

All three end up in `%USERPROFILE%\.cargo\bin`, and the installer adds that folder to your `PATH`.

### Steps on Windows
1. Go to **rust-lang.org**, choose *Get Started* and download the 64-bit `rustup-init.exe`. Install from there, not from some other package manager, which may offer a very old version.
2. Run it. On Windows the default toolchain also needs the Microsoft C++ build tools, because the linker comes from them. If they are missing, the installer tells you. Install the *Desktop development with C++* workload from the Visual Studio Build Tools installer.
3. Accept the default installation.
4. Open a **new** terminal window and run `rustc --version` and `cargo --version`. Both should print a version number.

Your terminal only reads `PATH` when it starts. A window that was open during the install does not know about the new folder. That is the reason for step 4.

### Stable, and updating
`rustup` installs the **stable** toolchain by default. The course and this library use stable. A new stable version comes out every six weeks. Run `rustup update` whenever you like. Rust promises that code written for stable keeps compiling, so your exercises will too.

### Editions
Your `Cargo.toml` has a line `edition = "2021"` or `"2024"`. An edition is a named set of small language rules, chosen per project. The course exercises say 2021. A project made by `cargo new` today gets the newest edition. Both work for everything in this module.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| Install CODESYS and the compiler version | Install with `rustup` | One toolchain is current for all projects unless you pin one |
| Version shown in Help → About | `rustc --version` | |
| Update via the CODESYS Installer | `rustup update` | Updates are free and frequent |
| Target support packages (a device package) | Targets, such as the default `x86_64-pc-windows-msvc` | You only need the default one for this course |
| Library Manager picks library versions | `Cargo.toml` and `Cargo.lock` | Cargo fetches the libraries for you; chapter 1.2.1 explains |
@@code mermaid | Diagram: what the installer puts on your PC
flowchart TD
  U["rustup-init.exe"] --> R["rustup: installer and updater"]
  R --> C["rustc: the compiler"]
  R --> G["cargo: projects, build, run, libraries"]
  G -->|calls| C
  G --> P["PATH gets the .cargo/bin folder"]
@@code rust | Try it: ask the compiler for its version
use std::process::Command;

fn main() {
    // Start `rustc --version` as a child process and capture what it prints.
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("rustc not found on PATH");

    // The bytes it printed, turned into text.
    let text = String::from_utf8_lossy(&output.stdout);
    println!("Installed: {}", text.trim());
}
// Output:
// Installed: rustc 1.96.0 (ac68faa20 2026-05-25)
// (your version number will differ)
@@callout warning | Watch out
After the install, `cargo` or `rustc` says "is not recognized as an internal or external command"? You are almost certainly in a terminal that was open during the install. Close it and open a new one. The second classic on Windows is `error: linker link.exe not found`: the Microsoft C++ build tools are missing. Install the *Desktop development with C++* workload and run your build again. Neither problem is in your Rust code.
@@text Exercise
Run these in a new PowerShell window and write the three version numbers in My notes.
@@code bash | Run it
rustc --version
cargo --version
rustup --version
@@quiz Check yourself
? You installed Rust, but a terminal that was already open says `cargo` is not recognized. What is the likely cause?
- The Rust installer failed silently
- cargo needs a licence key
+ The open terminal read PATH before the install and needs to be restarted
= The installer adds `.cargo\bin` to PATH, and a running terminal does not see the change. A new window fixes it.
? Which tool do you use all day to create, build and run projects?
- rustup
- rustc
+ cargo
= `rustup` installs and updates, `rustc` compiles, and `cargo` is the front door to both.
@@chapter 1.1.4 | Lesson 1.4 - Editor/IDE | 5
Choose an editor with Rust support and learn what to expect from it: errors as you type, inferred types, completion and one-key formatting.
@@objectives
- Choose an editor or IDE and give it Rust and TOML support
- Read an inferred type hint
- Run `cargo fmt` and `cargo clippy`, and say what each does
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 1.4 - Editor/IDE · 2:33 — attach the MP4 here when you have it.
@@callout note | Plant situation
It is 02:10 and a leak tester is rejecting good parts. You open an FB you did not write. In CODESYS the editor helps: hover over a variable and see its type, right-click and find where it is written, see a red underline the moment something does not fit. Without that help you read every line twice. A Rust editor with proper support gives you the same: the type of every variable, a jump to the definition, and errors under the line before you ever press build.
@@text Teacher's notes
### What the editor does for you
Rust has a language server of its own, *rust-analyzer*, and the editors below use it or an equivalent. It reads your project, runs the compiler's checks and shows you:

- **Errors and warnings while you type**, with the compiler's own message.
- **Type hints**. Rust works out many types itself (this is called inference), and the editor writes them in light grey beside the variable, such as `: i32`. Those grey letters are not in your file.
- **Completion** of names, methods and fields, and a jump to the definition.
- **Formatting** with one key.

### Which editor
The course suggests Visual Studio Code with the *rust-analyzer* extension, or an IntelliJ-family IDE. JetBrains RustRover is an IDE built only for Rust. Any of these is a good choice. The point is to pick **one** and give it Rust support, plus TOML support, because `Cargo.toml` is a TOML file. If you use VS Code, keep only one Rust extension installed, rust-analyzer.

### Two tools that arrive with Rust
- **`cargo fmt`** formats all your code in the one standard Rust style, so nobody argues about layout.
- **`cargo clippy`** is the linter. It reads working code and suggests clearer or faster ways to write it. You will meet it again in Part 4.

### Open the folder that has `Cargo.toml`
Open the project folder, the one with `Cargo.toml` directly in it. With an exercise, open the single exercise folder. If you open a parent folder holding many projects, the editor may not know which one you mean.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| CODESYS editor with IntelliSense and red underlines | rust-analyzer or RustRover | The editor combines its own analysis with the output of `cargo check`, so you see the compiler's own messages |
| Hover to see a variable's type | Inferred type hint | Rust declares types less often, so the hint matters more |
| Cross-reference list, "go to definition" | Go to definition | Works across crates, including the standard library |
| Static code analysis | `cargo clippy` | |
| The team's formatting convention | `cargo fmt` | One style for everyone, built in |
@@code mermaid | Diagram: what sits between your keys and the compiler
flowchart LR
  K["You type in the editor"] --> A["Rust support in the editor"]
  A -->|asks| C["rust-analyzer and cargo check"]
  C -->|errors, types, hints| A
  A --> S["Underlines and grey type hints"]
  K --> F["cargo fmt: standard layout"]
@@code rust | Try it: print the types your editor would show in grey
fn main() {
    let fill_volume_ml = 500;
    let pressure_bar = 6.2;
    let line_name = "Filler 2";

    // The compiler worked out every type. Ask it to print them.
    println!("fill_volume_ml: {}", std::any::type_name_of_val(&fill_volume_ml));
    println!("pressure_bar: {}", std::any::type_name_of_val(&pressure_bar));
    println!("line_name: {}", std::any::type_name_of_val(&line_name));
}
// Output:
// fill_volume_ml: i32
// pressure_bar: f64
// line_name: &str
@@callout warning | Watch out
A grey `: i32` in the editor is a hint, not text. If you copy the code into another place, it is not there. And if you write the type yourself where the compiler could have inferred it, nothing breaks. The second trap is fighting with a half-working editor: red marks everywhere on a project that builds fine in the terminal usually mean the editor opened the wrong folder or has two Rust plugins installed.
@@text Exercise
Set up your editor on a real project and look at the hints.

1. Open `D:\RustroverProjects\learn_rust\practice` (made in chapter 0.1.3) or any `cargo new` project in your editor.
2. Put the Try it code into `src\main.rs`. Find the grey type hints beside the three `let` lines.
3. Remove the semicolon after one `println!` line and watch the editor mark it before you build. Put it back.
4. Run `cargo fmt`, then `cargo clippy`, in a terminal inside the project. Read what they say.
@@quiz Check yourself
? What is the grey `: i32` that appears beside `let fill_volume_ml = 500;` in the editor?
- A comment the compiler added to your file
+ A hint showing the type the compiler inferred; it is not in the file
- An error about a missing type
= Rust infers many types. The editor displays them as hints so you can see what the compiler decided.
? What does `cargo fmt` do?
- It finds bugs by running your program
+ It reformats your code in the standard Rust style
- It installs the formatter
= Formatting is built in, and one style for everyone means layout is never up for debate.
@@chapter 1.1.5 | Module 1 - Review | 4
The one-page summary of Module 1: what the course is, how the exercises work, the install and the editor.
@@objectives
- Recall the main idea of each lesson in Module 1
- Check yourself against the cheat sheet before taking the quiz
@@attachment Ardan review sheet (PDF) | Module 1 - Review — download it from the Ardan course page and attach it here.
@@text Cheat sheet
| Topic | One-line meaning | PLC equivalent |
|---|---|---|
| Lesson 1.1 - Introduction | Rust compiles to a native program, has no garbage collector, and its strict compiler prevents memory bugs | The compiler that checks the project before you download it, but stricter |
| Lesson 1.2 - Exercises Overview | One folder per exercise, numbered comments, `cargo run` after every small change | A numbered FAT checklist |
| Lesson 1.3 - Installation | `rustup` installs and updates `rustc` and `cargo`; check with `--version` | Installing the CODESYS version and checking Help → About |
| Lesson 1.4 - Editor/IDE | An editor with Rust support shows errors, types and completions; `cargo fmt` and `cargo clippy` tidy and check | IntelliSense and static code analysis |
@@code mermaid | Module map
flowchart LR
  A["1.1 Introduction: why Rust"] --> B["1.2 Exercises: the loop"]
  B --> C["1.3 Installation: rustup, rustc, cargo"]
  C --> D["1.4 Editor: hints, fmt, clippy"]
  D --> E["Module 2: Fundamentals"]
@@callout tip | Before you take the quiz
- Without looking: what does the compiler produce, and what is missing from that program that you would find in many other languages?
- You open an exercise and the first `cargo run` prints an error. What do you do first?
- A new colleague says `cargo` is not recognized right after install. What do you tell them?
@@chapter 1.1.6 | Module 1 - Quiz | 6
Nine questions on Module 1: Rust itself, the exercises, the install and the editor.
@@objectives
- Check that you can recall the ideas of Module 1 without the notes
@@text How this quiz works
Pick one answer per question. The explanation under each question appears after you answer, whether you were right or not. Read them all.
@@quiz Module 1 - Quiz
? What does the Rust compiler `rustc` produce?
- A file you must load into a PLC runtime
- Bytecode for a Rust virtual machine
+ A native executable for the operating system
= A Rust program is compiled ahead of time to machine code and starts like any other program on Windows.
? What does Rust use instead of a garbage collector?
- A background thread that frees memory at intervals
+ Ownership rules that the compiler checks before the program is built
- A manual `free` call for every value
= Memory is released at a moment the compiler knows in advance, so there are no collector pauses and no manual freeing in safe code.
? `println("Line 2 ready");` does not compile. What is missing?
- A semicolon
- A second parameter
+ The `!`, because `println!` is a macro
= The error is E0423, "expected function, found macro". Write `println!(...)`.
? What is special about Exercise A?
- It is the only one with a video
+ It has no Cargo project: you create it yourself with `cargo new`
- It must be done in the Ardan online editor
= The Exercise A folder holds a README with a task list. The other exercises ship as ready Cargo projects.
? Where do you find the numbered instructions of Exercises B to K?
- In a PDF on the course page
- In the output of `cargo run`
+ In comments in the exercise's `src/main.rs`
= The comments sit next to the code you change, so you see the step and the code together.
? Which command shows the installed compiler version?
- `cargo install rust`
+ `rustc --version`
- `rustup remove`
= `rustc --version` prints the version, and `cargo --version` does the same for Cargo.
? A terminal that was open during the install says `cargo` is not recognized. Why?
- The Rust download was corrupt
- cargo only works after a restart of Windows
+ The terminal read PATH before the installer changed it
= Every new terminal reads the new PATH. Close the old window and open a fresh one.
? Why do you open a single exercise folder in your editor?
- The repository is too large for the editor
+ Each exercise is its own Cargo project and the editor works best with one at a time
- Because the folders are encrypted
= A repository with many separate projects confuses the editor's Rust support.
? What is the light grey `: f64` beside a variable in the editor?
+ The type that the compiler inferred, shown as a hint
- A comment you wrote last week
- A warning that the variable is unused
= Type hints are displayed by the editor and are not part of the file.
