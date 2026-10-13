@@part 1 · Ultimate Rust Crash Course
@@section Module 4: The Heart of Rust
@@chapter 1.4.1 | Lesson 4.1 - Ownership | 10
Every value in Rust has exactly one owner, and when the owner goes out of scope the value is cleaned up. Learn moves, `Copy`, `clone` and why the compiler refuses to let you use a value after you handed it away.
@@objectives
- State the three ownership rules in your own words
- Explain what a move is and why the old name stops working
- Choose between a move, a `Copy` and a `.clone()`
- Read error E0382 and name three ways to fix it
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 4.1 - Ownership · 6:07 — attach the MP4 here when you have it.
@@callout note | Plant situation
A vendor DLL on the DNC server keeps G-code programs in buffers it allocates itself. The sender task passes the buffer pointer to the logger task so that it can write "sent" into the program header. The sender finishes first and releases the buffer. The logger then writes into memory that is already free, and about once a month the next program reaches the mill with a wrong coordinate in one block. You can do the same with `__NEW` and `__DELETE` in CODESYS: two POUs hold one pointer and nobody knows who releases it. Rust answers with a rule the compiler checks: every value has one owner, and the owner is the only one who releases it.
@@text Teacher's notes
### Where values live
A number such as `let station = 7;` has a fixed size. It lives in the working area of the function, like a `VAR_TEMP`, and vanishes when the function ends. Text is different: a `String` can grow, so its characters live in separate memory that the program asks for while it runs (the **heap**, the place `__NEW` takes from). The `String` variable itself is only a small record: where the text is, how long it is, how much room it has. Whoever holds that record is responsible for the text. The question for every language is: who releases that memory, and when?

### The three rules
1. Every value has an **owner**, a variable.
2. There is **one owner at a time**.
3. When the owner goes out of scope, the value is **dropped**: its memory is released.

Think of one rung driving one coil. Two rungs on the same coil is the classic double-coil bug: the last rung silently wins. Two owners of one buffer is the same kind of bug, and Rust does not let it be written.

### Drop: cleanup you never write
At the closing brace Rust releases every value that the block still owns, in the reverse order of declaration. There is no `free` call to forget and none to call twice. The compiler puts the cleanup at a place it knows at build time. This is the "on stop, de-energise outputs" idea: cleanup that runs by itself.

### Move: the old name dies
`let backup = program;` does not copy the text. The small record changes hands, `backup` is the owner now, and `program` is dead: the compiler will not let you use it. If both names stayed valid, both would release the text at the end of the scope, and you would be back at the plant situation. The error comes on the line where you use the dead name, not where you moved: **E0382**, *borrow of moved value*. The message points at both lines, with *value moved here* and *value borrowed here after move*.

### Passing to a function is a move too
`send_to_machine(program)` makes the parameter the new owner, and the text is dropped when that function ends. The caller can no longer use `program`. A function can hand a value back as its return value, as `add_end_marker` does in the example, but doing this for every call is clumsy. Lesson 4.2 shows the better way, borrowing.

### `Copy`: small values are simply duplicated
Integers, floats, `bool`, `char` and tuples made only of those have a fixed size and own nothing that must be released. Assigning them copies the bits and both names stay valid. They are marked **`Copy`**. It is the same as a `BOOL` or `INT` going into a `VAR_INPUT` by value.

### `.clone()`: a second, separate value
When you really need two independent owners, write `.clone()`. It allocates new memory and copies the text, which costs time. Rust makes you write it so that the cost is visible in the code, like copying a recipe data set.

