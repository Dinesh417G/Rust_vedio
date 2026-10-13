@@part 1 · Ultimate Rust Crash Course
@@section Module 2: Fundamentals
@@chapter 1.2.1 | Lesson 2.1 - Cargo | 9
Cargo creates, builds and runs Rust projects and fetches the libraries they need. Learn the project layout, `Cargo.toml`, `Cargo.lock` and the commands you will type every day.
@@objectives
- Create a project with `cargo new` and say what each generated file is for
- Choose between `cargo check`, `cargo run` and `cargo build --release`
- Add a dependency to `Cargo.toml` and explain what `Cargo.lock` records
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.1 - Cargo · 6:55 — attach the MP4 here when you have it.
@@callout note | Plant situation
The DNC tool was built in March and sent programs to the milling cell without trouble. In October a colleague rebuilds it on the build PC and the transfers get slower and one status message looks different. Nobody changed a line of code. A library the tool depends on had released a new version in between, and the colleague's build picked it up. In a CODESYS project that cannot happen, because the Library Manager holds the exact versions inside the project. Cargo gives you the same guarantee with a file called `Cargo.lock`.
@@text Teacher's notes
### One tool for the whole project
Cargo is Rust's build tool and package manager in one. It creates the project, calls the compiler, downloads libraries from crates.io and runs your program. You will use it dozens of times a day.

### Creating a project
`cargo new bottle_filler` makes a folder with this content:

- `Cargo.toml`: the project settings. Name, version, edition and the list of libraries (called crates) you depend on.
- `src/main.rs`: your code, with a `main` function that already prints "Hello, world!".
- `.gitignore` and a git repository, unless you are already inside one.

After the first build, two more things appear: `Cargo.lock` and the `target` folder.

### The commands
- `cargo run` builds the program if needed and starts it.
- `cargo build` builds without starting.
- `cargo check` only asks the compiler whether the code is valid. It is the fastest, and it is what you use while fixing errors.
- `cargo build --release` builds the optimised version. Plain `cargo build` makes the **debug** version: quick to compile, slow to run, with extra checks for you. The release version is the one you copy to the line PC.

Debug output lands in `target\debug`, release output in `target\release`. The `.exe` in the release folder is the finished program.

### Reading Cargo's output
A run prints `Compiling bottle_filler v0.3.0` (your project name and the version from `Cargo.toml`), then `Finished`, then `Running` with the path of the `.exe`. If nothing changed since the last build, `Compiling` is skipped and the program starts at once. `cargo clean` deletes the `target` folder when you want to start from nothing.

