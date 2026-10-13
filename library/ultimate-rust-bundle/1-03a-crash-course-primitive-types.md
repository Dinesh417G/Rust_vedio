@@part 1 · Ultimate Rust Crash Course
@@section Module 3: Primitive Types & Control Flow
@@chapter 1.3.1 | Lesson 3.1 - Scalar Types | 9
The single-value types of Rust: integers, floats, `bool` and `char`. Learn their sizes, how to write literals, what happens when a number does not fit, and how `as` converts.
@@objectives
- Choose the Rust integer or float type that matches a PLC type such as `INT`, `UDINT` or `LREAL`
- Write number literals with `_` separators, hex, binary and type suffixes
- Pick `checked_`, `wrapping_` or `saturating_` arithmetic for a counter that can overflow
- Predict the result of an `as` cast that truncates
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 3.1 - Scalar Types · 5:52 — attach the MP4 here when you have it.
@@callout note | Plant situation
A conveyor sorter reads its belt encoder on a 16-bit counter card. The encoder delivers about 2,000 counts a second, so the counter runs through 0 to 65,535 and back to 0 every 33 seconds. The sorter measures how far a part has travelled by subtracting the old count from the new one. Right after each wrap the difference comes out negative or huge, and the diverter fires at the wrong moment. Nobody sees it on the bench, where the belt creeps along and the counter never reaches the top during a test. Rust makes you say what you want when a number does not fit: stop, wrap on purpose, or stick at the limit.
@@text Teacher's notes
### Scalar means one value
A scalar type holds a single value. Rust has four families: integers, floating-point numbers, `bool` and `char`.

### Integers: the size is in the name
`i16` is a signed 16-bit integer, `u16` an unsigned one. The number is the bit count: 8, 16, 32, 64 or 128. Here is how they line up with the types you know: `i8` is `SINT`, `u8` is `USINT` or `BYTE`, `i16` is `INT`, `u16` is `UINT` or `WORD`, `i32` is `DINT`, `u32` is `UDINT` or `DWORD`, `i64` is `LINT`, `u64` is `ULINT` or `LWORD`. Two more, `isize` and `usize`, have the width of a pointer on the machine, which is 64 bits on your PC. Rust uses `usize` for array indexes and lengths, so you will meet it often.

If you write no type, a whole number is an `i32`, the `DINT` of Rust.

### Floats, `bool`, `char`
`f32` is `REAL` and `f64` is `LREAL`. A decimal number with no type is an `f64`. A `bool` is `true` or `false` and nothing else: `1` is not `true`, and `if 1 { ... }` is refused. A `char` is one Unicode character in single quotes, such as `'°'`, and it takes four bytes. It is not a `BYTE`. Text gets its own lessons later in this module.

### Writing numbers
Underscores are ignored, so `1_000_000` is easy to read. Prefixes pick the base: `0xFF` for hex, `0b1010_1010` for binary. ST writes the same as `16#FF` and `2#1010_1010`. A suffix fixes the type: `250u8`, `3.5f32`. A literal that does not fit its type is caught while building: `let x: u8 = 256;` stops with *literal out of range for `u8`*.

### No silent conversions
In ST you can add an `INT` to a `DINT` and the compiler widens one side for you. Rust never does that. `a + b` with an `i16` and an `i32` fails with **E0308**, *mismatched types* (together with E0277, *cannot add `i32` to `i16`*). You write the conversion. `i32::from(a)` is the safe widening. `a as i32` is the general one.

### What `as` does
`as` always converts, and it never complains.

- Widening keeps the value.
- Narrowing keeps only the low bits: `40_000_i32 as i16` is `-25536`, and `300_i32 as u8` is `44`.
- A float turned into an integer loses its decimals towards zero: `3.989_f64 as i32` is `3`. Call `.round()` first if you want rounding. A value too big for the target sticks at the limit.
- An integer turned into a float is how you start analog scaling: `raw as f64`.

### Overflow: the counter that wraps
Add 1 to a `u16` that holds 65,535 and the result does not fit. A plain `+` then depends on the build. A **debug** build panics with *attempt to add with overflow*. A **release** build wraps to 0 without a word, unless you set `overflow-checks = true` for it in `Cargo.toml`. Do not rely on either. Say what you mean:

- `checked_add` gives an `Option`: `Some(result)`, or `None` when it does not fit.
- `wrapping_add` wraps around on purpose.
- `saturating_add` stops at the largest (or smallest) value.