### Why Rust is built this way
Other languages pick one of two answers. A garbage collector releases memory at moments you do not choose, which is awkward when a scan cycle must finish in 10 ms. Manual release gives you the bug above. Rust checks the ownership rules while compiling and then adds nothing at run time.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| One rung drives one coil, no double coil | One value, one owner | An owner can change: a move hands the value to a new owner |
| Handing a pallet to the next station | A move: `let b = a;` or `f(a)` | The compiler stops you reaching for the pallet afterwards |
| `BOOL` or `INT` into `VAR_INPUT` by value | A `Copy` type | Only small, fixed-size values qualify |
| Copying a recipe data set | `.clone()` | It costs time and memory, so you have to write it |
| `__NEW` and `__DELETE` on a pointer | An owner that drops its value automatically | There is no `__DELETE` to forget or to call twice |
| On stop, de-energise outputs | Drop at the end of the scope | It also runs when the function leaves early with `return` |
@@code mermaid | Diagram: what happens when you assign or pass a value
flowchart TD
  A["let program = String::from(...)"] --> B["program owns the text"]
  B --> C["let backup = program  or  send(program)"]
  C --> D["Is the type Copy? i32, f64, bool, char"]
  D -->|yes| E["Both names hold their own value"]
  D -->|no, like String| F["Ownership moves to the new owner"]
  F --> G["Old name is dead: E0382 if used"]
  F --> H["New owner drops the text at the end of its scope"]
  B --> I["backup = program.clone() makes a second owner"]
@@code rust | Try it: move, Copy, clone and drop
// Sends a G-code program to the mill and hands the text over for good.
fn send_to_machine(program: String) -> usize {
    println!("Sending {program}");
    program.len()
} // `program` goes out of scope here: its memory is released

// Takes the text, adds a line, gives the text back to the caller.
fn add_end_marker(mut program: String) -> String {
    program.push_str(" M30");
    program
}

fn main() {
    // Copy type: a plain number is duplicated bit for bit, both names stay valid.
    let station_a = 7;
    let station_b = station_a;
    println!("stations {station_a} and {station_b}");

    // String: owns heap memory, so assignment MOVES it.
    let program = String::from("O1001 G90 G54");
    let backup = program.clone(); // a second, separate copy
    let program = add_end_marker(program); // ownership goes in and comes back out
    let sent_bytes = send_to_machine(program); // ownership goes in and stays there
    // println!("{program}"); // ❌ E0382: borrow of moved value: `program`

    println!("sent {sent_bytes} bytes, the backup still reads: {backup}");
}
// Output:
// stations 7 and 7
// Sending O1001 G90 G54 M30
// sent 17 bytes, the backup still reads: O1001 G90 G54
@@callout warning | Watch out
You pass a `String` to a function, then print it on the next line: **E0382**, *borrow of moved value*. There are three fixes. Return the value from the function and take it back. Pass `.clone()` if the function really needs its own copy. Or, usually best, let the function borrow it (Lesson 4.2). Do not add `.clone()` just to silence the compiler: it works, but it hides a design question, and on a big recipe or a long G-code file it copies every byte.
@@text Exercise
Five minutes, in a scratch project.

1. Paste the Try it code and run it. Then remove the `//` in front of the `println!("{program}")` line and read the full error, including the two lines it points at.
2. Change `station_a` to `String::from("S7")`. The line `let station_b = station_a;` now moves it, and the error appears on the `println!` that uses `station_a`.
3. Fix step 2 with `.clone()` and say, in one sentence, why the original `i32` needed no clone.
@@quiz Check yourself
? You write `let a = String::from("valve"); let b = a;`. What happens to the text?
- It is copied, so `a` and `b` both work
+ Ownership moves to `b`, and `a` can no longer be used
- Both names share it, and the last one to finish releases it
= A `String` owns its memory, so assignment moves it. If both names stayed valid, both would release the same text.
? When is the memory of a local `String` released?
- When you call `.free()` on it
- When the whole program ends
+ When its owner goes out of scope, automatically
= Rust inserts the cleanup at the closing brace of the block that owns the value. There is no manual release.
? What does `.clone()` give you, and what does it cost?
- A reference to the same value, at no cost
+ A second independent value, at the cost of copying the data
- A faster version of the value
= Clone creates a new owner with its own memory. That is why Rust makes you write it out.
@@chapter 1.4.2 | Lesson 4.2 - References & Borrowing | 9
Use a value without taking it: `&` for read-only access, `&mut` for one writer, the rule that keeps them apart, and why a reference can never point at something that is gone.
@@objectives
- Pass a value to a function with `&` and with `&mut` and keep ownership
- State the rule: many readers or one writer, never both
- Read E0499, E0502 and E0596 and fix each one
- Explain why returning a reference to a local variable cannot compile
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 4.2 - References & Borrowing · 5:22 — attach the MP4 here when you have it.
@@callout note | Plant situation
On a conveyor sorter the counter `rejects_today` is a `VAR_IN_OUT` of two function blocks: the vision check adds one for every bad part, and a shift-end block resets it. Later someone moves the reset into a second task. Both blocks are correct on their own, yet on Monday mornings the shift report shows 3 rejects too few, and nobody can reproduce it. The PLC lets any number of blocks hold a write path to one tag. Rust does not: while one writer is active nobody else may read or write the value, and the compiler checks that before the program exists.
@@text Teacher's notes
### Lend it, don't give it away
Lesson 4.1 ended with a clumsy pattern: pass a value in, get it back out. A **reference** is the better tool. It lets a function use a value while the caller stays the owner. The owner still drops the value at the end of its scope, and the function only borrows it for a while. Rust calls this **borrowing**.