### Dependencies
Libraries live on **crates.io**. To use one you name it in the `[dependencies]` section of `Cargo.toml`, for example `rand = "0.8"` (the course's `hello` example does exactly that). Or you run `cargo add rand` and Cargo writes the line for you. The next build downloads it and everything it needs.

### Cargo.lock: the frozen versions
`"0.8"` means "any compatible 0.8 version". The first build picks the newest one and writes the exact result into `Cargo.lock`. Every build after that uses those exact versions until you run `cargo update`. Commit `Cargo.lock` with your program so that a rebuild in October matches the build of March.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| CODESYS project | Cargo project (a package) | Rust compiles to native code ahead of time |
| Project settings and Library Manager list | `Cargo.toml` | The list is plain text you can edit and diff |
| Exact library versions frozen in the project archive | `Cargo.lock` | It is generated for you; you do not edit it |
| CODESYS Store, vendor library repository | crates.io | Anyone can publish a crate, so check what you pull in |
| Build (F11) | `cargo build` | |
| Build, login and start | `cargo run` | There is no login: the program starts on this PC |
@@code mermaid | Diagram: what is inside a Cargo project
flowchart TD
  P["bottle_filler/"] --> T["Cargo.toml: name, version, dependencies"]
  P --> L["Cargo.lock: exact versions"]
  P --> S["src/main.rs: your code"]
  P --> B["target/: build output"]
  B --> D["debug/: cargo build and cargo run"]
  B --> R["release/: cargo build --release"]
@@code rust | Try it: a program that knows its own name and version
fn main() {
    // Cargo tells the compiler the package name and version it read from Cargo.toml.
    // `option_env!` gives None instead of an error when the program is not built by Cargo.
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("not built by cargo");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("?");
    println!("{name} version {version}");
}
// Output:
// bottle_filler version 0.3.0
// (for a Cargo project named bottle_filler whose Cargo.toml says version = "0.3.0")
@@callout warning | Watch out
Two classics. First, `cargo run` in the wrong folder: `error: could not find Cargo.toml in ... or any parent directory`. Change into the project folder, the one that contains `Cargo.toml`. Second, timing a program built with plain `cargo run`. That is the **debug** build, and it can be many times slower than the release build, so a speed test on it tells you little. Measure with `cargo run --release`.
@@text Exercise
Make a project, change it, and watch Cargo's output change with it.
@@code bash | Run it
cd D:\RustroverProjects\learn_rust
cargo new message
cd message
cargo run
# Now edit src\main.rs and change the message, then run again:
cargo run
# Open Cargo.toml, change version = "0.1.0" to "0.2.0", then:
cargo run
# Build the optimised version and look at the two folders:
cargo build --release
dir target\debug
dir target\release
@@text What to look for
In the third run, Cargo's first line says `Compiling message v0.2.0`. After `cargo build --release` the file `message.exe` appears in `target\release`. This project is based on the README's own mini task in the course repository.
@@quiz Check yourself
? What does `Cargo.lock` record?
- The licence of every crate you use
+ The exact version of every dependency used in the last build
- The password for crates.io
= Cargo writes the exact versions into the lock file, so that the same code builds the same way tomorrow. It is the equivalent of library versions frozen in a CODESYS project.
? Which command do you reach for while fixing compiler errors, because it is quickest?
- `cargo build --release`
+ `cargo check`
- `cargo clean`
= `cargo check` stops after the compiler's checks and skips producing the program.
@@chapter 1.2.2 | Lesson 2.2 - Variables | 8
Bindings that cannot change unless you say so: `let`, `let mut`, `const`, type inference and shadowing.
@@objectives
- Declare a value with `let` and explain why assigning to it again fails
- Choose between `let`, `let mut` and `const`
- Use shadowing to give a name a new value or type
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.2 - Variables · 5:07 — attach the MP4 here when you have it.
@@callout note | Plant situation
Batch 7 on the bottle filler: 500 ml bottles. Around bottle 340 the target volume suddenly reads 550 ml and the line overfills for twenty minutes. After half a day of searching you find a forgotten assignment in a seldom-used cleaning step that writes the volume tag. In Rust the volume would be created as a value that cannot change. The cleaning step could not write to it, and the mistake would be an error on the day you wrote it, not a loss on the day you ran the batch.
@@text Teacher's notes
### `let`: a value that stays put
`let fill_volume_ml = 500;` creates a **binding**, a name attached to a value. By default the binding is fixed. Writing `fill_volume_ml = 550;` later fails with error **E0384**: *cannot assign twice to immutable variable*. Think of a value that is set once, like a `VAR CONSTANT`, but decided variable by variable.

### `let mut`: the opt-in
`let mut bottles_filled = 0;` makes an ordinary changeable variable, like a `VAR` in a PLC. Because `mut` appears at the declaration, one glance tells you which values can change and which cannot. When you read unfamiliar code at 2 a.m., that is a gift.

### Types are inferred
You did not write a type for `fill_volume_ml`. The compiler works it out: whole numbers become `i32` unless something says otherwise, decimals become `f64`. You can write it yourself when you want: `let fill_volume_ml: i32 = 500;`. After that the type never changes. Module 3 goes through the number types.

### `const`: known before the program runs
`const MAX_FILL_ML: i32 = 520;` is a constant. Differences from `let`:

- The type is **required**.
- The value must be something the compiler can compute while building.
- It can sit outside any function, in the whole file.
- By convention the name is in CAPITAL_LETTERS.
- It is never `mut`.

### Shadowing: the same name, a new binding
`let fill_volume_ml = fill_volume_ml / 1000.0;` declares a **new** binding that hides the old one from that line on. It is not a change of the old value. That is why the new binding may have another type, which `mut` never allows. Shadowing suits a value that you convert step by step, like a text read from a file that becomes a number.

### Why Rust is built this way
Most bugs come from values that changed when nobody expected it. Fixed by default means a changing value is announced in the code where it is declared.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| A value fixed after its first write, such as a `VAR CONSTANT` | `let` | The rule is per binding; there is no separate "constant" section |
| Ordinary `VAR` | `let mut` | A `mut` binding is still local to its block |
| `VAR_GLOBAL CONSTANT` | `const` | Needs a type, and its value must be known at build time |
| Declaration in a variable list at the top | `let` where you first need it | You declare close to first use |
| Re-using a temp variable for something else | Shadowing with a new `let` | A new binding, possibly of a new type |
@@code mermaid | Diagram: three ways to name a value
flowchart LR
  A["let fill_ml = 500"] --> B["Fixed after this line"]
  C["let mut count = 0"] --> D["May be assigned again"]
  E["const MAX_ML: i32 = 520"] --> F["Known at build time, type required"]
  G["let fill_ml = fill_ml + 10"] --> H["New binding hides the old one"]
@@code rust | Try it: let, mut, const and shadowing
const MAX_FILL_ML: i32 = 520;

fn main() {
    let fill_volume_ml = 500; // fixed after this line
    let mut bottles_filled = 0; // opted in to change

    bottles_filled = bottles_filled + 1;
    bottles_filled = bottles_filled + 1;

    // fill_volume_ml = 550; // ❌ E0384: cannot assign twice to immutable variable

    println!("Volume {fill_volume_ml} ml, limit {MAX_FILL_ML} ml, filled {bottles_filled}");

    // Shadowing: a new binding with the same name, and this time another type.
    let fill_volume_ml = fill_volume_ml as f64 / 1000.0; // `as` converts a number type
    println!("Volume in litres: {fill_volume_ml}");
}
// Output:
// Volume 500 ml, limit 520 ml, filled 2
// Volume in litres: 0.5
@@callout warning | Watch out
The classic: you declare `let count = 0;`, add to it later and get **E0384**. The fix is `let mut count = 0;`. Do not add `mut` everywhere in advance: if a value does not change, leaving `mut` off tells the reader something true, and the compiler warns *variable does not need to be mutable* when you add it for nothing. The other trap is mixing up shadowing and `mut`. After `let x = x + 1;` the old `x` still exists, hidden behind the new one. Inside a block, it comes back when the block ends.
@@text Exercise
Exercise A in chapter 1.2.5 practises everything on this page. For now, a two-minute warm-up.

1. In your practice project, declare `let line_speed_mm_s = 250;` and print it.
2. Try to assign `line_speed_mm_s = 300;`. Read the error code. Fix it with `mut`.
3. Add `const MAX_SPEED_MM_S: i32 = 500;` above `main`, and print the speed as a percentage of it.
@@quiz Check yourself
? What happens if you write `let fill_ml = 500;` and then `fill_ml = 550;`?
- The second line silently wins
+ The compiler stops with E0384
- The program panics when it runs
= The binding is fixed by default. The error appears when you build, long before any bottle is filled.
? What is different about `const` compared to `let`?
- A `const` can be changed with `mut`
- A `const` has no name
+ A `const` needs a type and a value known when the program is built
= Constants are computed by the compiler and can live outside any function.
? After `let x = 5; let x = x * 2;`, what is true?
+ The second `x` is a new binding that hides the first one
- The first `x` was changed to 10
- The code does not compile
= Shadowing creates a new variable with the same name. It is not a change to the old one, and it may even change the type.
@@chapter 1.2.3 | Lesson 2.3 - Scope | 5
Where a name is visible and when its value goes away: blocks, the curly braces that end a scope, and blocks that produce a value.
@@objectives
- Explain where a variable is visible and when it stops existing
- Read error E0425 and fix it
- Use a block to compute a value without leaking its helper variables
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.3 - Scope · 2:44 — attach the MP4 here when you have it.
@@callout note | Plant situation
On a leak tester three steps share one global called `temp_pa`. Step 20 uses it for the start pressure. Step 30 uses it for the end pressure, but on one path it forgets to write it first and reads the leftover from step 20. The part shows zero decay and passes. It was a bad part. Rust avoids the problem by making a value live only in the block where it is created. A step cannot read a leftover, because there is no leftover to read.
@@text Teacher's notes
### A scope is a pair of braces
Everything between `{` and `}` is a **scope**. A variable declared inside it exists from its `let` line to the closing brace. After that it is gone, and its name no longer means anything. A function body is a scope too, and its parameters live in it.

Using a name outside its scope gives error **E0425**: *cannot find value in this scope*.

### Inner scopes can see outer names
Code inside a block can use the names declared outside it, as long as they were declared earlier. Names flow inwards, never outwards.

### Shadowing inside a block
If you declare a new binding with an outer name inside a block, it hides the outer one **only until the closing brace**. After the block, the outer binding is back with its old value. Compare this with `mut`, where an assignment inside the block would have changed the outer value.

### A block is an expression
The last line of a block, written **without a semicolon**, is the value of the block. You can use that to compute a value with helper variables that vanish afterwards:

`let decay_pa = { let start = ...; let end = ...; start - end };`

Only `decay_pa` survives. `start` and `end` cannot be misused later. This is the closest thing Rust has to `VAR_TEMP`, but it can be placed around any few lines.

### Why it matters
Values that are cleaned up at the end of a scope are the base of Rust's memory rules. When the scope ends, the value is dropped and its memory released, at a moment the compiler knows. Lesson 2.4 and Module 4 build on this.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| `VAR_TEMP`: gone when the call ends | A variable inside a block | The block can be any `{ }`, not just a whole POU |
| A global read from anywhere | A `static`, rarely needed | Locals are the normal way in Rust |
| Local variable of a function | A variable in the function body | Function parameters are in its scope too |
| Leftover value from the last scan | Does not exist in a block | A block starts fresh every time it runs |
| A name hidden by a local with the same name | Shadowing | Hiding ends at the closing brace |
@@code mermaid | Diagram: who can see what
flowchart TD
  A["main: station, pressure_bar"] --> B["Block 1: start_pa, end_pa"]
  B --> C["Block 1 ends: start_pa and end_pa are gone"]
  A --> D["Block 2: pressure_bar shadowed"]
  D --> E["Block 2 ends: outer pressure_bar is back"]
@@code rust | Try it: blocks, shadowing and a block that returns a value
fn main() {
    let station = "Leak tester 1";
    let pressure_bar = 6.0;

    let decay_pa = {
        let start_pa = 600_000.0; // lives only inside this block
        let end_pa = 598_500.0;
        start_pa - end_pa // no semicolon: this is the value of the block
    };

    // println!("{start_pa}"); // ❌ E0425: cannot find value `start_pa` in this scope
    println!("{station}: decay {decay_pa} Pa at {pressure_bar} bar");

    {
        let pressure_bar = 0.5; // hides the outer one until the closing brace
        println!("inside the block: {pressure_bar} bar");
    }
    println!("after the block: {pressure_bar} bar");
}
// Output:
// Leak tester 1: decay 1500 Pa at 6 bar
// inside the block: 0.5 bar
// after the block: 6 bar
@@callout warning | Watch out
You wrote `let answer = 42;` inside a block and print it one line after the closing brace. Result: **E0425**, *cannot find value `answer` in this scope*. Two fixes: move the `let` out of the block to where the value is used, or move the use into the block. Choose by asking who else needs the value. The second trap is the semicolon at the end of a block: `{ a - b; }` is a block that gives back nothing (`()`), not the difference.
@@text Exercise
Exercise B (chapter 1.2.7) has a scope puzzle in step 2. A warm-up now:

1. Write a block that computes `fill_time_s` from `volume_ml = 500` and `flow_ml_s = 250` with two helper variables inside the block.
2. Print the result outside the block.
3. Try to print one of the helper variables after the block, read the error and delete the line.
@@quiz Check yourself
? Where does a variable declared inside `{ ... }` stop existing?
- At the end of the function that holds the block
+ At the closing brace of that block
- When the program ends
= A scope ends at its closing brace. The name cannot be used after it.
? Inside a block you write `let x = 2;` while an outer `x` is 1. What is `x` after the block?
+ 1
- 2
- The code does not compile
= The inner `x` hides the outer one only until the block ends. After that the outer `x` is visible again with its old value.
@@chapter 1.2.4 | Lesson 2.4 - Memory Safety | 4
What memory-safety bugs are, how they look on a plant, and how Rust turns most of them into compile errors or loud, clean stops.
@@objectives
- Name the main memory bugs Rust is built to prevent
- Explain the difference between a compile error and a panic
- Use `.get()` to read from a table without risking a panic
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.4 - Memory Safety · 1:33 — attach the MP4 here when you have it.
@@callout note | Plant situation
On the packaging line the sealing temperature comes from a table of ten recipes. The HMI sends recipe number 12 by mistake. In a PLC program that has no bounds check on the array, the read lands on whatever variables sit next to the table in memory, and the sealer runs at a setpoint nobody chose. In CODESYS you add the `CheckBounds` function to get a stop instead. Rust makes the safe behaviour the default: an index is always checked, and out of range means the program stops with the exact line, or you can ask politely with `.get()`.
@@text Teacher's notes
### The bug family
Memory-safety bugs are mistakes in how a program touches memory:

- **Out of bounds**: reading or writing past the end of an array or buffer.
- **Use of uninitialised memory**: reading a variable that nobody wrote.
- **Use after release**: using a value whose memory was already given back.
- **Data races**: two threads changing the same data at the same time with no coordination.

These are the kinds of bugs that appear once a month, cannot be reproduced and leave no log.

### How Rust deals with each one
- **Out of bounds**: every index is checked. If it is out of range, the program **panics**: it stops with a message that has the file, the line, the length and the index.
- **Uninitialised**: you cannot read a variable that might not have been written. The compiler says **E0381**.
- **Use after release** and **data races**: the ownership rules of Module 4 forbid them while compiling. A program with such a bug is never built.

### Compile error or panic
Sometimes the compiler can see the mistake ahead of time, such as an index that is a literal number beyond the end of a fixed array. That is a compile error. When the index only arrives at run time, from the HMI or a file, the check happens then and a bad index panics. Either way the damage stops at the line that caused it.

### Reading safely with `.get()`
`table.get(index)` returns an `Option`: `Some(value)` when the index exists and `None` when it does not. You then decide what to do. That is how you write a service that does not stop because of one bad request. (`match` and `Option` get their own lessons later.)

### What safe does not mean
Safe Rust has no memory corruption. It can still contain logic errors, and it can still panic. A panic is a loud and clean stop, which is far better than silent damage, but it is still a stop.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| Array access with the `CheckBounds` function added | Indexing is always checked | There is no way to switch the check off in safe code |
| Variables are initialised to zero implicitly | You cannot read an unset variable (E0381) | Rust makes you say what the start value is |
| A pointer to a local that no longer exists | The borrow checker refuses it | Module 4 |
| Two tasks writing the same GVL variable | A data race is rejected while compiling | Part 2 |
| Runtime exception stops the PLC | `panic!` stops the thread, or the whole program in the main thread | The message tells you file and line |
@@code mermaid | Diagram: one mistake, two outcomes
flowchart LR
  A["Mistake: recipe 12 in a table of 10"] --> B["Unchecked language"]
  A --> C["Rust: the index is checked"]
  B --> D["Reads neighbour memory: random setpoint or crash"]
  C --> E["Known at build time: compile error"]
  C --> F["Known at run time: panic with file and line"]
  C --> G["Using get: you decide what to do"]
@@code rust | Try it: a recipe table that cannot be read out of range
fn show(table: &[i32], recipe: usize) {
    // table[recipe] would stop the program with "index out of bounds" if recipe >= table.len().
    // .get() gives back an Option instead: Some(value), or None when there is no such entry.
    match table.get(recipe) {
        Some(temp_c) => println!("Sealing temperature for recipe {recipe}: {temp_c} C"),
        None => println!(
            "Recipe {recipe} does not exist (table has {} entries), keeping the old setpoint.",
            table.len()
        ),
    }
}

fn main() {
    let sealing_temp_c = [160, 165, 170, 175, 180, 185, 190, 195, 200, 205];
    show(&sealing_temp_c, 3);
    show(&sealing_temp_c, 12);
}
// Output:
// Sealing temperature for recipe 3: 175 C
// Recipe 12 does not exist (table has 10 entries), keeping the old setpoint.
@@callout warning | Watch out
"Memory safe" is not "never stops". Index a table with `table[12]` when it has ten entries and the program panics: `index out of bounds: the len is 10 but the index is 12`, exit code 101. That is the intended behaviour, so the fix is to check first or to use `.get()`. Likewise, reading a variable that a branch may not have set gives **E0381**, *used binding is possibly-uninitialized*. Fix it by giving the variable a value on every path, as Exercise B step 4 does.
@@text Exercise
A short experiment.

1. In your practice project, paste the Try it code and run it.
2. Change `show(&sealing_temp_c, 12)` to `show(&sealing_temp_c, 9)` and see the table's last entry.
3. Add a third call with recipe `10`. It is one past the end, so where does your program end up?
@@quiz Check yourself
? An index arrives from the HMI at run time and is larger than the array. What does Rust do with `table[index]`?
- Reads the next variable in memory
- Returns 0
+ Panics with a message that names the line, the length and the index
= Indexing is always checked. A panic is a clean stop, not memory damage.
? What does `table.get(index)` return?
+ An `Option`: `Some(value)` or `None`
- Always the first element
- A copy of the whole table
= `get` lets you handle a missing entry as an ordinary case instead of a crash.
@@chapter 1.2.5 | Lesson 2.5 - Exercise A - Variables | 21
Exercise A: create a project, print two values, subtract, meet the mutability error, switch to constants. The only exercise without a Cargo project in the folder.
@@objectives
- Create a Cargo project, edit its version and see it in Cargo's output
- Write `let`, `let mut` and `const` in a working program
- Read the compiler's messages for E0384 and E0070
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.5 - Exercise A - Variables · 4:56 — attach the MP4 here when you have it.
@@callout note | Plant situation
The bottle hopper on the filler holds 8 bottles at the start of a test, and each batch takes 2. You are asked for a ten-line console program that prints "taking 2 of 8" and then "6 left", which is a throw-away stub for a stock counter. In the PLC project the starting stock is a constant and the running stock is a variable. In Rust you spell out the same difference with `const` and `let mut`. The exercise text talks about missiles. Read it as bottles if that helps; the Rust is identical.
@@text Exercise brief
**Goal.** Build a tiny program that holds two numbers, prints them, subtracts one from the other and prints again. Use it to practise the three ways to name a value: `let`, `let mut` and `const`.

Steps:

1. Make a new project called `variables` with `cargo new`. Put it in `D:\RustroverProjects\learn_rust\mine`, because this exercise folder holds only a README.
2. In `Cargo.toml` change the version to `2.3.4`. Run the program and find `2.3.4` in Cargo's output.
3. At the start of `main`, declare two values with `let` (8 and 2) and print them with the `println!` line given in the task list.
4. Subtract the second from the first and print the result. The compiler will complain about mutability. Read the message, then fix the declaration.
5. Replace the literal numbers with two `const` items, placed above `main`.
6. Try the extra challenges: type annotations, a pattern that binds both values on one line, using the expression without changing the variable, an unused variable, and assigning to a constant.

The task list from the course repository is below. Tick it off as you go.
@@code text | Starter code — exercise/a_variables/README.md (MIT © Nathan Stocks)
# Exercise A: Variables

### Part 1
- [ ] Make a new project named `variables` using cargo
  - See "cargo help" if you forgot the command.
- [ ] Open `Cargo.toml`
  - [ ] Change the version number to `2.3.4` and save the file.  Keep an eye out for that version number in cargo's output when you run it!
- [ ] In `src/main.rs` at the start of the `main()` function:
  - [ ] Declare the variable `missiles` and initialize it to `8`
  - [ ] Declare the variable `ready` and initialize it to `2`
- [ ] Change the `println!(...)` at the end of `main()` to:
  - `println!("Firing {} of my {} missiles...", ready, missiles);`
- [ ] Run your program using cargo (see "cargo help" if you forgot the command).
  Some common errors you may hit:
  - Forgot to use `let` to bind a variable
  - Forgot a semicolon `;` at the end of a line

### Part 2

- [ ] After the `println!(...)`, subtract `ready` from `missiles` like this:
  - `missiles = missiles - ready;`
- [ ] Add a second `println!(...)` to the end:
  - `println!("{} missiles left", missiles);`
- [ ] Run your program again using cargo
  - Did you run into an error about mutability? Go back and add `mut` at the right spot on the line where you declare `missiles`.
- [ ] Declare a constant named `STARTING_MISSILES` and set it to `8` (the type is `i32`).
- [ ] Declare a constant named `READY_AMOUNT` and set it to `2` (also `i32`).
- [ ] Use the constants to initialize `missiles` and `ready`
  - Where did you put the constants?  If you put them inside the `main()` function, try moving them up above `main()` to module scope! 
- [ ] Nice. Congratulate yourself on a job well done!  You are a Rust programmer now!

### Extra challenges:
- [ ] Explicitly annotate the variables with the type `i32`
- [ ] Try binding the variables all at once on one line using a pattern (parentheses and commas) -- can you figure out where `mut` goes?
  - [ ] Can you figure out the correct type annotation when you assign them all in one line?  Hint: it will use the same sort of pattern as the variables and values.
- [ ] Instead of changing missiles, use the value `missiles - ready` directly in the second `println!(...)`
  - What warning does cargo emit when you run your program now? Can you fix it?
- [ ] Add another variable to your program *but don't use it*.
  - What does cargo say when you run your program?
- [ ] Try modifying a constant in `main()` (for example, `READY_AMOUNT = 1`). What does the error look like?
@@code bash | Run it
# Local copy: D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\a_variables\README.md
New-Item -ItemType Directory -Force D:\RustroverProjects\learn_rust\mine | Out-Null
cd D:\RustroverProjects\learn_rust\mine
cargo new variables
cd variables
cargo run
@@callout tip | Hints
1. `cargo new variables` creates a folder called `variables`. Change into it before you run `cargo run`. `cargo help` lists the commands.
2. A new value is introduced with the word `let`. If you forget it, the compiler says `cannot find value` (E0425).
3. A message with the code **E0384** and the words *cannot assign twice to immutable variable* means: the declaration needs `mut`. The compiler even shows the place.
4. Constants are written `const NAME: i32 = 8;`. The type is required. Put them above `main`, outside any function.
@@code rust | Solution — try it yourself first
// Our solution, with plant names instead of missiles.
// For the exact exercise text rename bottles -> missiles and batch -> ready.
const STARTING_BOTTLES: i32 = 8;
const BATCH_SIZE: i32 = 2;

fn main() {
    // Parts 1 and 2: constants feed the variables; `mut` allows the one change.
    let mut bottles = STARTING_BOTTLES;
    let batch = BATCH_SIZE;
    println!("Taking {} of my {} bottles...", batch, bottles);
    bottles = bottles - batch;
    println!("{} bottles left", bottles);

    // Extra: bind both at once with a pattern, with explicit types. `mut` goes inside the pattern.
    let (mut hopper, per_batch): (i32, i32) = (STARTING_BOTTLES, BATCH_SIZE);
    hopper = hopper - per_batch;
    println!("Hopper after one batch: {hopper}");

    // Extra: use the expression directly instead of changing the variable (then no `mut` is needed).
    println!("After a second batch: {}", hopper - per_batch);
}
// Output:
// Taking 2 of my 8 bottles...
// 6 bottles left
// Hopper after one batch: 6
// After a second batch: 4
@@callout warning | Watch out
Two messages you are meant to meet. Assigning to a constant, as in `READY_AMOUNT = 1;`, gives `error[E0070]: invalid left-hand side of assignment`. A constant is not a place that you can write to. And if you leave `mut` on a variable that is never changed, Cargo prints a warning, *variable does not need to be mutable*. A warning is not an error and the program still runs, but treat each one as a to-do. Also remember the exercise folder holds no project: if `cargo run` there says it cannot find `Cargo.toml`, you are in the wrong folder.
@@quiz Check yourself
? In Exercise A the line `missiles = missiles - ready;` fails to compile. What is the fix?
- Declare it with `const`
+ Declare it with `let mut`
- Remove the semicolon
= The subtraction assigns a new value to a binding that is fixed by default. `mut` at the declaration opts it in.
? Where does the exercise suggest you put `const STARTING_MISSILES: i32 = 8;`?
- Inside the `println!` call
+ Above `main`, at the top level of the file
- In `Cargo.toml`
= A constant can be declared inside a function too, but the exercise moves it to the file level so that every function can use it.
? How do you bind two variables at once, with only the first changeable?
- `let mut (a, b) = (1, 2);`
+ `let (mut a, b) = (1, 2);`
- `let (a, b) = mut (1, 2);`
= In a pattern, `mut` goes in front of the single name that should change.
@@chapter 1.2.6 | Lesson 2.6 - Functions | 5
Defining and calling functions: typed parameters, a return type, and the last expression as the return value.
@@objectives
- Write a function with parameters and a return type
- Explain why a trailing semicolon changes the return value
- Choose between a tail expression and `return`
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.6 - Functions · 2:15 — attach the MP4 here when you have it.
@@callout note | Plant situation
The time to fill a bottle is computed in four places in the filler's program: volume times 1000 divided by the flow. In one of them the factor is 100, a typo made during a Friday change. Bottles in that nozzle group are short by 40 percent of what they should be, and the weigh-scale rejects them for a week. One function, `fill_time_ms`, written once and called four times, would have made the typo impossible. That is what functions are for: one place for each piece of logic.
@@text Teacher's notes
### The shape of a function
`fn fill_time_ms(volume_ml: u32, flow_ml_s: u32) -> u32 { ... }`

- `fn` and then the name, written in `snake_case`.
- Parameters in parentheses. Each one has a **name and a type**, always. There is no guessing in a signature.
- `->` and the type of the value it gives back. No arrow means it gives back nothing.
- The body in curly braces.

Calling it is just `fill_time_ms(500, 250)`. The order of functions in the file does not matter. You can call a function that is written further down.

### The return value is the last expression
Look at the body: `volume_ml * 1000 / flow_ml_s` has **no semicolon**. That makes it the value of the block and so the value the function returns. This is the same rule as a block giving a value in lesson 2.3.

If you add a semicolon, the line becomes a statement, which has no value. The function then gives back nothing, and the compiler objects because you promised a `u32`: **E0308**, *mismatched types*, with a hint to remove the semicolon.

### `return` for leaving early
`return x;` leaves the function at once. Use it for early exits, like a guard at the top: `if flow_ml_s == 0 { return 0; }`. For the normal last line, use the plain expression. Rust code does not usually write `return` at the very end.

### Functions have no memory
Like a PLC `FUNCTION`, a Rust function keeps nothing between calls. All that comes out is the return value. When you need memory between calls, you will use a `struct` (Module 5), which is the function block of Rust.

### Expressions and statements
`let x = 5;` is a statement: it does something and has no value. `a + b` is an expression: it has a value. Many things in Rust are expressions, including `if` and blocks. Keep that in the back of your mind. Many surprises disappear once you ask "is this a statement or an expression?"
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| `FUNCTION` with `VAR_INPUT` | `fn` with parameters | Parameter types are always written out |
| Assign the result to the function's own name | Last expression without a semicolon | `return` also exists, for early exits |
| `RETURN` | `return` | |
| No memory between calls | The same | State lives in a `struct` |
| Several outputs through `VAR_OUTPUT` | Return one value, which may be a tuple | Tuples are in Module 3 |
@@code mermaid | Diagram: a function call
sequenceDiagram
  participant M as main
  participant F as fill_time_ms
  M->>F: volume 500, flow 250
  F->>F: volume times 1000 / flow
  F-->>M: 2000
@@code rust | Try it: a function that returns a value, and one that does not
fn fill_time_ms(volume_ml: u32, flow_ml_s: u32) -> u32 {
    volume_ml * 1000 / flow_ml_s // no semicolon: this is the return value
}

fn log_line(station: &str, text: &str) {
    println!("[{station}] {text}");
}

fn main() {
    let time_ms = fill_time_ms(500, 250);
    log_line("Filler 2", "fill time calculated");
    println!("Fill time: {time_ms} ms");
    println!("A 330 ml can: {} ms", fill_time_ms(330, 250));
}
// Output:
// [Filler 2] fill time calculated
// Fill time: 2000 ms
// A 330 ml can: 1320 ms
@@callout warning | Watch out
You typed `a + b;` as the last line of a function that promises `-> i32`. The compiler stops with **E0308**: *mismatched types*, and its help text says to remove the semicolon. It is the most common error of the first week. The fix is always the same: if the last line is meant to be the answer, it has no semicolon. A function without a `->` gives back `()`, the "nothing" value, which is why a trailing semicolon there is correct.
@@text Exercise
Exercise B is the next lesson. A short warm-up first.

1. Write `fn pitch_mm(speed_mm_s: u32, bottles_per_s: u32) -> u32` that returns the distance between bottles.
2. Call it from `main` with 300 and 3, and print the result.
3. Add a semicolon to the last line of the function, run, and read the error. Remove it.
@@quiz Check yourself
? What makes `a + b` the return value of a function?
- The keyword `return` in front of it
+ It is the last line and has no semicolon
- A comment saying `// return`
= The final expression of a block, written without a semicolon, is the value of the block and so of the function.
? You write `fn add(a: i32, b: i32) -> i32 { a + b; }`. What happens?
- It returns the sum
+ Error E0308, mismatched types
- It returns 0
= The semicolon turns the sum into a statement. The body then produces `()`, not the `i32` promised.
@@chapter 1.2.7 | Lesson 2.7 - Exercise B - Functions | 21
Exercise B: shadowing a variable with a new type, fixing a scope problem, writing an `add` function, and initialising a variable late.
@@objectives
- Shadow a value with a new binding of another type
- Fix a scope error by moving a declaration
- Write a function that returns a value, and declare a variable before initialising it
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.7 - Exercise B - Functions · 4:51 — attach the MP4 here when you have it.
@@callout note | Plant situation
A flow meter on the extruder's cooling loop reports 3.989 and the downstream function wants a whole number. A colleague's half-finished code does not build: the types do not fit, a variable is declared in a block and used outside it, a helper function is missing, and one value may be read before anything has set it. These are the four most common first-week problems in Rust, lined up in one file. You fix them one at a time, and after each one the compiler's list gets shorter.
@@text Exercise brief
**Goal.** Take the half-finished `main.rs` below and get it to run. Each of the four numbered steps teaches one thing: shadowing, scope, writing a function, and initialising late.

Steps:

1. **Shadowing.** `number` is an `f64`, but `inspect_integer` wants an `i32`. Declare a new `number` that holds the converted value. Use `let` again; do not try to change the old one.
2. **Scope.** Uncomment the block that declares `answer` and the line that prints it. They do not fit together. Fix the scope so it prints `The answer is 42`.
3. **A function.** Write `add`, which takes two `i32` and returns their sum, then uncomment the lines that use it. You should see `4 + 42 = 46`.
4. **Late initialisation.** Uncomment the `countdown` code. The compiler complains that `countdown` may be used before it is set. Give it a value in the `else` branch. You should see a countdown of 10.
5. **Challenge.** Rewrite the two functions with a tail expression instead of `return`.

Open the folder in your editor, run `cargo run` and read the first error. The first lines of the output you are working towards are `The integer is 4` and `The answer is 42`.
@@code rust | Starter code — exercise/b_functions/src/main.rs (MIT © Nathan Stocks) | The first line, `// no-check`, is ours: it tells our snippet checker that this file is unfinished on purpose. The rest is the file as it is in the repository.
// no-check
// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let number: f64 = 3.989; // don't change this line!

    // 1. Try running the code and looking at the error. We would like to use the variable name
    // `number` as an i32 (an integer), but it is already used as an f64 (a floating point number).
    //
    // - Uncomment the commented-out code below
    // - Complete the code to shadow the old `number` variable with a new `number` variable
    //   of the correct type.

    // ... = convert_to_integer(number); // uncomment this line and finish shadowing `number`
    inspect_integer(number); // don't change this line!

    // 2. Uncomment and run the code below. Fix the scope problem so that the code compiles and runs
    // producing the answer 42.

    // {
    //     let answer = 42;
    // }
    // println!("The answer is {}", answer);

    // 3. Create a function named `add` that adds two i32 values together and returns the result.
    // Then uncomment the code below. You should get the output "4 + 42 = 46"
    //
    // Note: If you fixed the scope problem from #2 by moving the `println` up into the nested
    // scope, then you will have to change the code above again so that `answer` is in this scope.

    // let sum = ...  // call your `add` function and pass it `number` and `answer` as arguments.
    // println!("{} + {} = {}", number, answer, sum);

    // 4. You can declare a variable without initializing it, but the compiler must be able to
    // ensure that it will always be initialized before you can use it.
    //
    // Uncomment and run the code below to see the error. Fix the error by setting countdown to 0
    // in the `else` branch of the `if` expression. Run the code. You should see a countdown of 10.

    // let countdown: i32; // declares countdown, but doesn't initialize it
    // if answer < 100 {
    //     countdown = 10;
    // } else {
    //     println!("The answer is clearly wrong.");
    //     // set countdown to some value here
    // }
    // println!("The countdown begins at {}", countdown);
}