The same family exists for `sub` and `mul`. For the encoder, wrapping is exactly right: `now.wrapping_sub(previous)` gives the true distance across the wrap. In ST the same trick works with `UINT` arithmetic. And when both sides are plain constants, the compiler already refuses `255u8 + 1`.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| `SINT` `INT` `DINT` `LINT` and `USINT` `UINT` `UDINT` `ULINT` | `i8` `i16` `i32` `i64` and `u8` `u16` `u32` `u64` | `BYTE` and `WORD` are not separate types: a `u16` is a `WORD` |
| `REAL`, `LREAL` | `f32`, `f64` | A decimal literal with no suffix is `f64` |
| `BOOL` | `bool` | Only `true` or `false`. No `1` standing in for true |
| `16#FF`, `2#1010` | `0xFF`, `0b1010` | `_` may sit anywhere inside the number |
| `INT` plus `DINT` widens by itself | Refused: `i32::from(x)` or `x as i32` | You write every conversion |
| Integer overflow wraps silently | Debug: panic. Release: wraps. Or choose `checked_`, `wrapping_`, `saturating_` | The method name states the intent |
| `REAL_TO_INT(x)` | `x as i32` | Check what your runtime does; `as` cuts the decimals, so use `.round()` first |
@@code mermaid | Diagram: one u16 at its limit
flowchart TD
  A["u16 holds 65535, add 1"] --> B["Plain +, debug build: panic"]
  A --> C["Plain +, release build: wraps to 0"]
  A --> D["You choose the behaviour"]
  D --> E["checked_add: None"]
  D --> F["wrapping_add: 0"]
  D --> G["saturating_add: stays 65535"]
@@code rust | Try it: encoder wrap, overflow methods and casts
fn main() {
    // A 16-bit encoder card counts 0..=65_535 and then starts again at 0.
    let previous_count: u16 = 65_530;
    let now_count = previous_count.wrapping_add(12); // the card has wrapped: now 6
    // A plain `now_count - previous_count` would panic in a debug build (result below 0).
    let travelled = now_count.wrapping_sub(previous_count);
    println!("encoder {previous_count} -> {now_count}, travelled {travelled} counts");

    // Three honest answers to "what if the result does not fit?"
    let spare_counts: u16 = 65_000;
    println!("checked:    {:?}", spare_counts.checked_add(1_000)); // None = would overflow
    println!("wrapping:   {}", spare_counts.wrapping_add(1_000)); // wraps around
    println!("saturating: {}", spare_counts.saturating_add(1_000)); // sticks at the maximum

    // `as` never refuses. Narrowing keeps only the low bits.
    let setpoint_dint: i32 = 40_000;
    let setpoint_int = setpoint_dint as i16;
    println!("DINT {setpoint_dint} as INT: {setpoint_int}");

    // Analog scaling: raw 0..27648 means 0..10 bar. Convert to f64 before you divide.
    let raw: i16 = 20_000;
    let pressure_bar = raw as f64 * 10.0 / 27_648.0;
    println!("raw {raw} = {pressure_bar:.2} bar");

    let door_closed: bool = true;
    let unit: char = '°';
    println!("door closed: {door_closed}, unit: {unit}, 0xFF = {}, 0b1010_1010 = {}", 0xFF, 0b1010_1010);
}
// Output:
// encoder 65530 -> 6, travelled 12 counts
// checked:    None
// wrapping:   464
// saturating: 65535
// DINT 40000 as INT: -25536
// raw 20000 = 7.23 bar
// door closed: true, unit: °, 0xFF = 255, 0b1010_1010 = 170
@@callout warning | Watch out
Two traps. First, mixing sizes: an `i16` plus an `i32` gives **E0308**. Convert one side, `i32::from(a)` or `a as i32`, and decide on purpose which one. Second, the same `count + 1` panics under `cargo run` and wraps silently under `cargo run --release`. A counter test that passes in debug can then fail on the line PC. Use `wrapping_`, `checked_` or `saturating_` wherever overflow is possible, and test the release build as well.
@@text Exercise
A three-minute warm-up. Exercise D (chapter 1.3.3) comes after the next lesson.

1. Declare `let encoder_counts: u16 = 65_535;`. Print `checked_add(1)` (with `{:?}`), `wrapping_add(1)` and `saturating_add(1)`.
2. Before you run it, write down what `-1_i32 as u32` and `300_i32 as u8` will print. Then check.
3. Add the line `let sum = 1_i16 + 1_i32;`. Read the error code, then delete the line.
@@quiz Check yourself
? A `u16` holds 65535 and a release build runs `count + 1`. What happens?
- The program panics with an overflow message
+ The value wraps to 0 and the program carries on
- The value stays at 65535
= Release builds do not check plain arithmetic by default, so the value wraps. A debug build would panic. If you want the value to stay at the limit, ask for `saturating_add`.
? Which method gives the true distance between two readings of a 16-bit encoder that may have wrapped in between?
- `checked_sub`
+ `wrapping_sub`
- `saturating_sub`
= Wrapping subtraction counts the steps forward from the old reading to the new one, across the wrap. `checked_sub` would return `None` and `saturating_sub` would return 0.
? What does `40_000_i32 as i16` give?
- A compile error, because 40,000 does not fit
+ -25536
- 32767
= `as` never refuses. It keeps the low 16 bits of the number, and in a signed 16-bit value those bits mean -25536.
@@chapter 1.3.2 | Lesson 3.2 - Compound Types | 5
Tuples and arrays: several values held in one variable. Learn how to read a field, split a tuple apart, and why an array is fixed in length and always starts at index 0.
@@objectives
- Create a tuple, read its fields with `.0` and `.1`, and destructure it
- Declare an array with `[T; N]` and the repeat form `[value; N]`
- Index an array with a `usize` and say why index 0 is the first entry
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 3.2 - Compound Types · 2:14 — attach the MP4 here when you have it.
@@callout note | Plant situation
An extruder has eight heater zones. The HMI shows zone 1 to zone 8, and the CODESYS program keeps the temperatures in `ARRAY[1..8] OF REAL`. When the logic is ported to Rust, someone uses the HMI zone number straight as the index. Zone 1 now shows the temperature of zone 2, and zone 8 stops the program with an index error. Rust arrays always start at 0. The same port has a leak-test routine that must hand back two results, pass flag and decay in Pa. ST uses two `VAR_OUTPUT`s. Rust returns one tuple.
@@text Teacher's notes
### Two ways to group values
A compound type holds several values in one. Rust's basic two are the **tuple** and the **array**. Named groups (`struct`) come in Module 5, and arrays that can grow (`Vec`) later.