### `&T`: a shared reference, read only
`report(&rejects)` hands over a read-only view. The function signature says so: `fn report(rejects: &i32)`. You can have as many shared references to one value as you like. Writing through one is refused: **E0594**, *cannot assign to `*rejects`, which is behind a `&` reference*. It is a read-only `VAR_IN_OUT CONSTANT`.

### `&mut T`: an exclusive reference, one writer
`count_reject(&mut rejects)` hands over a write path. Three places show it:

- the owner must be declared `let mut`, else **E0596**, *cannot borrow as mutable*;
- the call site writes `&mut`;
- the signature says `rejects: &mut i32`.

Anyone reading the call sees at once that this function may change the value. To reach the value inside the reference you write `*rejects += 1`. The `*` means "follow the reference". Method calls such as `name.push_str("s")` follow it for you.

### The rule: many readers or one writer
For one value at one moment you may have **any number of `&`** or **exactly one `&mut`**, and never both. Two writers give **E0499**, *cannot borrow as mutable more than once at a time*. A reader next to a writer gives **E0502**.

Why refuse a reader next to a writer? Take `let first = &log[0];` and then `log.push(4);`. A growing `Vec` may move all its elements into a bigger block of memory. `first` would then point at memory that was released. The compiler sees the danger and refuses, with E0502, as long as `first` is used afterwards.

### A borrow lasts until its last use
A reference does not live to the closing brace. It lives until the last line that uses it. In the Try it below the two readers are finished before the writer starts, so the program is accepted. This is why most real code needs no special tricks.

### Dangling references cannot be written
A reference must never outlive the value it points at. Three variants of the same mistake, all refused at build time:

- returning `&local` from a function: **E0515**, *cannot return reference to local variable*;
- a function that returns `&String` but has nothing to borrow from: **E0106**, *missing lifetime specifier*;
- storing a reference to a value from an inner block and using it after the block: **E0597**, *does not live long enough*.

The fix is the same: return the owned value, so that it moves out. "Lifetime" is the name for how long a reference may be used, and the course comes back to it later. For now, ask: who owns this value, and does the owner live long enough?

### `&String` or `&str`
Module 3 showed `&str`, a read-only view of text. Prefer it for text parameters: when a function wants a `&str` you may pass a `&String`. The exercise in the next lesson uses `&String` because its task text asks for it.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| Read-only `VAR_IN_OUT CONSTANT` or `REFERENCE TO` | `&T` | Any number at once, and the compiler enforces read-only |
| `VAR_IN_OUT` with write access | `&mut T` | Only one at a time. A PLC lets several blocks hold it |
| Two blocks writing one tag | E0499, refused | A PLC runs it and you debug it on the machine |
| HMI trend reads a buffer while the task rewrites it | E0502, refused | Rust refuses it even in single-threaded code |
| `ADR(x)`, the address of a variable | `&x` | A reference cannot be null, cannot go stale and has no arithmetic |
| Pointer to a `VAR_TEMP` that no longer exists | A dangling reference, refused at build time | No lifetime tracking exists in a PLC compiler |
@@code mermaid | Diagram: borrows take turns
sequenceDiagram
  participant O as Owner rejects
  participant R as report function
  participant W as count_reject function
  O->>R: lend a shared borrow
  R-->>O: borrow ends
  O->>W: lend an exclusive borrow
  W-->>O: borrow ends
  Note over O,W: Readers may overlap each other. A writer is always alone.