fn inspect_integer(x: i32) {
    println!("The integer is {}", x);
}

// Challenge: A "tail expression" is when the last expression in a block does not end with a
// semicolon, making it the value of the block.
//
// - Refactor the body of this function to be a "tail expression" instead of a return statement.
// - Make the same change to the `add` function that you created
// - Run the code and make sure you get the same output as you did before
fn convert_to_integer(num: f64) -> i32 {
    // For more information on using `as` to cast between numeric types, see:
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
    return num.round() as i32;
}
@@code bash | Run it
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\b_functions
cargo run
# To keep the clone clean, copy the folder first and work in the copy:
#   Copy-Item -Recurse D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\b_functions D:\RustroverProjects\learn_rust\mine\b_functions
@@callout tip | Hints
1. Step 1: the shadowing line has the form `let number = convert_to_integer(number);`. The right-hand `number` is still the old `f64`; the new one is an `i32`.
2. Step 2: the simplest fix is to take `let answer = 42;` out of its braces so that it lives in `main`'s own scope. Then the print line can see it.
3. Step 3: `fn add(a: i32, b: i32) -> i32 { ... }` can sit anywhere in the file, below `main` is fine. Mind the semicolon on the last line (lesson 2.6).
4. Step 4: the compiler wants every path to set `countdown`. In the `else` branch write `countdown = 0;`. The variable does not need `mut`, because it is assigned exactly once on any path.
@@code rust | Solution — try it yourself first
// Our solution to Exercise B, all four steps and the challenge.
#![allow(unused_variables)]