### Tuples: a few values, any types
`let result: (bool, f64) = (true, 1500.0);` holds a `bool` and an `f64`. The count and the types are fixed when you write it. You read a field by its position: `result.0` and `result.1`. The index is written out in the code; you cannot use a variable there.

You can also take a tuple apart in one line. This is **destructuring**: `let (passed, decay_pa) = result;` creates two bindings. An underscore throws a field away: `let (passed, _) = result;`.

Tuples are how a function gives back more than one value. They suit short, local bundles. Once the fields need names, use a `struct`. Remember the empty tuple `()` from lesson 2.6: it is the "nothing" value.

### Arrays: many values, one type
`let zone_temp_c: [f64; 8] = [182.0, 185.5, ...];` reads as "an array of eight `f64`". The length is part of the type, so `[f64; 8]` and `[f64; 6]` are different types, and passing one where the other is expected is **E0308**. `[180.0; 8]` is the repeat form: eight copies of one value, like `[8(180.0)]` in ST.

You read an entry with `zone_temp_c[2]`. Two rules matter:

- **Counting starts at 0.** The first entry is `[0]`, the last is `[len - 1]`. ST lets the bounds start anywhere; Rust has no such choice. So HMI zone 3 is `zone_temp_c[3 - 1]`.
- **The index is a `usize`.** An `i32` variable as an index gives **E0277**, *the type `[f64]` cannot be indexed by `i32`*.

`zone_temp_c.len()` gives the length. Every index is checked, as lesson 2.4 showed: a literal index past the end is refused when building, and one that arrives at run time panics.

### Printing them
`println!("{}", result)` fails with E0277, because a tuple has no default text form. Use `{:?}`, the debug format: `(true, 1500.0)`. It works for arrays too. An array lives in one block of memory, like an `ARRAY` inside a function block, so reading any entry takes the same time.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| `ARRAY[0..9] OF REAL` | `[f32; 10]` | The ST lower bound can be 1 or -5. Rust always starts at 0 |
| `aTemp[3]` | `temp_c[3]` | The index must be a `usize` |
| `ARRAY[1..8] OF REAL := [8(180.0)]` | `[180.0; 8]` | Same idea; the array's length is part of its type |
| Two or three `VAR_OUTPUT`s | A tuple as the return value | Tuple fields have no names, so keep them short |
| A `STRUCT` with differently typed fields | A tuple, or a `struct` from Module 5 | Tuple fields are read by position only |
| `CheckBounds` on array access | Every index is checked | A bad run-time index panics with the line |
@@code mermaid | Diagram: tuple or array
flowchart TD
  A["Compound types: several values in one"] --> B["Tuple: bool, f64, i32"]
  A --> C["Array: 8 values of f64"]
  B --> D["Types may differ, count is fixed"]
  B --> E["Read with result.0 or destructure"]
  C --> F["One type, length is part of the type"]
  C --> G["Read with zones and an index, first index is 0"]
@@code rust | Try it: a tuple result and a zone array
fn evaluate_leak_test(start_pa: f64, end_pa: f64, limit_pa: f64) -> (bool, f64) {
    let decay_pa = start_pa - end_pa;
    (decay_pa <= limit_pa, decay_pa) // a tuple: pass flag and decay in one value
}

fn main() {
    let result = evaluate_leak_test(600_000.0, 598_500.0, 2_000.0);
    println!("passed: {}, decay: {} Pa", result.0, result.1); // fields by position
    println!("whole tuple: {:?}", result); // {:?} prints tuples and arrays

    let (passed, decay_pa) = evaluate_leak_test(600_000.0, 590_000.0, 2_000.0); // destructuring
    println!("passed: {passed}, decay: {decay_pa} Pa");

    // The HMI shows zones 1 to 8. The array starts at index 0.
    let zone_temp_c: [f64; 8] = [182.0, 185.5, 190.0, 191.2, 189.0, 188.4, 186.0, 183.7];
    let hmi_zone: usize = 3;
    println!("zone {hmi_zone}: {} C", zone_temp_c[hmi_zone - 1]);
    println!("{} zones, last index {}", zone_temp_c.len(), zone_temp_c.len() - 1);

    let setpoint_c = [180.0; 8]; // eight copies of 180.0
    println!("setpoints: {:?}", setpoint_c);
}
// Output:
// passed: true, decay: 1500 Pa
// whole tuple: (true, 1500.0)
// passed: false, decay: 10000 Pa
// zone 3: 190 C
// 8 zones, last index 7
// setpoints: [180.0, 180.0, 180.0, 180.0, 180.0, 180.0, 180.0, 180.0]
@@callout warning | Watch out
The off-by-one from the plant story: an array of eight has indexes 0 to 7, so `zone_temp_c[8]` is wrong. With a literal the compiler stops you (*this operation will panic at runtime*). With a variable the program panics when it runs. The second trap: tuple and array use different brackets. Take `let line = ("filler", [182.0, 185.5]);`. `line.1[0]` is correct: field 1, then entry 0. `line.1.0` gives **E0609**, *no field `0` on type `[{float}; 2]`*, because an array is read with `[ ]`, not with a dot.
@@text Exercise
Exercise D (next chapter) practises all of this. A short warm-up first.

