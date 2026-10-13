@@course Ultimate Rust Bundle | Ardan Labs curriculum, taught through the PLC world | Teacher's notes for a PLC engineer
This library follows the Ardan Labs Ultimate Rust Bundle chapter by chapter: 304 chapters in eight parts, in the same order and with the same titles as the course player. Every video chapter keeps an empty slot where you attach the Ardan MP4 yourself. Around that slot sits our own teaching, written for someone who commissions PLC systems for a living: a plant situation, plain notes, a PLC to Rust table, a diagram, code that runs, the usual mistake, the matching exercise and a short quiz. The videos, slides and quizzes belong to Ardan Labs. The notes, examples and questions here are ours.
@@part 0 · Start Here
@@section Ultimate Rust Bundle Guide
@@chapter 0.0.1 | Start Here | 6
The map of the whole bundle: eight parts, what each one gives you, and what it replaces in your PLC toolbox. Read it once now and again when you finish Part 1.
@@objectives
- Name the eight parts of the bundle and say what each one teaches
- Choose which part to open next from what you need on your next project
- Explain why Part 1 comes first and Part 2 comes second
@@attachment Ardan review sheet (PDF) | Start Here — download it from the Ardan course page and attach it here.
@@text Cheat sheet
The bundle has seven Ardan parts, taught by two instructors: Nathan Stocks (Parts 1 and 5) and Herbert Wolverson (Parts 2, 3, 4, 6 and 7). Part 0 is our own addition.

| Part | One-line meaning | PLC equivalent |
|---|---|---|
| 0 · Start Here | This guide, the PLC ↔ Rust dictionary and a study plan | The project handbook you read before the first commissioning day |
| 1 · Ultimate Rust Crash Course | The whole language in small steps: Cargo, variables, functions, modules, types, ownership, structs, traits, collections, enums | The IEC 61131-3 basics course: data types, POUs, libraries, structured text |
| 2 · Ultimate Rust Foundations | Threads, async/await, memory and resources, and a network data collector built from scratch | Multitasking and task configuration, shared GVLs, a fieldbus data logger |
| 3 · Ultimate Rust: Foundations - Next Steps | Two-way communication with the collector, limits on queue growth, faster file reading | Adding commands and handshakes to a telegram, bounded buffers |
| 4 · Ultimate Rust Best Practices | Formatting, the clippy linter, documentation, dependency hygiene, small functions, float care | Code review checklist, static analysis, library version governance |
| 5 · Ultimate Rust 2: Intermediate Concepts | Idiomatic code, docs, publishing, iterators, closures, errors, tests, logging, threads, then a small game engine | Library packaging, FB unit tests, the event log, the scan cycle as a game loop |
| 6 · Rust as a Service | REST service, tracing, OpenAPI, configuration, gRPC, WebSockets, deployment | The machine's endpoint for MES, parameter files, live HMI push, the shippable cabinet |
| 7 · Rust Webinars | Five recorded talks: Rust in a company, concurrency, async, highlights, debugging | The vendor seminar you watch after you know the product |

The numbers are Ardan's order. The arrows above are the order we suggest, and chapter 0.1.3 explains why Part 5 comes before Part 4. Parts 1 to 3 teach the language and the first real program. Parts 5 and 4 make your code the kind other people can maintain. Part 6 turns a program into a service another system can call. Part 7 is the extra shelf.
@@code mermaid | Module map
flowchart LR
  P0["Part 0: Start Here"] --> P1["Part 1: Crash Course"]
  P1 --> P2["Part 2: Foundations"]
  P2 --> P3["Part 3: Next Steps"]
  P3 --> P5["Part 5: Ultimate Rust 2"]
  P5 --> P4["Part 4: Best Practices"]
  P4 --> P6["Part 6: Rust as a Service"]
  P6 --> P7["Part 7: Webinars"]
@@callout tip | Before you start
Answer these three for yourself. If one of them is a no, fix it before Part 1.
- Can you open a terminal and run `rustc --version` and `cargo --version`? Chapter 1.1.3 shows how to install both.
- Have you cloned the exercise repositories to `D:\RustroverProjects\learn_rust\ardan\`? Chapter 0.1.1 lists them.
- Which small job on your own plant would you like to rebuild in Rust one day: a data logger, a recipe loader, a Modbus poller? Pick one now. Every later lesson is easier to remember when it has a place to land.