fn main() {
    let number: f64 = 3.989; // don't change this line!

    // 1. Shadow: a new `number`, this time an i32.
    let number = convert_to_integer(number);
    inspect_integer(number);

    // 2. `answer` is declared in main's own scope, so later lines can see it.
    let answer = 42;
    println!("The answer is {}", answer);

    // 3. `add` takes two i32 and gives back their sum.
    let sum = add(number, answer);
    println!("{} + {} = {}", number, answer, sum);

    // 4. Every path through the if/else sets `countdown` before it is read.
    let countdown: i32;
    if answer < 100 {
        countdown = 10;
    } else {
        println!("The answer is clearly wrong.");
        countdown = 0;
    }
    println!("The countdown begins at {}", countdown);
}

fn inspect_integer(x: i32) {
    println!("The integer is {}", x);
}

// Challenge: the last line has no semicolon, so it is the return value.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn convert_to_integer(num: f64) -> i32 {
    num.round() as i32
}
// Output:
// The integer is 4
// The answer is 42
// 4 + 42 = 46
// The countdown begins at 10
@@callout warning | Watch out
Each step has its own error. Step 1 starts as **E0308**, *mismatched types*: expected `i32`, found `f64`. A scope mistake in step 2 gives **E0425**, *cannot find value `answer` in this scope*. Step 4 gives **E0381**, *used binding `countdown` is possibly-uninitialized*. A common wrong turn is "fixing" step 4 by writing `let mut countdown = 0;` at the top. That compiles, but it hides the point of the step: the compiler is checking that every path sets the value, and you want it to keep doing that.
@@quiz Check yourself
? In step 1 you write `let number = convert_to_integer(number);`. What is the type of the new `number`?
- `f64`, like the old one
+ `i32`, the return type of the function
- It depends on what you do next
= Shadowing creates a new binding, so it can have a different type from the one it hides.
? Step 4 gives E0381. What does the compiler want?
- That `countdown` is declared `mut`
+ A value assigned on every path before `countdown` is read
- That the `if` has no `else`
= The compiler follows every path through the `if`. If one path leaves `countdown` unset, reading it afterwards is refused.
? A function ends with `a + b` and no semicolon. What is that line called?
- A statement
+ A tail expression
- A comment
= The last expression of a block, with no semicolon, is the block's value.
@@chapter 1.2.8 | Lesson 2.8 - Module System | 12
Splitting a program into modules and files: `mod`, `pub`, `use`, paths, and the difference between `lib.rs` and `main.rs`.
@@objectives
- Declare a module inline and in its own file
- Make an item visible with `pub` and bring it into scope with `use`
- Explain how a package can hold both a library and a program
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.8 - Module System · 8:57 — attach the MP4 here when you have it.
@@callout note | Plant situation
The palletiser project has grown to a single file of 2,400 lines. Two engineers each wrote a `start()`: one starts the conveyor, one the stacker. They collide, so one is renamed `start2`, and nobody knows what `start2` means after a month. In CODESYS you would have put the POUs in folders and given the library a namespace, so that `Conveyor.Start` and `Stacker.Start` live peacefully side by side. Rust's module system is that, and it is also the fence that decides what the outside may touch.
@@text Teacher's notes
### A module is a named box
`mod conveyor { ... }` creates a module called `conveyor`. Things inside belong to it, and you reach them with a path: `conveyor::clamp_speed(650)`. Two modules can both have a `start` without trouble, since the paths are different.