1. Write `fn min_max(a: f64, b: f64) -> (f64, f64)` that returns the smaller value first.
2. Call it with a destructuring `let (low, high) = ...` and print both.
3. Make an array `[0.0; 5]` of `f64` called `valve_position_pct` and print it with `{:?}`. Then print the entry for valve number 5 as the operator sees it (numbered from 1).
@@quiz Check yourself
? Given `let t = (true, 12.5, 7);`, how do you read the `12.5`?
- `t[1]`
+ `t.1`
- `t.2`
= Tuple fields are read with a dot and their position, counting from 0. Square brackets are for arrays.
? An array is `[f64; 8]` and the HMI asks for zone 8. Which index do you use?
- 8
- 9
+ 7
= Arrays count from 0, so the eighth entry has index 7. Index 8 would panic.
? You write `let i: i32 = 2;` and then `zone_temp_c[i]`. What does the compiler say?
- Nothing, it compiles
+ E0277: an array cannot be indexed by an `i32`
- E0382: borrow of moved value
= Array indexes must be `usize`. Declare the index as `usize` or convert it with `i as usize`.
@@chapter 1.3.3 | Lesson 3.3 - Exercise D - Simple Types | 21
Exercise D: pull single values out of tuples, arrays and a deeply nested bundle, build an array from a tuple, and destructure a tuple in a function's parameter list.
@@objectives
- Read tuple fields and array entries by index, also when they are nested
- Build an array from the fields of a tuple
- Destructure a tuple in a function's parameter list
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 3.3 - Exercise D - Simple Types · 5:00 — attach the MP4 here when you have it.
@@callout note | Plant situation
A gateway on a labelling machine hands over its diagnostics as one untidy bundle: a pair of counters, a temperature, a list of door switches that each carry a flag and a value, a retry count and a text. You do not control the layout, and you need single values out of it: one flag to switch a lamp, one number to compare. The exercise gives you a bundle exactly like this one, called `mess`, and five small jobs. After this you can read any nested tuple or array without guessing.
@@text Exercise brief
**Goal.** Make five small functions receive the right values from a tuple, an array and a nested bundle, by editing `main` in the starter file. The instructions are also written as comments in the file.

Steps:

1. **Tuple fields.** `coords` is `(6.3, 15.0)`. Pass its two fields as two separate arguments to `print_difference`, using tuple indexing.
2. **A tuple into an array.** `print_array` wants an `[f64; 2]`. Build one from the two fields of `coords` and pass it in.
3. **Array index.** Pass the value 13 from `series` to `ding`, by array indexing.
4. **Nested.** Pass the `true` hidden inside `mess` to `on_off`. You should see `Lights are on!`.
5. **Destructuring.** Uncomment `print_distance(coords)` and run it. Then change the function's parameter from `z` to `(x, y)` and use `x` and `y` in the body. The output must stay the same.
6. **Challenge.** Write out the full type of `series` and `mess` instead of letting the compiler infer them.

Open the folder in your editor, run it, and add one step at a time.
@@code rust | Starter code — exercise/d_simple_types/src/main.rs (MIT © Nathan Stocks)
// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

fn main() {
    let coords: (f64, f64) = (6.3, 15.0);
    // 1. Pass the two elements of the `coords` tuple as two separate arguments to the
    // `print_difference` function. Use tuple indexing.
    //
    // The `print_difference` function is defined near the end of this file if you would like to
    // look at how it is defined.
    //
    //print_difference( ... );   // Uncomment and finish this line

    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f64; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    //let coords_arr...               // create an array literal out of parts of `coord` here
    //print_array(coords_arr);        // and pass it in here (this line doesn't need to change)

    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    //ding(...);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    //
    //on_off(mess.2 ...);

    // 5. (Part A)
    //
    // Uncomment the line below, run the code, and examine the output. Then go refactor the
    // `print_distance` function according to the instructions in the comments inside that function.

    // print_distance(coords);
}

// 5. (Part B)
//
// Using `z.0` and `z.1` is not nearly as nice as using `x` and `y`.  Lucky for us, Rust supports
// destructuring function arguments.  Try replacing `z` in the parameter list below with `(x, y)`
// and then adjust the function body below to use `x` and `y` instead of `z.0` and `z.1`
//
// You should be able to run the code again and get the output as before.
fn print_distance(z: (f64, f64)) {
    println!(
        "Distance to the origin is {}",
        (z.0.powf(2.0) + z.1.powf(2.0)).sqrt()
    );
}