@@code rust | Try it: report with &, change with &mut
// Reads only: a shared borrow, many of these may exist at once.
fn report(rejects: &i32, station: &str) {
    println!("{station}: {rejects} rejects so far");
}

// Writes: an exclusive borrow, only one may exist at a time.
fn count_reject(rejects: &mut i32) {
    *rejects += 1; // `*` follows the reference to the value itself
}

fn main() {
    let mut rejects = 0;

    count_reject(&mut rejects);
    count_reject(&mut rejects);
    report(&rejects, "Sorter 1");

    // Many readers at the same time are fine.
    let reader_a = &rejects;
    let reader_b = &rejects;
    println!("two readers see {reader_a} and {reader_b}");

    // The readers are finished (last use above), so a writer may start now.
    let writer = &mut rejects;
    *writer += 10;
    // println!("{reader_a}"); // ❌ E0502: reader_a would still be alive next to the writer
    println!("after the writer: {rejects}");
}
// Output:
// Sorter 1: 2 rejects so far
// two readers see 2 and 2
// after the writer: 12
@@callout warning | Watch out
The most common one is the `Vec` case: `let first = &log[0]; log.push(4); println!("{first}");` gives **E0502**. Finish using `first` before the `push`, or copy the number out with `let first = log[0];`, which is fine for a `Copy` type. If you see **E0596** when you write `&mut x`, you forgot `let mut x`. Do not fight E0499 with `.clone()` everywhere. Shorten the time each borrow lives, so the borrows take turns.
@@text Exercise
Five minutes, in a scratch project.

1. Paste the Try it code and run it. Remove the `//` on the `println!("{reader_a}")` line and read the E0502 message.
2. Change `fn count_reject(rejects: &mut i32)` to `rejects: &i32`, and read the error you get on the `*rejects += 1` line. (It is E0594.)
3. Write `fn reset(rejects: &mut i32)` that sets the counter to 0 with `*rejects = 0;` and call it after the report.
@@quiz Check yourself
? How many shared references (`&`) to one value may exist at the same time?
- Exactly one
+ Any number
- None, a value cannot be shared
= Readers cannot hurt each other. The limit only appears when a writer is involved.
? A function has the parameter `count: &mut i32`. What do you write in the call?
- `bump(count)`
- `bump(&count)`
+ `bump(&mut count)`, and `count` was declared `let mut`
= The `&mut` appears in the signature and at the call site, and the owner must be `mut`. The call tells the reader that the value may change.
? What does `*count += 1` do when `count` is a `&mut i32`?
- Multiplies the reference by one
+ Follows the reference and adds 1 to the value it points at
- Creates a new reference to the number 1
= The `*` goes through the reference to the value, so the owner's number changes.
@@chapter 1.4.3 | Lesson 4.3 - Exercise G - Ownership & References | 22
Exercise G: write three functions with three different relationships to a String: look at it with `&`, change it with `&mut`, and consume it by taking ownership. A command-line argument feeds the program.
@@objectives
- Write a function that takes `&String` and one that takes `&mut String`
- Write a function that takes ownership and see why the caller loses the value
- Replace the whole content behind a `&mut` with `*`
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 4.3 - Exercise G - Ownership & References · 5:29 — attach the MP4 here when you have it.
@@callout note | Plant situation
The carton labeller prints 120 labels a minute from the part name the operator types at the HMI. Three routines touch that one text. The checker only looks at it and says whether it is plural, as in `valve` or `valves`. The fixer edits the operator's own text in place so that the label always reads `valves`. The print queue takes the name and keeps it, so after it nobody else may use the text. Looking, editing, handing over: three different promises, and in Rust each one is a different function signature.
@@text Exercise brief
**Goal.** Write three small functions that each treat a `String` differently, and watch the compiler enforce the difference. The program takes a word from the command line.

Steps:

1. **Look.** Write `inspect`. It takes a **shared reference** to a `String`, returns nothing, and prints whether the text is plural (ends with `s`) or singular. Uncomment the call `inspect(&arg);`.
2. **Edit.** Write `change`. It takes a **mutable reference** to a `String` and adds an `s` at the end unless there is one already. Uncomment the two lines that call it.
3. **Hand over.** Write `eat`. It takes **ownership** of the `String` and returns `true` if the text starts with `b` **and** contains an `a`. Uncomment the `if eat(arg)` block. Try `boat`, `banana` and `grapes`.
4. **Challenge.** Write `bedazzle`. It takes a `&mut String` and replaces the whole content with `sparkly`, ignoring what was in it. You need `*` for this one.

Run the program with a word after `cargo run`, for example `cargo run apple`. Without a word it prints a message and stops.
@@code rust | Starter code — exercise/g_ownership_references/src/main.rs (MIT © Nathan Stocks) | The file as it is in the course repository. Its line endings were converted to this page's, and nothing else changed.
// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    //inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    //change(&mut arg);
    //println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    //if eat(arg) {
    //    println!("Might be bananas");
    //} else {
    //    println!("Not bananas");
    //}

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    // let mut material = "mud".to_string();
    // println!("This material is just `{}`.", material);
    // bedazzle(&mut material);
    // println!("Wow! Now the material is `{}`!", material);
}
@@code bash | Run it
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\g_ownership_references
cargo run apple
cargo run apples
cargo run boat
cargo run banana
# To keep the clone clean, copy the folder first and work in the copy:
#   Copy-Item -Recurse D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\g_ownership_references D:\RustroverProjects\learn_rust\mine\g_ownership_references
@@text What to look for
With `apple` and all four steps done you should see `apple is singular`, `I have many apples` and `Not bananas`. With `banana`: `banana is singular`, `I have many bananas` and `Might be bananas`. Until you uncomment a call, the matching lines do not exist for the compiler, so you can add the steps one at a time and run after each. With the challenge done, the last two lines are always `This material is just `mud`.` and `Wow! Now the material is `sparkly`!`.
@@callout tip | Hints
1. Step 1: `fn inspect(name: &String)`. Call it as `inspect(&arg);`. Inside, `name.ends_with("s")` works straight through the reference.
2. Step 2: `fn change(name: &mut String)`. Call it as `change(&mut arg);`. Inside, `name.push_str("s")` adds the letter to the caller's String. `arg` is already declared `let mut` in the starter.
3. Step 3: `fn eat(name: String) -> bool`. Combine the two tests with `&&`. After `eat(arg)` the variable `arg` is gone, so any `println!` that uses it below is **E0382**. That is the lesson, not a bug.
4. Challenge: `*material = String::from("sparkly");`. Without the `*` you try to replace the reference itself and get **E0308**, *mismatched types*.
@@code rust | Solution — try it yourself first
// Our solution to Exercise G: look, edit, hand over.
#![allow(unused_mut, unused_variables)]

// 1. A shared reference: the function may read the name, never change or keep it.
fn inspect(name: &String) {
    if name.ends_with("s") {
        println!("{name} is plural");
    } else {
        println!("{name} is singular");
    }
}

// 2. An exclusive reference: the function may change the caller's String in place.
fn change(name: &mut String) {
    if !name.ends_with("s") {
        name.push_str("s");
    }
}

// 3. Ownership: the String moves in and is dropped when the function ends.
fn eat(name: String) -> bool {
    name.starts_with("b") && name.contains("a")
}