### Privacy: closed by default
Everything inside a module is **private**. Code outside cannot use it. To open a door you write `pub` in front: `pub fn clamp_speed`. A private item is invisible from outside, and trying to use it gives **E0603**, *function is private*. Rust's default is "internal" and you choose what to publish. That is also how you keep a library's inner workings free to change.

A module's children can see the private items of their parents, but not the other way round. A child reaches its parent with `super::`.

### `use`: a shorter name
Writing a long path every time is tedious. `use conveyor::drive;` lets you write `drive::start()` afterwards. A `use` line only shortens a path; it does not copy or run anything. Paths can start with `crate::` (the root of this crate), `self::` (this module) or `super::` (the parent).

### Modules in files
A module can live in its own file. The line `mod russian;` in `lib.rs` tells the compiler: "there is a module called `russian`, look for `src/russian.rs`". If that module has submodules, they go in a folder with the module's name: `pub mod authors;` inside `russian.rs` finds `src/russian/authors.rs`. A file that no `mod` line mentions is not part of the program at all.

### Paths in detail
A path says where an item lives, the way `Conveyor.Drive.Start` does in CODESYS.

- `conveyor::drive::start()` is a relative path: start from a name that is in scope.
- `crate::conveyor::drive::start()` is an absolute path from the root of the crate. It works from any module.
- `super::log(...)` goes up one level. `self::` means the current module.