// Challenge:
//
// Although types can often be inferred by the compiler, sometimes we write them out for clarity.
// Like we did with the `let coords: (f64, f64) = ...` declaration at the top of the `main`
// function.
//
// - Add the type annotation for the `series` variable in `main`.
// - Add the type annotation for the `mess` variable in `main`. (This may be a good example of why
//   it is nice to *not* have to add the type annotation! 😆)

fn print_difference(x: f64, y: f64) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f64; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}
@@code bash | Run it
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\d_simple_types
cargo run
# Nothing prints at first: every step is still a comment. To keep the clone clean, work in a copy:
#   Copy-Item -Recurse D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\d_simple_types D:\RustroverProjects\learn_rust\mine\d_simple_types
@@callout tip | Hints
1. Step 1: a tuple field is read with a dot and its position, `coords.0`. Step 2: an array literal is `[a, b]`, and its type is written `[f64; 2]`.
2. Step 3: arrays count from 0. The array has seven entries and 13 is the last one, so its index is one less than the length.
3. Step 4: go one level at a time. `mess.2` is an array. Take its second entry with `[1]`. That entry is a tuple, so finish with `.0`.
4. Step 5: a parameter can be a pattern, just like a `let`. `fn print_distance((x, y): (f64, f64))` splits the tuple before the body starts.
@@code rust | Solution — try it yourself first
// Our solution to Exercise D: all five steps and the type-annotation challenge.
fn main() {
    let coords: (f64, f64) = (6.3, 15.0);

    // 1. A tuple field is read with a dot and its position.
    print_difference(coords.0, coords.1);

    // 2. An array holds one type; build [f64; 2] from the two tuple fields.
    let coords_arr: [f64; 2] = [coords.0, coords.1];
    print_array(coords_arr);

    // 3. Arrays count from 0, so the seventh entry has index 6.
    let series: [i32; 7] = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);

    // 4. Go in one level at a time:
    //    mess.2       the array of two tuples
    //    mess.2[1]    its second tuple, (true, -100)
    //    mess.2[1].0  the bool in that tuple
    let mess: ([i32; 2], f64, [(bool, i32); 2], i32, &str) =
        ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    // 5. The tuple is split into x and y in the parameter list (see print_distance).
    print_distance(coords);
}

fn print_distance((x, y): (f64, f64)) {
    println!("Distance to the origin is {}", (x.powf(2.0) + y.powf(2.0)).sqrt());
}

fn print_difference(x: f64, y: f64) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f64; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}
// Output:
// Difference between 6.3 and 15 is 8.7
// The coordinates are (6.3, 15)
// Ding, you found 13!
// Lights are on!
// Distance to the origin is 16.269296235547497
@@callout warning | Watch out
Step 4 is the one that trips people. `mess.2.1.0` looks natural, but `mess.2` is an array, and an array does not have a field called `1`: **E0609**, *no field `1` on type `[(bool, i32); 2]`*. Arrays use brackets, tuples use dots: `mess.2[1].0`. In step 3, `series[7]` is refused while building, because the compiler can see that the array has only seven entries. And passing `coords` itself to `print_array` is **E0308**: a tuple is not an array, even when both hold two `f64`.
@@quiz Check yourself
? Which expression reads the `true` out of `mess`?
- `mess.2.1.0`
+ `mess.2[1].0`
- `mess[2][1][0]`
= Field 2 of the tuple is an array, so the next step uses brackets. The entry at index 1 is a tuple, so the last step is a dot again.
? Why does `ding(series[7])` not even build?
- `ding` takes an `i32`, not an array entry
+ `series` has seven entries, so the last index is 6, and the compiler can see that 7 is past the end
- Array indexes must be written in hex
= A literal index past the end of a fixed-size array is caught while building. An index that arrives at run time would panic instead.
? What does `fn print_distance((x, y): (f64, f64))` do?
- Takes two parameters called `x` and `y`
+ Takes one tuple and splits it into `x` and `y` before the body runs
- Returns a tuple
= The parameter is a pattern. The caller still passes one tuple, and the function gets two bindings.
@@chapter 1.3.4 | Lesson 3.4 - Control Flow | 8
Deciding and repeating: `if` as an expression, and `loop`, `while` and `for` with `break`, `continue` and labels.
@@objectives
- Write an `if` / `else if` / `else` chain that produces a value
- Choose between `loop`, `while` and `for` for a given repeat
- Use `break` with a value and with a label
- Write ranges with `..` and `..=` without an off-by-one
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 3.4 - Control Flow · 5:03 — attach the MP4 here when you have it.
@@callout note | Plant situation
A sorter sends parts to lane 1, 2 or 3 by weight. The ST version is an `IF ... ELSIF ... END_IF` chain that writes the diverter output. Over the years someone adds a weight band and forgets the final `ELSE`. A heavy part falls through all the branches, nothing is written, and the output keeps its value from the last part. The part goes down the wrong lane. In Rust `if` is an expression that has to produce a value, so a missing `else` is an error on the day it is written.
@@text Teacher's notes
### `if` needs a `bool`, and no parentheses
`if weight_g < 200 { ... }` has no parentheses around the condition. Adding them still compiles but earns a warning. The braces are always required. The condition must be a real `bool`: `if count { ... }` with an integer fails with **E0308**, *expected `bool`, found integer*. ST is strict in the same way, and C is not.

