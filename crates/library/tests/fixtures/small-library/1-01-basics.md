@@part 1 · Fixture Crash Course
@@section Module 1: Basics
@@chapter 1.1.1 | Lesson 1.1 - Variables | 12
Bindings, immutability, `mut` and constants.
@@objectives
- Declare a binding and say why it cannot change by default
- Choose between `let`, `let mut` and `const`
@@video Lesson video | **Fixture Labs** · Crash Course · Lesson 1.1 - Variables · 5:07 — attach the MP4 here.
@@callout note | Plant situation
You are commissioning a bottle filler. The fill volume must not drift
while the cycle runs, so it is written once and then only read.
@@text Teacher's notes
In a PLC every tag you declare can be written by any rung.

In Rust a binding is read-only until you ask otherwise with `mut`.
@@code mermaid | Diagram: where a binding lives
flowchart LR
  A["let x = 5"] --> B["x is read-only"]
@@code rust | Try it: fill volume
fn main() {
    let fill_volume_ml = 500;
    println!("{fill_volume_ml}");
}
// Output: 500
@@quiz Check yourself
? Which keyword makes a binding changeable?
- const
+ mut
- static
= `mut` is the opt-in.
? What does `let x = 5;` give you?
+ A read-only binding
- A changeable binding
@@chapter 1.1.2 | Lesson 1.2 - Cargo | 15
Projects, crates and the manifest.
@@image Project layout | A screenshot of the project tree goes here.
@@code toml | Cargo.toml
[package]
name = "filler"
version = "0.1.0"
@@code bash | Run it
cargo run
@@code json | Sample payload
{"station": "filler-1", "fill_ml": 500}
@@code text | Expected output
500
@@code rust | Uses a crate
// Cargo.toml: serde = { version = "1", features = ["derive"] }
use serde::Serialize;

#[derive(Serialize)]
struct Reading {
    fill_ml: u32,
}
@@code rust | A fragment
// no-check
impl Station {
    fn run(&self) {}
}
@@callout warning | Watch out
Do not edit `Cargo.lock` by hand.
@@callout danger | Never do this
Do not bridge an interlock to make a test pass.
@@quiz Cargo quiz
? Which file lists the dependencies?
- Cargo.lock
+ Cargo.toml
= The lock file only records what was resolved.