To give an item a second, shorter address for other people, you can re-export it with `pub use`. A library does this for its most popular items. That is what Challenge 2 in Exercise C asks for.

### Library and program in one package
A Cargo package can have `src/lib.rs` (the library, whose items others can use) and `src/main.rs` (the program). The program then uses the library by the **package name**, as `use hello::english;`. In the course's `hello` example, `lib.rs` declares the modules and `main.rs` calls them. Splitting this way lets the same code be used by several programs and tested on its own.

### The example you will see in the lesson
It has one inline module (`english`) and two file modules (`russian`, `spanish`), with a nested `authors` module in `russian`. The five files are listed after the Watch out box.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| Folder or namespace in the POU tree | `mod` | A module can also be a file |
| Public in the library, otherwise internal | `pub` | Private is the default |
| Referring to a library namespace, `Conveyor.Start()` | A path such as `conveyor::start()` | `use` shortens paths |
| The library's top level | `lib.rs`, the crate root | |
| The application, `PLC_PRG` | `main.rs` | One package can hold both |
| A POU outside every folder is still in the project | A file with no `mod` line is not compiled | The `mod` line is what adds the file |
@@code mermaid | Diagram: the module tree of the hello example
flowchart TD
  R["lib.rs: crate root hello"] --> E["english: inline module"]
  R --> RU["russian: src/russian.rs"]
  R --> S["spanish: src/spanish.rs"]
  RU --> A["authors: src/russian/authors.rs"]
  M["main.rs: use hello::english, russian, spanish"] -.->|uses| R
@@code rust | Try it: modules, pub, use and super in one file
mod conveyor {
    pub const MAX_SPEED_MM_S: u32 = 500;

    pub fn clamp_speed(requested_mm_s: u32) -> u32 {
        if requested_mm_s > MAX_SPEED_MM_S {
            MAX_SPEED_MM_S
        } else {
            requested_mm_s
        }
    }

    fn log(text: &str) {
        println!("[conveyor] {text}"); // private: only this module and its children can call it
    }

    pub mod drive {
        pub fn start() {
            super::log("drive started"); // `super::` is the parent module
        }
    }
}

mod stacker {
    pub fn start() {
        println!("[stacker] started");
    }
}

use conveyor::drive;

fn main() {
    println!("Allowed speed: {} mm/s", conveyor::clamp_speed(650));
    drive::start();
    stacker::start(); // same function name as drive::start, no clash
    // conveyor::log("hello"); // ❌ E0603: function `log` is private
}
// Output:
// Allowed speed: 500 mm/s
// [conveyor] drive started
// [stacker] started
@@callout warning | Watch out
Two things trip everyone. First, you wrote a function in a new module and cannot call it from outside: **E0603**, *function is private*. Add `pub`. Second, you created `src/sound.rs` but nothing happens, or you get **E0433**, *cannot find `sound` in `crate`*: the file is not part of the program until a `mod sound;` line names it, usually in `lib.rs` or `main.rs`. If you do write `mod sound;` and the file is missing, you get **E0583**, *file not found for module*.
@@text Lecture code
These are the five files of the course's `hello` example crate (`examples/hello/src` in the course repository), shown unchanged. The first line `// no-check` in each box is ours. It tells our snippet checker that the file cannot be compiled alone.
@@code rust | Lecture code — examples/hello/src/lib.rs (MIT © Nathan Stocks)
// no-check
pub mod russian;
pub mod spanish;