// Challenge: `*` reaches through the reference to the String itself, so we can replace it whole.
fn bedazzle(material: &mut String) {
    *material = String::from("sparkly");
}

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }
    // println!("{}", arg); // ❌ E0382: borrow of moved value: `arg`, because `eat` consumed it

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}
// Output (for `cargo run banana`):
// banana is singular
// I have many bananas
// Might be bananas
// This material is just `mud`.
// Wow! Now the material is `sparkly`!
@@callout warning | Watch out
Most of the errors here are about the little `&`. Passing `arg` where `&String` is expected, or `&arg` where `&mut String` is expected, gives **E0308**, *mismatched types*. `&mut arg` on a variable that is not `let mut` gives **E0596**. Calling `push_str` inside a function that only has `&String` also gives **E0596**, *behind a `&` reference*: read-only means read-only. And after `eat(arg)` the variable is dead, so `println!("{}", arg)` gives **E0382**. Do not "fix" it with `.clone()`. Ask first whether `eat` should really consume its argument.
@@quiz Check yourself
? Which signature promises "I will only look at the text and not change or keep it"?
- `fn inspect(name: String)`
+ `fn inspect(name: &String)`
- `fn inspect(name: &mut String)`
= A shared reference gives read access and nothing else, and the caller keeps ownership. `String` by value would take the text away, and `&mut` would allow changes.
? After `if eat(arg) { ... }`, the next line prints `arg`. What does the compiler say?
- Nothing, `eat` only reads the text
- E0596: `arg` must be declared `mut`
+ E0382: borrow of moved value, `eat` took ownership
= `eat` takes a `String` by value, so the text moved into it and was dropped at the end of `eat`. `arg` has no value left.
? In `fn bedazzle(material: &mut String)` you write `material = String::from("sparkly");` without the `*`. What is wrong?
+ You are assigning a `String` to the reference variable itself, which is E0308, mismatched types
- Nothing, it replaces the caller's text
- References cannot be assigned to at all
= The variable `material` holds a reference. Only `*material = ...` writes to the String it points at.
@@chapter 1.4.4 | Module 4 - Review | 4
The one-page summary of Module 4: ownership, move, `Copy`, `clone`, drop, shared and exclusive references, and dangling references.
@@objectives
- Recall the key rule of each lesson in Module 4
- Check yourself against the cheat sheet before taking the quiz
@@attachment Ardan review sheet (PDF) | Module 4 - Review — download it from the Ardan course page and attach it here.
@@text Cheat sheet
| Topic | One-line meaning | PLC equivalent |
|---|---|---|
| Lesson 4.1 - Ownership: the rules | One owner per value, and the value is dropped when its owner leaves its scope | One rung, one coil. On stop, de-energise outputs |
| Lesson 4.1 - Move | `let b = a;` and `f(a)` hand a non-`Copy` value over, and the old name is dead (E0382) | Handing a pallet to the next station |
| Lesson 4.1 - `Copy` and `clone` | Small fixed-size types are copied automatically, anything else needs `.clone()` | `INT` into `VAR_INPUT` by value, copying a recipe data set |
| Lesson 4.2 - `&T` | Borrow to read. Any number at once | Read-only `VAR_IN_OUT CONSTANT` |
| Lesson 4.2 - `&mut T` | Borrow to write. One at a time, owner must be `mut`, follow it with `*` (E0499, E0596) | `VAR_IN_OUT` with write access, but only one writer |
| Lesson 4.2 - The rule | Many readers or one writer, never both (E0502). A borrow lasts to its last use | Static analysis that refuses two writers |
| Lesson 4.2 - Dangling references | A reference may not outlive its owner (E0515, E0597, E0106) | A pointer to a `VAR_TEMP` that is gone |
| Exercise G - Ownership & References | `inspect(&s)` looks, `change(&mut s)` edits, `eat(s)` consumes | Read, write and hand over |

The error codes of this module, in one place:

| Code | Message in short | Usual fix |
|---|---|---|
| E0382 | Borrow of moved value | Borrow instead, return the value, or `.clone()` |
| E0499 | Two `&mut` at once | Make the borrows take turns |
| E0502 | `&` and `&mut` at once | Finish using the reader first, or copy the value out |
| E0594 | Assign through a `&` | Take `&mut` in the signature |
| E0596 | Cannot borrow as mutable | `let mut` on the owner, or `&mut` in the signature |
| E0515, E0597, E0106 | A reference outlives its owner | Return the owned value |
@@code mermaid | Module map
flowchart TD
  A["Every value has one owner"] --> B["Assign or pass it: a move"]
  B --> C["Copy types and clone keep a second value"]
  B --> D["Or lend it instead"]
  D --> E["Shared borrow: many readers"]
  D --> F["Exclusive borrow: one writer"]
  E --> G["Never both at once"]
  F --> G
  G --> H["A reference never outlives its owner"]
