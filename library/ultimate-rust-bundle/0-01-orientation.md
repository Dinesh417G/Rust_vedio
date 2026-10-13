@@part 0 · Start Here
@@section Teacher's Orientation
@@chapter 0.1.1 | How to use this library | 8
What a lesson page contains, how to attach the Ardan videos, where your notes and progress live, and where the exercise code sits on your disk.
@@objectives
- Switch between Learn mode and Author mode and say what each one is for
- Attach an Ardan MP4 to the empty video block of a chapter
- Use My notes, progress marks and Ctrl+K search while studying
- Find the exercise folder for a lesson under `D:\RustroverProjects\learn_rust\ardan\`
@@callout note | Plant situation
Friday, 16:40. The shift handover on the packaging line is at 17:30 and you have one free hour. On Tuesday you learned what a `Vec` is. Today you want to carry on at the next chapter, not scroll through 304 titles to find it. You also remember writing a note about the move rule somewhere, and the video for tonight's lesson is still in your Downloads folder. The library is built for that hour: it remembers where you stopped, keeps your notes under the lesson they belong to, and finds a word when you type it.
@@text Teacher's notes
### Two modes, one app
RustClass has two modes, and you switch with the mode switch in the title bar or with `Ctrl` `E`.

- **Learn** is the reading view. Clean pages, your progress, your notes. Study here.
- **Author** adds the editing tools: the outline, the block palette, drop zones and the inspector. You go there to attach a video file, and for nothing else most days.

### What one lesson page holds
Every video lesson has the same blocks in the same order. After a few lessons your eye knows where to look.

| Block | What it is | Whose it is |
|---|---|---|
| Lesson video | An empty slot with a caption that names the Ardan lesson and its length | Ardan's video, your file |
| Plant situation | A scene on a machine where this exact Rust idea is the answer | Ours |
| Teacher's notes | The idea built up step by step, in plain words | Ours |
| PLC ↔ Rust | A table that also tells you where the comparison stops | Ours |
| Diagram | One picture, one idea | Ours |
| Try it | Code that compiles and runs, with its output shown as a comment | Ours |
| Watch out | The classic mistake and the compiler error that comes with it | Ours |
| Exercise | The matching GitHub exercise, or our own | Nathan Stocks (MIT) or ours |
| Check yourself | A few questions with explanations | Ours |

The order of study inside one lesson matters more than the order of the blocks, and chapter 0.1.3 gives it to you.
@@code mermaid | Diagram: the loop inside one lesson
flowchart LR
  A["Watch the Ardan video"] --> B["Read the teacher's notes"]
  B --> C["Run Try it in your own project"]
  C --> D["Do the exercise"]
  D --> E["Check yourself"]
  E --> F["Mark the chapter complete"]
  B -.->|write what surprised you| N["My notes"]
@@text Attaching the Ardan video
Each video block starts empty. Its caption tells you which lesson belongs in it, for example `Crash Course · Lesson 2.2 - Variables · 5:07`. To fill it:

1. Press `Ctrl` `E` to enter Author mode and open the chapter.
2. The block says **Video not uploaded yet**. Click **Choose video…**, or drop the MP4 file straight onto the block.
3. Press `Ctrl` `E` again to go back to Learn mode and press play.

By default the app **copies** the file into its own media folder and never touches your original. If you would rather keep the file where it is, Settings → Media can switch to reference mode, but then moving the original breaks the link, and the Health page will tell you so. The player handles H.264 video in MP4 and VP8/VP9 in WebM. MKV, HEVC and AV1 usually do not play; the player says so and offers to open the file in your default player.

In the player: `Space` plays and pauses, `J` and `L` jump ten seconds, `Shift` `,` and `Shift` `.` change the speed, `F` is fullscreen. Press `?` anywhere for the whole shortcut sheet.
@@text Notes, progress and search
- **My notes** is at the bottom of every chapter in Learn mode. Notes belong to that chapter, are written in Markdown, and you can click one to edit it. Write the thing that surprised you, in your own words, in your own plant language ("a move is a pallet handed to the next station").
- **Progress**: a video counts as watched when you have played 90 percent of it. At the bottom of the chapter, **Mark chapter complete** finishes it by hand, which is what you use for text pages and exercises. The dashboard has a **Continue where you left off** button.
- **Search**: `Ctrl` `K` searches the whole course. Search for `mut` or `Mutex` or `E0382` and jump straight to the lesson.
- **Next and previous chapter**: `Alt` `→` and `Alt` `←`.
@@text Where the exercises live
All exercise code is cloned to `D:\RustroverProjects\learn_rust\ardan\`. Each lesson page that has an exercise says which folder to open.

| Folder under `ardan\` | Used in | Licence |
|---|---|---|
| `ultimate_rust_crash_course` | Part 1: exercises A to K | MIT, Nathan Stocks |
| `ultimate_rust2` | Part 5: the exercise lessons | MIT, Nathan Stocks |
| `rusty_engine` | Part 5: the game modules | MIT or Apache-2.0 |
| `ArdanUltimateRust-5Days` | Parts 2 and 3 | no licence: we link, we never copy |
| `ArdanRustService` | Part 6 | no licence: we link, we never copy |
| `Ardan-1HourAsync`, `ArdanLabs_RustInYourEnterprise`, `ArdanRustDebugOptimize` | Part 7 webinars (and two chapters of Part 3) | no licence: we link, we never copy |

For the MIT repositories our pages print the starter code with the author's name next to it. For the others, the page tells you the GitHub address, the folder, and what to run and look for. Part 4 has no public exercise repository, so its exercises are ours.

If a folder is missing on another PC, clone it:
@@code bash | Clone the exercise repositories
cd D:\RustroverProjects\learn_rust\ardan
git clone https://github.com/CleanCut/ultimate_rust_crash_course
git clone https://github.com/CleanCut/ultimate_rust2
git clone https://github.com/CleanCut/rusty_engine
git clone https://github.com/thebracket/ArdanUltimateRust-5Days
git clone https://github.com/thebracket/ArdanRustService
git clone https://github.com/thebracket/Ardan-1HourAsync
git clone https://github.com/thebracket/ArdanLabs_RustInYourEnterprise
git clone https://github.com/thebracket/ArdanRustDebugOptimize
@@callout tip | Keep the clones clean
You edit exercise files, and every edit shows up as a change in the clone. Copy the exercise folder you are working on to a folder of your own, for example `D:\RustroverProjects\learn_rust\mine\b_functions`, and work there. The original stays as the starting point, and `git pull` never fights your work.
@@chapter 0.1.2 | The PLC ↔ Rust dictionary | 18
The big reference table: every Rust idea in this bundle next to the PLC idea you already own, with a sentence on what it means and where the comparison breaks.
@@objectives
- Look up a Rust word and find its nearest PLC equivalent
- Say, for each pair, one way in which Rust behaves differently
- Use the dictionary as the first stop when a lesson uses a word you have not met
@@callout note | Plant situation
You are reading a Rust example someone sent you for a data logger on a packaging line. It has `Option`, `Mutex`, a `trait` and a `match`. You can read ladder at speed and you debug a PROFINET fault in ten minutes, but none of these words has a rung in your head yet. You do not need all of them today. You need a place where each word sits next to something you already trust, so that the first time you meet it, it is half familiar. This page is that place.
@@text How to read the dictionary
Each table has four columns. **Rust** is the word. **PLC world** is the nearest thing on your side. **What it means** says in one sentence what the Rust word does. **Where it stops** is the honest part: the point where the comparison stops being true. If you only remember the first two columns you will get fooled sooner or later. The last column is the one that protects you.

The tables are grouped the way a project grows: set it up and build it, give it data, add logic, look after memory, structure it, run things at the same time, and finally ship and maintain it. Come back here whenever a lesson uses a word you have not met.
@@text 1 · Project and build
| Rust | PLC world | What it means | Where it stops |
|---|---|---|---|
| Cargo project, crate | CODESYS project or `.library` | One folder with a `Cargo.toml` and source files. A crate is the unit the compiler builds. | Rust compiles to native code ahead of time. Nothing is downloaded to a runtime. |
| `Cargo.toml` | Project settings and the Library Manager list | Name, version, edition and the list of crates you depend on. | |
| `Cargo.lock` | Exact library versions frozen in the project archive | Records the exact version of every dependency so tomorrow's build matches today's. | |
| crates.io | CODESYS Store, vendor library repository | The public place where crates are published and downloaded. | Anyone can publish. Read what you pull in. |
| `cargo build`, `cargo run` | Build (F11), build plus login plus start | Build compiles. Run compiles if needed and then starts the program. | There is no login to a target. The program starts as a normal program on the PC. |
| binary and library crate | Application (`PLC_PRG`) and library POUs | A binary has `main.rs` and a `main` function. A library has `lib.rs` and offers items to others. | |
| `rustup` | The installer that puts the CODESYS version and its compiler on a PC | Installs and updates the Rust compiler and Cargo. | |
@@text 2 · Data and types
| Rust | PLC world | What it means | Where it stops |
|---|---|---|---|
| `bool`, `u8`, `u16`, `i16`, `i32`, `u32`, `f32`, `f64` | `BOOL`, `BYTE`, `UINT`, `INT`, `DINT`, `UDINT`, `REAL`, `LREAL` | The number types, named by sign and bit width. | Rust never converts between them silently. You write `as` or `From`. A debug build panics on overflow. |
| `let` | A value fixed after its first write, like `VAR CONSTANT` | Binds a name to a value. The binding cannot change. | The restriction is per binding, not per type. |
| `let mut` | Ordinary `VAR` | The same binding, opted in to change. | |
| `const` | `VAR_GLOBAL CONSTANT` | A value known at compile time, with a type you must write. | |
| `static` | `VAR_GLOBAL` | One fixed place in memory for the whole program. | Changing a `static mut` needs `unsafe`. |
| `struct` (data only) | `STRUCT`, a DUT | A named group of fields. | A struct can also get methods. |
| `enum` | State `ENUM` | A value that is exactly one of several named variants. | Each variant can carry its own data. |
| `Option<T>` | A value plus a "signal valid" bit | `Some(value)` or `None`. A wire break gives `None`. | The compiler forces you to deal with `None` before you touch the value. |
| `Result<T, E>` | FB outputs `xDone`, `xError`, `udiErrorId` | `Ok(value)` or `Err(error)`. | The error is a real value you can inspect, and you cannot ignore it by accident. |
| `[T; N]` and `Vec<T>` | `ARRAY[0..N-1] OF T` and a growing array such as a FIFO buffer | A fixed-size array, and a list that can grow. | PLC arrays never grow. |
| `String` and `&str` | A `STRING(80)` buffer you own, and a read-only view of text | An owned, growable text and a borrowed view into text. | Always UTF-8. Length counts bytes, not letters. |
| `HashMap` | A lookup table, recipe name to recipe | Finds a value by key. | The order of entries is not defined. |
| `VecDeque` | FIFO, shift register | A queue you push to at one end and pop from the other. | |
@@text 3 · Logic and control
| Rust | PLC world | What it means | Where it stops |
|---|---|---|---|
| `if` / `else` | `IF` / `ELSE` | Chooses a branch on a `bool`. | It is an expression: it gives back a value. |
| `match` | `CASE state OF` | Picks a branch by the shape of a value. | The compiler forces you to cover every case. |
| `loop`, `while`, `for` | `WHILE`, `REPEAT`, `FOR` | Repeat until a condition or a range ends, or until `break`. | `for` walks an iterator, not only a counter. |
| `fn` | `FUNCTION` | Inputs in, one value out, no memory between calls. | The last expression, written without a semicolon, is the return value. In ST you assign to the function's name. |
| `?` operator | `IF xError THEN` leave and hand the error upward | Returns early from the function with the error if a `Result` is `Err`. | It only works inside a function that itself returns a `Result` or `Option`. |
| closure | A small rule handed over as a parameter, a callback | A function without a name that can use variables around it. | It can capture those variables, which a plain PLC function pointer cannot. |
| iterator chain | A conveyor: `filter` is the reject gate, `map` the machining station, `sum` the counter at the end | A chain of small steps over a sequence of values. | Lazy: nothing moves until the end of the chain pulls. |
| `panic!` | The runtime stops on an exception, such as an array bound or division by zero | Stops the current thread with a message. | In the main thread it ends the program. |
@@text 4 · Memory and ownership
| Rust | PLC world | What it means | Where it stops |
|---|---|---|---|
| scope, block | `VAR_TEMP`, gone when the call ends | A value lives until the closing brace of the block where it was declared. | Scope also decides when the value is cleaned up. |
| ownership | Single-writer rule: one rung owns one coil | Every value has exactly one owner. | The owner can change; see move. |
| move | Handing a pallet to the next station: you no longer have it | Assigning or passing a non-`Copy` value gives it away. | Using the old name afterwards is a compile error (E0382). |
| `clone()` | Copying a recipe data set | Makes an independent copy. | Costs time and memory, and you must ask for it. |
| `Copy` types | `BOOL` or `INT` passed to `VAR_INPUT` by value | Small values that are copied instead of moved. | Only types made of simple copyable parts. |
| `&T` | Read-only `REFERENCE TO`, or `VAR_IN_OUT CONSTANT` | Borrow a value to read it. Many at once are fine. | |
| `&mut T` | `VAR_IN_OUT` with write access | Borrow a value to change it. Only one at a time. | |
| borrow checker | Static analysis that refuses two writers, or reading while writing | The compiler part that enforces ownership and borrowing. | It runs at compile time and costs nothing at run time. |
| lifetimes | A reference must not outlive the instance or `VAR_TEMP` it points to | The compiler's name for "how long this borrow is valid". | Often you never write them; the compiler works them out. |
| stack, heap | The `VAR_TEMP` area and dynamic memory (`__NEW`) | Fast, fixed-size storage for locals, and flexible storage for things that grow. | The heap is normal in Rust and rare in PLCs. |
| `Rc` and `Arc` | One shared recipe with a user counter, the last user releases it | Shared ownership by counting users. `Arc` is the thread-safe one. | `Rc` is for a single thread only. |
| `Drop`, RAII | "On stop, de-energise outputs": cleanup runs by itself | Code that runs automatically when a value goes out of scope. | It does not run if the process is killed, for instance by a power cut. |
| `unsafe` | Forcing an I/O or bridging an interlock: allowed, and you sign for it | A block where you promise the compiler you have checked the rules by hand. | The rest of the program stays checked. |
@@text 5 · Structure and interfaces
| Rust | PLC world | What it means | Where it stops |
|---|---|---|---|
| `mod` | A folder or namespace in the POU tree | A named container for code. | |
| `pub` | Public in the library, otherwise internal | Lets code outside the module use the item. | Everything is private by default. |
| `struct` plus `impl` | A `FUNCTION_BLOCK` type with its variables and `METHOD`s | Data plus the functions that work on it. | FB instances are static. Rust values live on the stack or the heap. |
| `&self`, `&mut self` | A METHOD that reads or writes the FB's own variables | The first parameter of a method: read-only or writable access to the value. | |
| `trait` | An `INTERFACE` such as `I_Motor`, implemented by several FBs | A list of functions a type promises to offer. | You can also give a trait default method bodies. |
| generics | One FB written for `ANY_NUM` | Code written once for many types. | Rust makes a separate copy for each type at compile time. |
| `#[repr(C)]`, packed | The `pack_mode` attribute on fieldbus structs | Tells the compiler to lay out a struct the way C, or a wire format, expects. | |
| FFI | Calling a C library or vendor DLL | Calling functions written in another language. | The call is `unsafe`. |
@@text 6 · Concurrency
| Rust | PLC world | What it means | Where it stops |
|---|---|---|---|
| thread | A PLC task on its own core | An independent flow of execution. | The operating system can pre-empt it at any moment. |
| `Mutex` | A semaphore guarding shared GVL data between tasks | One thread at a time gets at the data. | The lock guards the data itself, not a stretch of code. |
| atomic | A write that can never be seen half-done, like a `LINT` on a 32-bit CPU | A single value that threads can read and write safely without a lock. | Only simple types. |
| `RwLock` | Many HMI readers, one writer | Many readers at the same time, or one writer alone. | |
| channel | A mailbox or message queue between tasks | One thread sends values, another receives them. | |
| thread pool | A bank of identical workstations | A fixed set of worker threads taking jobs from a queue. | Not in the standard library; a crate provides it. |
| async, Tokio | One scan serving many step chains: a step that is waiting just returns | Code that can pause at `.await` and let others run. | The executor that runs it is a library, Tokio, not the language. |
| blocking inside async | A `WHILE` wait loop inside the scan: watchdog | A long wait that never gives control back. | It stalls every other task on that thread. |
| `select!` | Whichever comes first: part sensor or `TON` timeout | Waits on several futures and acts on the first to finish. | It is a macro from Tokio. |
@@text 7 · Services and tooling
| Rust | PLC world | What it means | Where it stops |
|---|---|---|---|
| `#[test]`, `cargo test` | FB unit tests in simulation (CfUnit, TcUnit), FAT | Small functions that check your code and run on demand. | They run on the PC, not on the machine. |
| benchmarks | Measuring scan time | Timing a piece of code over many runs. | |
| tracing, logging | PLC trace and the event or alarm history | Structured messages with levels and timestamps. | |
| axum service | The machine's REST or OPC UA endpoint for MES | A small web server that answers requests. | |
| middleware, layer | The signal-conditioning chain in front of the handler | Steps every request passes before the real work. | |
| configuration | Machine parameters loaded at start-up | Settings read from a file or the environment when the program starts. | |
| gRPC and protobuf | A typed fieldbus with a device description file (GSDML or EDS) | A typed call over the network, defined in a schema file. | |
| WebSocket | Live HMI push instead of polling | The server sends updates when they happen. | |
| Docker | A pre-wired standard cabinet you can ship anywhere | A package that holds the program and what it needs to run. | |
| clippy, rustfmt | Static code analysis, one formatting rule for the team | A linter that suggests improvements, and a tool that formats code one way. | |
| `cargo audit`, `cargo deny` | Checking libraries for CVEs and licences | Scanners for known security issues and licence rules in your dependencies. | Extra tools you install, not part of Cargo itself. |
| game loop (Rusty Engine) | The PLC scan: read inputs, run logic, write outputs, every frame | A function the engine calls again and again. | |
@@quiz Check yourself
? A colleague hands you a `Result<f64, String>` and says "that is your `xError` pattern". What is the part of the PLC picture that Rust adds?
- Rust turns the error into a number that you must clear with a reset button
- Rust runs the function again by itself until it succeeds
+ The error is a value of its own type, and the compiler makes you handle both outcomes before you can use the number
= `Result` carries either the value or the error, and you cannot reach the value without matching on both cases. An `xError` output can be ignored without a word from the compiler.
? Which PLC idea is closest to a Rust `trait`?
- A `VAR_GLOBAL CONSTANT`
+ An `INTERFACE` implemented by several function blocks
- A task configuration
= A trait lists the functions a type promises to offer, as an interface does. Different types can implement it differently.
? Why does this page keep a "Where it stops" column?
- To list Rust features that PLCs do not have
- Because Rust is faster than a PLC
+ Because every analogy is useful only up to a point, and you should know where it fails
= An analogy that you take too far produces wrong beliefs, for example that a Rust value lives in a fixed memory address like a PLC tag.
@@chapter 0.1.3 | Your learning path and study rhythm | 12
The order in which to take the eight parts, a weekly rhythm that fits around plant work, the five-step loop for one lesson, and the cargo habits that make practice cheap.
@@objectives
- Plan the order of the bundle's parts and justify it
- Fit a weekly study rhythm around shift and commissioning work
- Apply the five-step loop to one lesson and name the minutes each step should take
- Set up a practice project in `D:\RustroverProjects\learn_rust` and break it on purpose
@@callout note | Plant situation
Your calendar this month: two commissioning trips, one FAT, and a conveyor retrofit that will run late. You cannot promise anyone a two-hour block each evening, and the last time you started a course you gave up in week three because one missed week made the rest feel like a hill. What works for this kind of calendar is a small, fixed unit of work that you can do in a car park between two calls, and a rule for what to do after a missed week. This page gives you both.
@@text Teacher's notes
### The order of the parts
Ardan numbers the courses 1 to 7. We suggest this order:

1. **Part 1, Crash Course.** The whole language at a walking pace. Nothing else makes sense before it.
2. **Part 2, Foundations.** Threads, async and memory. This is where Rust starts to beat your usual tools, because a PLC engineer has always had to think about tasks and shared data.
3. **Part 3, Next Steps.** Finishes the network collector from Part 2, so take it straight after.
4. **Part 5, Ultimate Rust 2.** Iterators, closures, errors, tests and logging. These are the things you reach for in every real program. Its last three modules build small games with the Rusty Engine; they are fun and they show a game loop, which is a scan cycle in another costume. You can postpone them without losing the thread.
5. **Part 4, Best Practices.** Easier to read once you have written some code that you can improve. It talks about favouring iterators and checking dependencies, and both land better after Part 5.
6. **Part 6, Rust as a Service.** Turns a program into a service that MES, an HMI or another PC can call. It needs everything before it.
7. **Part 7, Webinars.** Talks, taken in any order when you want a second voice.

You do not need to go in this order. If your next job is a data logger with a REST endpoint, you can jump to Part 6 after Part 2 and come back. The Crash Course is the only part that is not optional.

### A rhythm that survives a plant calendar
A realistic week for a working engineer has four small sessions and one longer one.

| When | How long | What |
|---|---|---|
| Three weekday evenings | About 40 minutes each | One video lesson: video, notes, Try it |
| One weekday evening | About 40 minutes | Quiz and review for the module, or catch-up |
| Weekend | About 90 minutes | The exercise lessons of the week, and the module quiz |