pub mod english {
    pub fn greet() {
        println!("Hi!");
    }
}
@@code rust | Lecture code — examples/hello/src/main.rs (MIT © Nathan Stocks)
// no-check
use hello::{english, russian, spanish};

fn main() {
    english::greet();
    spanish::greet();
    russian::greet();
}
@@code rust | Lecture code — examples/hello/src/russian.rs (MIT © Nathan Stocks)
// no-check
pub mod authors;

pub fn greet() {
    println!("Привет!");
}
@@code rust | Lecture code — examples/hello/src/russian/authors.rs (MIT © Nathan Stocks)
// no-check
pub const DOSTOEVSKY: &'static str = "Достоевский";
@@code rust | Lecture code — examples/hello/src/spanish.rs (MIT © Nathan Stocks)
// no-check
pub fn greet() {
    println!("¡Hola!");
}
@@code bash | Run it
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\examples\hello
cargo run
@@text What to look for
The program prints three greetings, in the order `main.rs` calls them: English, Spanish, Russian. The `hello` crate uses `rand` as a dependency, so the first build downloads it. Open `russian.rs` and see how it declares `authors` in its own folder.
@@quiz Check yourself
? Items in a module are...
- public by default
+ private by default, and opened with `pub`
- visible only in `main.rs`
= Rust's default is closed. You decide what goes out, which keeps the inside free to change.
? You created `src/sound.rs`, but the compiler says `cannot find sound in crate`. What is missing?
- `use sound;` in the file
+ A line `mod sound;` in `lib.rs` or `main.rs`
- A `pub` on the file
= A file only becomes a module when a `mod` line names it.
? What does `super::log(...)` call?
+ The `log` function in the parent module
- A built-in logging function
- A function in the standard library
= `super` is the module one level up. Child modules can reach their parents' private items.
@@chapter 1.2.9 | Lesson 2.9 - Exercise C - Module System | 21
Exercise C: move constants into a library with `lib.rs`, then create a `sound` module in its own file.
@@objectives
- Create `src/lib.rs` and move items into it with `pub`
- Create a module in its own file and call it from `main.rs`
- Read E0603 and E0433 and know what they mean
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 2.9 - Exercise C - Module System · 4:27 — attach the MP4 here when you have it.
@@callout note | Plant situation
The palletiser tool has three constants and three functions in `main.rs`: `FIRST`, `SECOND`, `THIRD`, and `dog`, `cat`, `fox`. They are test names, but the same thing happens with real code: the file grows until nobody finds anything. Your job is a refactor that must not change what the program prints. In a PLC project that is moving POUs into folders and checking that the machine runs the same. The output is your acceptance test.
@@text Exercise brief
**Goal.** Reorganise a small program into a library and a module without changing its output. After every step you run it and the output must be the same.

Steps:

1. **A library.** Create `src/lib.rs`. Move the three constants `FIRST`, `SECOND` and `THIRD` into it and add `pub` in front of each. In `main.rs` add a `use` line so the constants are in scope. The library's name is the package name in `Cargo.toml`, which is `animal`.
2. **A module in a file.** In `lib.rs` add `pub mod sound;`. Create `src/sound.rs`. Move the functions `dog`, `cat` and `fox` into it and make them `pub`. In `main.rs` bring `sound` into scope and call `sound::dog()` and so on.
3. **Challenge 1.** Split `sound` into submodules: `dog` and `cat` in `animal::sound::tame`, `fox` in `animal::sound::wild`.
4. **Challenge 2.** Add an `animal::prelude` module that re-exports everything, so `main.rs` needs only `use animal::prelude::*;`.

The output at every stage is:
`Listening to animal 1: Dog goes WOOF!`, then the same for animals 2 (cat) and 3 (fox).
@@code rust | Starter code — exercise/c_modules/src/main.rs (MIT © Nathan Stocks)
// 1. Organize code into a library without changing the output of the program.
//
// For each step of this exercise, you should be able to run the program before and after your
// change without affecting the output of the program.
//
// Move the constants below (FIRST, SECOND, and THIRD) into the library:
// - Create a `src/lib.rs` file
// - Move all of the constants into lib.rs
// - Make the constants public by adding the `pub` keyword in front of them
// - Add `use` statement(s) to main.rs to bring the constants into scope.
//
// Hint: the name of the library is defined in Cargo.toml

const FIRST: i32 = 1;
const SECOND: i32 = 2;
const THIRD: i32 = 3;

// 2. Create a library module named `sound` and move the animal functions into it.
//
// - In your lib.rs file add the line `pub mod sound;`
// - Create a `src/sound.rs` file for your module
// - Move the `dog`, `cat`, and `fox` functions into sound.rs
// - Make the functions public by adding the `pub` keyword in front of them
// - Add a `use` statement to bring the `sound` module into scope.
// - Change the function calls to access the functions through the `sound` module.
//   For example: sound::dog()

fn dog() {
    println!("Dog goes WOOF!");
}

fn cat() {
    println!("Cat goes MEOW!");
}

fn fox() {
    println!("What does the fox say???");
}

fn main() {
    print!("Listening to animal {}: ", FIRST);
    dog();

    print!("Listening to animal {}: ", SECOND);
    cat();

    print!("Listening to animal {}: ", THIRD);
    fox();
}

// Challenge 1
//
// - Move the `dog` and `cat` functions into a submodule `animal::sound::tame`
// - Move the `fox` function into a submodule `animal::sound::wild`
//
// Hint: You will need to create a subdirectory for the top-level `sound` modules' submodules to
// be placed in.

// Challenge 2
//
// Create an `animal::prelude` module which re-exports all of the constants and functions of the
// library. (A real library would only re-export the most commonly-used items in its prelude.)
//
// Change your `use` statement(s) in main.rs to just `use animal::prelude::*`
//
// Hint: You will need `pub use` to re-export an item, for more details see:
// https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
@@code bash | Run it
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\c_modules
cargo run
# To keep the clone clean, copy the folder first and work in the copy:
#   Copy-Item -Recurse D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\c_modules D:\RustroverProjects\learn_rust\mine\c_modules
@@callout tip | Hints
1. A `src/lib.rs` next to `main.rs` is picked up by Cargo on its own. The library gets the package name, `animal`, and `main.rs` uses it like any other crate: `use animal::FIRST;`.
2. If the compiler says a constant is **private** (E0603), you forgot `pub` on it in `lib.rs`.
3. `pub mod sound;` goes into `lib.rs`. The functions go into the file `src/sound.rs`, with `pub fn` in front of each.
4. In `main.rs`, `use animal::sound;` and then `sound::dog();`. Run after every step: the output must not change.
5. Challenge 1: `sound.rs` declares `pub mod tame;` and `pub mod wild;`, and the files go in a folder `src/sound/`.
@@code rust | Solution — try it yourself first
// no-check
// Our solution, with the files of the finished project one after another.
// Output of the program at every stage:
//   Listening to animal 1: Dog goes WOOF!
//   Listening to animal 2: Cat goes MEOW!
//   Listening to animal 3: What does the fox say???

// ===== Steps 1 and 2 =====

// ---- src/lib.rs ----
pub mod sound;

pub const FIRST: i32 = 1;
pub const SECOND: i32 = 2;
pub const THIRD: i32 = 3;

// ---- src/sound.rs ----
pub fn dog() {
    println!("Dog goes WOOF!");
}