@@callout tip | Before you take the quiz
- Without the notes: what are the three ownership rules, and which error do you get when you use a `String` after passing it to a function?
- You need to read a recipe in one function and add a step in another, and keep using it afterwards. Which two signatures do you write?
- `let first = &log[0]; log.push(4); println!("{first}");` is refused. Which error code, and in one sentence, what could go wrong if it were allowed?
@@chapter 1.4.5 | Module 4 - Quiz | 8
Ten questions on Module 4: ownership, move, `Copy`, `clone`, drop, `&`, `&mut` and dangling references.
@@objectives
- Check that you can recall the ideas of Module 4 without the notes
@@text How this quiz works
Pick one answer per question. The explanation under each question appears after you answer, right or wrong. Read them all, and go back to the lesson for any you missed.
@@quiz Module 4 - Quiz
? You write `let recipe = String::from("Cola 500"); let next = recipe; println!("{recipe}");`. What happens?
- It prints `Cola 500`
- It prints an empty line
+ It does not compile: E0382, borrow of moved value
= `next` owns the text now. The old name `recipe` may not be used again.
? Why does `let b = a;` leave `a` usable when `a` is an `i32`, but not when `a` is a `String`?
- Rust only copies values of fewer than 8 bytes
- An `i32` lives on the heap, and a `String` lives on the stack
+ An `i32` is `Copy`, so it is duplicated. A `String` owns memory that must be released once, so it moves
= Moving prevents two owners from releasing the same memory. Plain numbers own nothing, so copying them is safe.
? When does Rust release the memory of a `String` held by a local variable?
+ When the variable goes out of scope, at the closing brace of its block
- When you call `drop` on the program
- When the last line of `main` has run
= Cleanup is inserted by the compiler at the end of the scope, in reverse order of declaration. There is no manual release.
? `fn consume(batch: String)` is called as `consume(batch)`, and `batch` is still needed afterwards. Which call keeps it working?
- `consume(&batch)`
+ `consume(batch.clone())`
- `consume(batch.copy())`
= The function wants its own `String`, so you give it a clone. `&batch` has the wrong type for this signature, and `String` has no `copy` method. If the function only reads, changing its parameter to a reference is the cleaner fix.
? A function must add 1 to a counter that belongs to the caller. Which signature works?
- `fn bump(count: i32)`
- `fn bump(count: &i32)`
+ `fn bump(count: &mut i32)`
= By value, only a private copy changes. Through `&i32` writing is refused (E0594). `&mut i32` gives the one write path.
? Which of these does the borrow checker refuse?
- Two `&` references to one value at the same time
- A `&mut` created after the last use of an earlier `&`
+ A `&mut` created while an earlier `&` is still used afterwards
= Many readers are fine, and a borrow ends at its last use. A writer next to a reader that is still going to be used is refused with E0502.
? `let a = &mut rejects; let b = &mut rejects; *a += 1; *b += 1;` gives which error code?
- E0382
- E0502
+ E0499
= Two exclusive borrows alive at once is E0499. E0502 is a shared borrow together with an exclusive one.
? `let first = &log[0]; log.push(4); println!("{first}");` is refused. What could go wrong if it were allowed?
- `push` could never run on a `Vec` that has been read
+ The `Vec` may move its elements to a bigger block, and `first` would point at released memory
- `first` would become a copy of the whole `Vec`
= It is the same family as the plant situation in Lesson 4.1, using memory after it was released, and the compiler turns it into an error.
? A function builds `let label = format!("{station}-01");` and ends with `&label`, its return type being `&str`. What does the compiler say, and what is the fix?
- Nothing, it works as long as `station` is long enough
- E0382, add `.clone()` to the reference
+ E0515: it cannot return a reference to a local variable, so return the `String` itself
= `label` is dropped when the function ends, so a reference to it would dangle. Returning the `String` moves it out to the caller.
? In `fn bedazzle(m: &mut String)`, what does `*m = String::from("sparkly");` do?
- It changes only the local `m`, and the caller's text stays as it was
+ It replaces the caller's String with the new one
- It does not compile, because `*` only works on numbers
= The `*` follows the reference to the value behind it, so the owner's String is overwritten.