In Part 1, plan roughly one module per week. That is a plan, not a promise. When you miss a week, do not try to catch up. Pick up at the next unfinished chapter and shorten nothing else; the **Continue where you left off** button on the dashboard exists for exactly this.

### The five-step loop for one lesson
| Step | What you do | Minutes |
|---|---|---|
| 1. Video | Watch the Ardan lesson once at normal speed, without typing | The video length |
| 2. Notes | Read our teacher's notes and the PLC table. Write one sentence in My notes | 5 to 10 |
| 3. Try it | Type the example into your practice project and run it. Do not paste it | 5 to 10 |
| 4. Exercise | Do the matching exercise. Look at the hints before the solution | 15 to 30 |
| 5. Quiz | Answer the questions, then read every explanation, including the ones you got right | 3 |

Typing the example yourself is not busywork. Your fingers meet the syntax, and the compiler tells you exactly what you got wrong.

### Practising with cargo
Keep one practice project and, later, a folder per experiment. Everything lives under `D:\RustroverProjects\learn_rust`:
@@code bash | Make a practice project
cd D:\RustroverProjects\learn_rust
cargo new practice
cd practice
cargo run
@@text Cargo habits that save time
- `cargo run` builds and starts the program.
- `cargo check` only asks the compiler whether the code is valid. It is faster, and it is what you want while you are fixing errors.
- Read the **first** error first. Later errors are often a result of the first one.
- Each error has a code, like `E0384`. You can look it up with `rustc --explain E0384`, which prints a short explanation with an example.