pub fn cat() {
    println!("Cat goes MEOW!");
}

pub fn fox() {
    println!("What does the fox say???");
}

// ---- src/main.rs ----
use animal::sound;
use animal::{FIRST, SECOND, THIRD};

fn main() {
    print!("Listening to animal {}: ", FIRST);
    sound::dog();

    print!("Listening to animal {}: ", SECOND);
    sound::cat();

    print!("Listening to animal {}: ", THIRD);
    sound::fox();
}

// ===== Challenges 1 and 2 (these files replace the ones above) =====

// ---- src/lib.rs ----
pub mod sound;

pub const FIRST: i32 = 1;
pub const SECOND: i32 = 2;
pub const THIRD: i32 = 3;

pub mod prelude {
    pub use crate::sound::tame::{cat, dog};
    pub use crate::sound::wild::fox;
    pub use crate::{FIRST, SECOND, THIRD};
}

// ---- src/sound.rs ----
pub mod tame;
pub mod wild;

// ---- src/sound/tame.rs ----
pub fn dog() {
    println!("Dog goes WOOF!");
}

pub fn cat() {
    println!("Cat goes MEOW!");
}

// ---- src/sound/wild.rs ----
pub fn fox() {
    println!("What does the fox say???");
}

// ---- src/main.rs ----
use animal::prelude::*;

fn main() {
    print!("Listening to animal {}: ", FIRST);
    dog();

    print!("Listening to animal {}: ", SECOND);
    cat();

    print!("Listening to animal {}: ", THIRD);
    fox();
}
@@callout warning | Watch out
Two errors belong to this exercise. If a constant or function is not marked `pub`, `main.rs` stops with **E0603**: the item is private. And if you create `src/sound.rs` but forget `pub mod sound;` in `lib.rs`, the line `use animal::sound;` stops with **E0432**, *unresolved import `animal::sound`*. The same code appears if `src/lib.rs` does not exist at all: then `animal` itself is the unresolved import. Also check that you changed the calls in `main.rs` to `sound::dog()`. If the old functions are still in `main.rs`, the program compiles and runs the old ones, and you have learned nothing.
@@quiz Check yourself
? In Exercise C, what is the library called when you write `use animal::FIRST;`?
- `main`, the name of the file
+ `animal`, the package name in `Cargo.toml`
- `lib`, the name of the file
= The library crate takes the package name from `Cargo.toml`.
? `main.rs` says `item FIRST is private`. What is missing?
- `use` in `lib.rs`
+ `pub` in front of the constant in `lib.rs`
- `mut` in front of the constant
= Items are private to their module until marked `pub`.
? Where does the file for the module `sound::tame` go?
- `src/tame.rs`
+ `src/sound/tame.rs`, with `pub mod tame;` in `sound.rs`
- `src/sound/mod.tame`
= Submodules of a file module live in a folder named after the module.
@@chapter 1.2.10 | Module 2 - Review | 4
The one-page summary of Module 2: Cargo, variables, scope, memory safety, functions and modules.
@@objectives
- Recall the key rule of each lesson in Module 2
- Check yourself against the cheat sheet before taking the quiz
@@attachment Ardan review sheet (PDF) | Module 2 - Review — download it from the Ardan course page and attach it here.
@@text Cheat sheet
| Topic | One-line meaning | PLC equivalent |
|---|---|---|
| Lesson 2.1 - Cargo | `cargo new`, `check`, `run`, `build --release`; `Cargo.toml` lists crates, `Cargo.lock` freezes their versions | CODESYS project, Library Manager, frozen library versions |
| Lesson 2.2 - Variables | `let` is fixed, `let mut` can change, `const` is known at build time; a new `let` shadows | `VAR CONSTANT`, `VAR`, `VAR_GLOBAL CONSTANT` |
| Lesson 2.3 - Scope | A name lives from its `let` to the closing brace; a block's last expression is its value | `VAR_TEMP` |
| Lesson 2.4 - Memory Safety | Indexing is checked, no unset reads, and ownership prevents the rest | Bounds checking, variable initialisation |
| Exercise A - Variables | `let`, `mut` and `const` in a tiny program; E0384 and E0070 | |
| Lesson 2.6 - Functions | Typed parameters, `->` return type, last expression is the result | `FUNCTION` |
| Exercise B - Functions | Shadow, fix scope, write `add`, set a variable on every path | |
| Lesson 2.8 - Module System | `mod`, `pub`, `use`, `lib.rs` and `main.rs` | POU folders, namespaces, library and application |
| Exercise C - Module System | Move code into `lib.rs` and a file module | Moving POUs into folders |
@@code mermaid | Module map
flowchart LR
  A["Cargo: project and build"] --> B["Variables: let, mut, const"]
  B --> C["Scope: braces"]
  C --> D["Memory safety: checks"]
  D --> E["Functions: fn and return"]
  E --> F["Modules: mod, pub, use"]
  F --> G["Module 3: Types and control flow"]
@@callout tip | Before you take the quiz
- Without the notes: which keyword makes a value changeable, and what error do you get if you forget it?
- A function ends with `a + b;` and the compiler complains about types. What is wrong, and what is the error code?
- You added `src/sound.rs`, but the compiler cannot find `sound`. Which line is missing, and in which file?
@@chapter 1.2.11 | Module 2 - Quiz | 8
Ten questions on Module 2: Cargo, variables, scope, memory safety, functions and modules.
@@objectives
- Check that you can recall the ideas of Module 2 without the notes
@@text How this quiz works
Pick one answer per question. The explanation under each question appears after you answer, right or wrong. Read them all, and go back to the lesson for any you missed.
@@quiz Module 2 - Quiz
? You build the DNC tool in March and again in October and get different behaviour. Which file prevents this when it is committed with your code?
- `Cargo.toml`
+ `Cargo.lock`
- `main.rs`
= `Cargo.lock` records the exact versions used by the last build. `Cargo.toml` only gives ranges, such as `"0.8"`.
? Which command gives the quickest answer to "is my code valid?"
+ `cargo check`
- `cargo build --release`
- `cargo new`
= `cargo check` runs the compiler's checks and skips producing the program.
? `let speed = 250;` followed by `speed = 300;` gives an error. Which one?
- E0425
+ E0384
- E0308
= E0384 means you assigned twice to an immutable variable. `let mut speed` fixes it.
? What is true about `const MAX_SPEED_MM_S: i32 = 500;`?
- The type can be left out
- It can be changed with `mut`
+ The type is required and the value is known when the program is built
= A constant is computed by the compiler. It cannot change at run time.
? After a block `{ let x = 5; }`, the next line uses `x`. What happens?
- It prints 5
+ Error E0425, cannot find value `x` in this scope
- It prints 0
= The variable ends at the closing brace of its block.
? Which of these is a statement that gives no value?
- `a + b`
+ `let x = 5;`
- `{ 5 }`
= `let` does something but produces no value. `a + b` and `{ 5 }` are expressions.
? `fn sum(a: i32, b: i32) -> i32 { a + b; }` fails. Why?
- `sum` is a reserved word
+ The semicolon makes the body produce nothing, which is E0308
- The parameters need `mut`
= Without the semicolon, `a + b` is the return value.
? A table of ten entries is indexed with a run-time value of 12 using `table[12]`. What happens?
- It reads the next variable in memory
- It returns 0
+ The program panics with an index-out-of-bounds message
= Indexing is always checked. Use `.get()` if you want to deal with it yourself.
? Items inside a module are...
+ private unless marked `pub`
- public unless marked `private`
- always visible inside the same file
= Privacy is the default, which lets a library change its inside freely.
? You added `src/sound.rs` but the compiler says it cannot find `sound`. What is missing?
- `pub fn` in `main.rs`
- A `[dependencies]` entry
+ A `mod sound;` line in `lib.rs` or `main.rs`
= A file is only part of the program when a `mod` line names it.