### `if` is an expression
Every branch gives a value, so the whole `if` can sit on the right of a `let`:

`let lane = if weight_g < 200 { 1 } else if weight_g < 400 { 2 } else { 3 };`

All branches must have the same type, and the last `else` is required. Leave it out and you get **E0317**, *`if` may be missing an `else` clause*. Rust has no `? :` operator; this is it.

### `loop`: until you say stop
`loop { ... }` repeats forever. You leave it with `break`, and `continue` jumps to the next pass. A `loop` can hand a value out: `let layers = loop { ... break count; };`. This is the shape of a whole Rust control program too: read inputs, run the logic, write outputs, repeat. The difference from a PLC is that nobody else calls you again. A `loop` that never reaches `break` hangs the program, the same way a `WHILE` wait inside a PLC task trips the watchdog.

### `while`: test first
`while pressure_bar > 0.5 { ... }` tests the condition before every pass, like `WHILE ... DO`. Rust has no `REPEAT ... UNTIL`. Write `loop { ...; if done { break; } }` for a test at the end.

### `for`: a range or a collection
`for station in 1..=3` runs with 1, 2 and 3. `1..4` stops before 4, so it gives 1, 2 and 3 as well: **`..` excludes the end, `..=` includes it**. ST's `FOR i := 1 TO 3` includes the end, so it is the `..=` form. `(0..10).step_by(2)` is `BY 2`, and `(1..4).rev()` counts down. A `for` also walks an array: `for weight_g in weights_g`. There is no index to update by hand and no way to run past the end.

### Labels
Inside nested loops, `break` leaves only the inner one. Name the outer loop with `'search: for ...` and write `break 'search;` to leave both. `CASE` has its own tool in Rust, `match`, which comes in a later lesson.
@@text PLC ↔ Rust
| PLC world | Rust | Where the comparison stops |
|---|---|---|
| `IF ... ELSIF ... ELSE ... END_IF` | `if ... else if ... else` | Rust's `if` gives a value, so a missing `else` is an error |
| `CASE x OF` | `match`, in a later lesson | `match` must cover every case |
| `FOR i := 1 TO 8 DO` | `for i in 1..=8` | `TO` includes the end; `..` does not |
| `WHILE c DO ... END_WHILE` | `while c { ... }` | Same: tested before each pass |
| `REPEAT ... UNTIL c` | `loop { ...; if c { break; } }` | No dedicated keyword |
| `EXIT` and `CONTINUE` | `break` and `continue` | `break` can carry a value and a label |
| The runtime calls your program every scan | You write the cycle yourself as a `loop` | A loop that never ends hangs the program |
@@code mermaid | Diagram: which repeat do I need
flowchart LR
  Q["Which repeat do you need?"] --> A["A known range or an array"]
  Q --> B["Repeat while something is true"]
  Q --> C["Until an event, maybe with a result"]
  A --> A2["for x in 1..=8"]
  B --> B2["while still above the limit"]
  C --> C2["loop, then break or break value"]
@@code rust | Try it: if as a value, loop with break, while, for with a label
fn lane_for(weight_g: u32) -> u32 {
    // `if` is an expression: every branch gives a value, so the final `else` is required.
    if weight_g < 200 { 1 } else if weight_g < 400 { 2 } else { 3 }
}

fn main() {
    println!("182 g -> lane {}, 460 g -> lane {}", lane_for(182), lane_for(460));

    // `loop` can hand back a value through `break`: stack 150 mm layers up to 1200 mm.
    let mut height_mm = 0;
    let mut layers = 0;
    let layers_on_pallet = loop {
        layers += 1;
        height_mm += 150;
        if height_mm >= 1_200 { break layers; }
    };
    println!("{layers_on_pallet} layers, {height_mm} mm");

    // `while` checks first, like WHILE ... DO: bleed the chamber below 0.5 bar.
    let mut pressure_bar = 6.0;
    while pressure_bar > 0.5 {
        pressure_bar /= 2.0;
    }
    println!("chamber at {pressure_bar} bar");

    // `1..=3` includes the 3, like FOR i := 1 TO 3. A label lets `break` leave both loops.
    'search: for station in 1..=3 {
        for slot in 1..=4 {
            if station == 2 && slot == 3 { // stands in for a sensor read
                println!("jam at station {station}, slot {slot}");
                break 'search;
            }
        }
    }
}
// Output:
// 182 g -> lane 1, 460 g -> lane 3
// 8 layers, 1200 mm
// chamber at 0.375 bar
// jam at station 2, slot 3
@@callout warning | Watch out
Two classic slips. First, the range end: `for slot in 1..8` visits 1 to 7, not 8, where ST's `FOR slot := 1 TO 8` visits 8. Write `1..=8`. Second, a condition that is not a `bool`. `if status_word { ... }` is **E0308**, and so is `if x = 5` (a single `=` assigns, and an assignment is not a `bool`). Compare with `==` and write the test out: `if status_word != 0 { ... }`. ST readers take care: in ST `=` compares, in Rust `=` assigns.
@@text Exercise
Exercise E (next chapter) does the full set. A warm-up now.