The best study habit in this library is to **break it on purpose**. Take the Try it code of a lesson, uncomment the line that is marked with the red cross, run it, and read the message. The next time you see it, it is an old friend.
@@code rust | Try it: break it on purpose
fn main() {
    let line_speed_mm_s = 250;
    println!("Line speed: {line_speed_mm_s} mm/s");

    // Uncomment the next line, run `cargo run`, and read the error code.
    // line_speed_mm_s = 300; // ❌ E0384: cannot assign twice to immutable variable
}
// Output:
// Line speed: 250 mm/s
@@callout warning | Watch out
Do not copy-paste the Try it code and call it done. Pasted code runs, so you feel progress, but nothing has passed through your head. The other trap is to keep reading lessons without ever opening a terminal. If a week goes by without one `cargo run`, the week did not count. Make the smallest possible change, run it, and move on.
@@quiz Check yourself
? You missed a week of study because of a commissioning trip. What does this page recommend?
- Double the next week's sessions to catch up
+ Continue at the next unfinished chapter and keep the normal rhythm
- Start Part 1 again from lesson 1
= A plan that punishes a missed week makes you quit. The dashboard remembers where you stopped, and a normal week is the cure.
? What is `cargo check` for?
- It runs the program with extra safety checks
- It checks that the Ardan videos have been attached
+ It asks the compiler whether the code is valid, without producing and starting the program
= `cargo check` stops after type and borrow checking, so it is quicker than a full build. Use `cargo run` when you want to see the program run.