1. Write `fn lane_for(weight_g: u32) -> u32` as in the Try it. Add a fourth lane for weights of 600 g and up.
2. Remove the final `else` and read the error code. Put it back.
3. Print the numbers 10, 8, 6, 4, 2 with a `for` loop and `step_by`, then the same numbers with `.rev()` on a `1..=5` range.
@@quiz Check yourself
? How many passes does `for i in 1..4` make, and with which values?
- Four passes: 1, 2, 3, 4
+ Three passes: 1, 2, 3
- Three passes: 2, 3, 4
= `..` excludes the end value. `1..=4` would include it, like ST's `FOR i := 1 TO 4`.
? What is the value of `let x = loop { break 7; };`?
- `()`, because a loop gives nothing back
+ 7
- The code does not compile
= `break` can carry a value out of a `loop`. That value becomes the value of the whole `loop` expression.
? `let lane = if w < 200 { 1 } else if w < 400 { 2 };` fails with E0317. Why?
- `if` cannot be used on the right of `let`
- The numbers must be written as `1u32` and `2u32`
+ When `if` gives a value, the last branch must be an `else`, so that every weight has a value
= Without the final `else`, a weight of 400 g or more would have no value. Rust refuses to leave that undefined.
@@chapter 1.3.5 | Lesson 3.5 - Exercise E - Control Flow | 20
Exercise E: write a `loop` with `break`, a `for` over an inclusive range, a `while` that fills a list, and an `if` / `else if` / `else` chain.
@@objectives
- Choose and write `loop`, `for` and `while` for four small jobs
- Use `..=` to include the end of a range
- Rewrite each loop as a different kind of loop
@@video Lesson video | **Ardan Labs** · Crash Course · Lesson 3.5 - Exercise E - Control Flow · 3:53 — attach the MP4 here when you have it.
@@callout note | Plant situation
On the commissioning laptop you need four tiny calculations, the kind you write in an afternoon in a test POU: how many times a quantity can double before it passes a limit, the sum of a range of numbers, a table of the first twelve multiples of five, and a score that goes up or down by the value of each code. In ST you would use a `REPEAT`, a `FOR`, a `WHILE` and an `IF` chain. The exercise asks for the same four, in Rust, one per loop kind. The numbers belong to bunnies; the loops are the ones you will use to average twelve samples or to walk a table of recipes.
@@text Exercise brief
**Goal.** Fill in four gaps in `main` so the program prints the four answers below. Then rewrite three of them with a different kind of loop.

Steps:

1. **`loop`.** Start with `count = 0` and `bunnies = 2`. In an endless `loop` add 1 to `count`, double `bunnies`, and `break` when `bunnies` is above 500. The program should say 8.
2. **`for` and a range.** Add the whole numbers from 7 to 23, **including 23**, into `sum`. The answer is 255.
3. **`while`.** Fill the vector `fives` with the first twelve multiples of 5. The loop runs while `fives.len()` is below 12. Each pass pushes `number` and then adds 5 to it. (`Vec` is a list that can grow; it gets its own lesson later. You only need `push` and `len` here.)
4. **`if`, `else if`, `else`.** For each number in the list: 0 adds 7 to `total`, 1 or 2 adds 30, anything else subtracts 5. The answer is 52.
5. **Challenge.** Redo step 1 with `while`, step 2 with `loop`, and step 3 with `for` over a range, multiplying by 5 inside the loop.
@@code rust | Starter code — exercise/e_control_flow/src/main.rs (MIT © Nathan Stocks)
// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // 1. Use an unconditional `loop` to count how many times we can double `bunnies` until there
    // are over 500 bunnies. (Hint: The answer is 8 times)
    //
    // Inside the loop:
    // - Add 1 to `count`
    // - Multiply `bunnies` by 2
    // - If `bunnies` is larger than 500, break out of the loop.

    let mut count = 0;
    let mut bunnies = 2;

    // (write your `loop` here)

    println!(
        "Bunnies doubled {} times before there were more than 500",
        count
    );

    // 2. Use a `for` loop to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (add each value to the `sum` variable).  Hint: You should get 255

    let mut sum = 0;

    // (write the `for` loop here)

    println!("The sum is {}", sum);

    // 3. Use a `while` loop to add 12 numbers to the `fives` vector.
    //
    // The loop should continue while `fives.len()` is less than 12.
    //
    // Each time through the loop:
    // - Call `fives.push(number)` to push (a copy of) `number` onto the vector
    // - Add 5 to `number`
    //
    // If you do this correctly, the vector will be [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60]

    let mut fives: Vec<i32> = vec![];
    let mut number = 5;

    // (write the `while` loop here)

    println!("Here are the first 12 multiples of 5: {:?}", fives);

    // 4. Use `if`, `else if` and `else` inside the `for` loop below to do the following:
    //
    // - If the number is 0, then add 7 to `total`
    // - If the number is 1 or 2 then add 30 to `total`
    // - If the number is anything else, subtract 5 from `total`
    //
    // Hint: The total should be 52

    let mut total = 0;
    let numbers = vec![0, 1, 2, 3, 4, 5];
    for number in numbers {
        // (write your `if/else` expression here)
    }

    println!("The total is {}", total);

    // Challenge: Change the implementation of your answers to #1-#3 as follows:
    //
    // - Change #1 to use `while`
    // - Change #2 to use `loop`
    // - Change #3 to use `for` and a range (multiply the range value by 5 inside your loop before
}
@@code bash | Run it
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\e_control_flow
cargo run
# The starter already compiles and prints zeros and an empty list. To keep the clone clean, work in a copy:
#   Copy-Item -Recurse D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\e_control_flow D:\RustroverProjects\learn_rust\mine\e_control_flow
@@callout tip | Hints
1. Step 1: `loop { ... }`, and inside it an `if bunnies > 500 { break; }`. Put the doubling before the test, so the pass that goes over 500 still counts.
2. Step 2: `for n in 7..=23 { sum += n; }`. Without the `=` the range stops at 22.
3. Step 3: `while fives.len() < 12 { ... }`. Inside, `fives.push(number);` first, then `number += 5;`.
4. Step 4: `if number == 0 { ... } else if number == 1 || number == 2 { ... } else { ... }`. The `||` means "or". Use `total += 7;` and `total -= 5;` inside.
@@code rust | Solution — try it yourself first
// Our solution to Exercise E: the four steps, then the three challenge rewrites.
fn main() {
    // 1. `loop` runs until `break`. The `if` decides when.
    let mut count = 0;
    let mut bunnies = 2;
    loop {
        count += 1;
        bunnies *= 2;
        if bunnies > 500 {
            break;
        }
    }
    println!("Bunnies doubled {} times before there were more than 500", count);

    // 2. `7..=23` includes the 23. With `7..23` the sum would be 232.
    let mut sum = 0;
    for n in 7..=23 {
        sum += n;
    }
    println!("The sum is {}", sum);

    // 3. `while` tests the condition before every pass.
    let mut fives: Vec<i32> = vec![];
    let mut number = 5;
    while fives.len() < 12 {
        fives.push(number);
        number += 5;
    }
    println!("Here are the first 12 multiples of 5: {:?}", fives);

    // 4. `if`, `else if`, `else`: the first branch whose condition is true runs.
    let mut total = 0;
    let numbers = vec![0, 1, 2, 3, 4, 5];
    for number in numbers {
        if number == 0 {
            total += 7;
        } else if number == 1 || number == 2 {
            total += 30;
        } else {
            total -= 5;
        }
    }
    println!("The total is {}", total);

    // Challenge: the same answers with another kind of loop.
    // #1 with `while`: the test comes first, so we test the value that is still allowed.
    let mut count_b = 0;
    let mut bunnies_b = 2;
    while bunnies_b <= 500 {
        count_b += 1;
        bunnies_b *= 2;
    }
    // #2 with `loop`: add first, then step on, then check the end.
    let mut sum_b = 0;
    let mut n = 7;
    loop {
        sum_b += n;
        n += 1;
        if n > 23 {
            break;
        }
    }
    // #3 with `for`: the range gives 1..=12 and we multiply by 5 inside.
    let mut fives_b: Vec<i32> = vec![];
    for step in 1..=12 {
        fives_b.push(step * 5);
    }
    println!("Challenge: {} doublings, sum {}, same list: {}", count_b, sum_b, fives_b == fives);
}
// Output:
// Bunnies doubled 8 times before there were more than 500
// The sum is 255
// Here are the first 12 multiples of 5: [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60]
// The total is 52
// Challenge: 8 doublings, sum 255, same list: true
@@callout warning | Watch out
The usual slips: `7..23` leaves out 23, so the sum becomes 232 instead of 255. Writing `number == 1 || 2` is **E0308**, because `2` is not a `bool`; each side of `||` needs its own comparison. In the challenge, `while` tests before it doubles. The `loop` stopped when `bunnies > 500`, so the `while` must keep going while `bunnies <= 500`. If your count is off by one pass, check that test first. And `for number in numbers` takes the list with it: using `numbers` after the loop is **E0382**, *borrow of moved value*. Module 4 explains why.
@@quiz Check yourself
? In step 2, what would the sum be if you wrote `7..23` instead of `7..=23`?
- 255, the same
+ 232, because 23 is left out
- 278, because 24 is added
= `..` stops before the end value. 255 minus 23 is 232.
? Which line writes "1 or 2" correctly?
- `if number == 1 or 2`
- `if number == (1, 2)`
+ `if number == 1 || number == 2`
= `||` joins two complete `bool` tests. `number == 1` is one test and `number == 2` is the other.
? You do not know in advance how many passes you need and you want the test before each pass. Which loop fits?
- `for`
+ `while`
- `loop` with no `break`
= `while` checks its condition before every pass, like `WHILE ... DO` in ST. A `for` suits a known range or collection.
