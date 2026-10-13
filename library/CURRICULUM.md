# Ultimate Rust Bundle — curriculum map

Source: the Ardan Labs course player of the owner's enrolled account, read
2026-10-09. This file is the single source of truth for **structure and
numbering**. Every content file under `library/ultimate-rust-bundle/` must
reproduce these parts, sections, display numbers and titles exactly.

Rules

- `display` = `<part>.<section>.<item>`; it goes into `@@chapter <display> | <title> | <minutes>`.
- `title` is Ardan's item name, verbatim (trailing spaces trimmed).
- `video` is the Ardan video length (m:ss or h:mm:ss). Use it in the video
  placeholder caption. `—` means the Ardan item is not a video.
- Type: `Video` lesson, `Text` (Ardan text page), `PDF` (review sheet),
  `Quiz`, `Own` (our own addition, not an Ardan item).
- Skipped on purpose (marketing / feedback, no learning content):
  every "Our Services", "ardanlabs.com", "Want to see more?", "Our Rust
  Instructor" page and every "Feedback Survey".
- Two Ardan headings are copy-paste errors and are renamed here:
  URC2's first chapter is titled "Ultimate Rust Crash Course" on Ardan
  (renamed `Ultimate Rust 2: Intermediate Concepts`), and the Debugging
  webinar's chapter is titled "Tracing Programs with Go" (renamed
  `Debugging & Optimizing Rust`).
- Never write Ardan's file URLs (PDF CDN links) into this repo: it is public.

---

## Part 0 · Start Here

`@@part 0 · Start Here`

### Section 0 — `Ultimate Rust Bundle Guide`
| display | title | type | video |
|---|---|---|---|
| 0.0.1 | Start Here | PDF | — |

### Section 1 — `Teacher's Orientation` (our additions)
| display | title | type | video |
|---|---|---|---|
| 0.1.1 | How to use this library | Own | — |
| 0.1.2 | The PLC ↔ Rust dictionary | Own | — |
| 0.1.3 | Your learning path and study rhythm | Own | — |

---

## Part 1 · Ultimate Rust Crash Course

`@@part 1 · Ultimate Rust Crash Course` · instructor Nathan Stocks ·
exercises: `CleanCut/ultimate_rust_crash_course` (MIT)

### Section 0 — `Ultimate Rust Crash Course`
| display | title | type | video |
|---|---|---|---|
| 1.0.1 | Ultimate Rust Crash Course | Text | — |

### Section 1 — `Module 1: Introduction`
| display | title | type | video |
|---|---|---|---|
| 1.1.1 | Lesson 1.1 - Introduction | Video | 4:08 |
| 1.1.2 | Lesson 1.2 - Exercises Overview | Video | 1:03 |
| 1.1.3 | Lesson 1.3 - Installation | Video | 1:36 |
| 1.1.4 | Lesson 1.4 - Editor/IDE | Video | 2:33 |
| 1.1.5 | Module 1 - Review | PDF | — |
| 1.1.6 | Module 1 - Quiz | Quiz | — |

### Section 2 — `Module 2: Fundamentals`
| display | title | type | video |
|---|---|---|---|
| 1.2.1 | Lesson 2.1 - Cargo | Video | 6:55 |
| 1.2.2 | Lesson 2.2 - Variables | Video | 5:07 |
| 1.2.3 | Lesson 2.3 - Scope | Video | 2:44 |
| 1.2.4 | Lesson 2.4 - Memory Safety | Video | 1:33 |
| 1.2.5 | Lesson 2.5 - Exercise A - Variables | Video | 4:56 |
| 1.2.6 | Lesson 2.6 - Functions | Video | 2:15 |
| 1.2.7 | Lesson 2.7 - Exercise B - Functions | Video | 4:51 |
| 1.2.8 | Lesson 2.8 - Module System | Video | 8:57 |
| 1.2.9 | Lesson 2.9 - Exercise C - Module System | Video | 4:27 |
| 1.2.10 | Module 2 - Review | PDF | — |
| 1.2.11 | Module 2 - Quiz | Quiz | — |

### Section 3 — `Module 3: Primitive Types & Control Flow`
| display | title | type | video |
|---|---|---|---|
| 1.3.1 | Lesson 3.1 - Scalar Types | Video | 5:52 |
| 1.3.2 | Lesson 3.2 - Compound Types | Video | 2:14 |
| 1.3.3 | Lesson 3.3 - Exercise D - Simple Types | Video | 5:00 |
| 1.3.4 | Lesson 3.4 - Control Flow | Video | 5:03 |
| 1.3.5 | Lesson 3.5 - Exercise E - Control Flow | Video | 3:53 |
| 1.3.6 | Lesson 3.6 - Strings | Video | 4:45 |
| 1.3.7 | Lesson 3.7 - String Literals | Video | 3:59 |
| 1.3.8 | Lesson 3.8 - Exercise F - Strings | Video | 2:18 |
| 1.3.9 | Module 3 - Review | PDF | — |
| 1.3.10 | Module 3 - Quiz | Quiz | — |

### Section 4 — `Module 4: The Heart of Rust`
| display | title | type | video |
|---|---|---|---|
| 1.4.1 | Lesson 4.1 - Ownership | Video | 6:07 |
| 1.4.2 | Lesson 4.2 - References & Borrowing | Video | 5:22 |
| 1.4.3 | Lesson 4.3 - Exercise G - Ownership & References | Video | 5:29 |
| 1.4.4 | Module 4 - Review | PDF | — |
| 1.4.5 | Module 4 - Quiz | Quiz | — |

### Section 5 — `Module 5: The Meat of Rust`
| display | title | type | video |
|---|---|---|---|
| 1.5.1 | Lesson 5.1 - Structs | Video | 3:14 |
| 1.5.2 | Lesson 5.2 - Exercise H - Structs | Video | 6:20 |
| 1.5.3 | Lesson 5.3 - Traits | Video | 5:05 |
| 1.5.4 | Lesson 5.4 - Exercise I - Traits | Video | 6:05 |
| 1.5.5 | Lesson 5.5 - Collections | Video | 3:15 |
| 1.5.6 | Lesson 5.6 - Exercise J - Collections | Video | 4:25 |
| 1.5.7 | Lesson 5.7 - Enums | Video | 8:15 |
| 1.5.8 | Lesson 5.8 - Exercise K - Enums | Video | 8:13 |
| 1.5.9 | Module 5 - Review | PDF | — |
| 1.5.10 | Module 5 - Quiz | Quiz | — |

### Section 6 — `Full Course Review`
| display | title | type | video |
|---|---|---|---|
| 1.6.1 | Full Final Review | PDF | — |
| 1.6.2 | Ultimate Rust Crash Course Final Exam | Quiz | — |

---

## Part 2 · Ultimate Rust Foundations

`@@part 2 · Ultimate Rust Foundations` · instructor Herbert Wolverson ·
code: `thebracket/ArdanUltimateRust-5Days` (no licence — link only)

### Section 0 — `Ultimate Rust Foundations`
| display | title | type | video |
|---|---|---|---|
| 2.0.1 | Ultimate Rust Foundations | Text | — |

### Section 1 — `Getting Started with Rust`
| display | title | type | video |
|---|---|---|---|
| 2.1.1 | 1.0 - Introduction | Video | 1:56 |
| 2.1.2 | 1.1 - Setup & Update Rust | Video | 4:07 |
| 2.1.3 | 1.2 - Setup Dev Environment | Video | 4:16 |
| 2.1.4 | 1.3 - IDE Configuration | Video | 3:42 |
| 2.1.5 | 1.4 - Rust Fundamentals | Video | 28:55 |
| 2.1.6 | 1.5 - Console Text Input | Video | 7:49 |
| 2.1.7 | 1.6 - Create a Library | Video | 15:04 |
| 2.1.8 | 1.7- World’s Simplest Login System | Video | 9:09 |
| 2.1.9 | 1.8 - Enumerations | Video | 23:40 |
| 2.1.10 | 1.9 - Structures | Video | 14:55 |
| 2.1.11 | 1.10 - Vectors | Video | 12:10 |
| 2.1.12 | 1.11 - HashMaps | Video | 10:01 |
| 2.1.13 | 1.12 - Serialization/Deserialization | Video | 15:47 |
| 2.1.14 | 1.13 - Hashing Passwords | Video | 11:09 |
| 2.1.15 | 1.14 - Start a CLI App | Video | 41:04 |
| 2.1.16 | Module 1 - Review | PDF | — |
| 2.1.17 | Module 1 - Quiz | Quiz | — |

### Section 2 — `Fearless System Thread Concurrency`
| display | title | type | video |
|---|---|---|---|
| 2.2.1 | 2.0 - Introduction | Video | 4:26 |
| 2.2.2 | 2.1 - System Threads: Overview | Video | 6:01 |
| 2.2.3 | 2.2 - Create Your First Thread | Video | 5:12 |
| 2.2.4 | 2.3 - Spawning Threads with Parameters and Closures | Video | 6:05 |
| 2.2.5 | 2.4 - Returning Data from Threads | Video | 5:16 |
| 2.2.6 | 2.5 - Dividing Workloads | Video | 9:35 |
| 2.2.7 | 2.6 - The ThreadBuilder Pattern | Video | 8:08 |
| 2.2.8 | 2.7 - Scoped Threads for Easy Local Data Sharing | Video | 10:07 |
| 2.2.9 | 2.8 - Sharing Data with Atomics | Video | 18:12 |
| 2.2.10 | 2.9 - Sharing Data with Mutexes | Video | 9:48 |
| 2.2.11 | 2.10 - Read/Write Locks | Video | 14:39 |
| 2.2.12 | 2.11 - Deadlocks, Panics and Poisoning | Video | 18:28 |
| 2.2.13 | 2.12 - Sharing Data with Lock-Free Structures | Video | 7:23 |
| 2.2.14 | 2.13 - Parking Threads | Video | 7:13 |
| 2.2.15 | 2.14 - Sending Data Between Threads with Channels | Video | 8:34 |
| 2.2.16 | 2.15 - Sending Functions to Worker Threads | Video | 13:57 |
| 2.2.17 | 2.16 - Let’s build a work queue with a thread pool | Video | 9:27 |
| 2.2.18 | 2.17 - Thread CPU/Core Affinity | Video | 7:46 |
| 2.2.19 | 2.18 - Thread Priority | Video | 12:19 |
| 2.2.20 | 2.19 - Making it Easy with Rayon | Video | 16:33 |
| 2.2.21 | 2.20 - Scopes and Pooled Threads with Rayon | Video | 17:17 |
| 2.2.22 | Module 2 - Review | PDF | — |
| 2.2.23 | Module 2 - Quiz | Quiz | — |

### Section 3 — `Async/Await Concurrency`
| display | title | type | video |
|---|---|---|---|
| 2.3.1 | 3.0 - Introduction / Async Overview | Video | 7:29 |
| 2.3.2 | 3.1 - Hello Async/Await | Video | 15:27 |
| 2.3.3 | 3.2 - Getting Started with Tokio | Video | 9:06 |
| 2.3.4 | 3.3 - Working with Tokio Futures: Awaiting, Yielding and Spawning | Video | 14:01 |
| 2.3.5 | 3.4 - Blocking Tasks | Video | 10:04 |
| 2.3.6 | 3.5 - Unit Testing Tokio | Video | 10:20 |
| 2.3.7 | 3.6 - Handling Errors | Video | 41:43 |
| 2.3.8 | 3.7 - File I/O | Video | 17:20 |
| 2.3.9 | 3.8 - Basic Network I/O | Video | 23:49 |
| 2.3.10 | 3.9 - Async Channels (Tokio) | Video | 14:27 |
| 2.3.11 | 3.10 - Shared State (Tokio) | Video | 3:46 |
| 2.3.12 | 3.11 - Selecting Futures | Video | 9:15 |
| 2.3.13 | 3.12 - Pinning | Video | 16:47 |
| 2.3.14 | 3.13 - Tokio Tracing | Video | 22:55 |
| 2.3.15 | 3.14 - Working with Databases | Video | 34:12 |
| 2.3.16 | 3.15 - Axum - A Web Framework built on Tokio | Video | 18:55 |
| 2.3.17 | 3.16 - Let’s Build a Thumbnail Server | Video | 1:12:25 |
| 2.3.18 | Module 3 - Review | PDF | — |
| 2.3.19 | Module 3 - Quiz | Quiz | — |

### Section 4 — `Memory & Resource Management`
| display | title | type | video |
|---|---|---|---|
| 2.4.1 | 4.0 - Why Haven’t We Manually Managed Any Memory Yet? | Video | 3:41 |
| 2.4.2 | 4.1 - The unsafe Keyword | Video | 9:05 |
| 2.4.3 | 4.2 - Low-Level Memory Management | Video | 7:14 |
| 2.4.4 | 4.3 - The Drop Trait & RAII (Resource Acquisition is Initialization) | Video | 17:08 |
| 2.4.5 | 4.4 - Reference Counting | Video | 18:29 |
| 2.4.6 | 4.5 - Lifetimes | Video | 17:05 |
| 2.4.7 | 4.6 - Traits | Video | 25:07 |
| 2.4.8 | 4.7 - Generics | Video | 19:29 |
| 2.4.9 | 4.8 - Iterators | Video | 15:04 |
| 2.4.10 | 4.9 - Cycles and the Difficulty of Linked Lists | Video | 6:52 |
| 2.4.11 | 4.10 - Memory Fragmentation, Allocators and Arenas | Video | 7:38 |
| 2.4.12 | 4.11 - Packing, Reordering & Mangling | Video | 5:26 |
| 2.4.13 | 4.12 - From Bytes to Types | Video | 12:03 |
| 2.4.14 | 4.13 - Safely Interacting with Other Languages & Surprise: Memory Leaks are Safe! | Video | 17:09 |
| 2.4.15 | Module 4 - Review | PDF | — |
| 2.4.16 | Module 4 - Quiz | Quiz | — |

### Section 5 — `Build a Network Service`
| display | title | type | video |
|---|---|---|---|
| 2.5.1 | 5.0 - Introduction & Planning Our Project | Video | 6:27 |
| 2.5.2 | 5.1 - Shared Data Structures | Video | 15:30 |
| 2.5.3 | 5.2 - Collection Daemon Mk 1 | Video | 13:22 |
| 2.5.4 | 5.3 - Collection Server Mk 1 | Video | 9:00 |
| 2.5.5 | 5.4 - Error Handling in the Collector | Video | 22:46 |
| 2.5.6 | 5.5 - Setting the Collector ID | Video | 5:09 |
| 2.5.7 | 5.6 - Web Service Mk 1 | Video | 27:31 |
| 2.5.8 | 5.7 - Web Server | Video | 11:09 |
| 2.5.9 | 5.8 - Let’s Use Less Bandwidth | Video | 6:53 |
| 2.5.10 | Module 5 - Review | PDF | — |
| 2.5.11 | Module 5 - Quiz | Quiz | — |

### Section 6 — `Full Course Review`
| display | title | type | video |
|---|---|---|---|
| 2.6.1 | Full Final Review | PDF | — |
| 2.6.2 | Ultimate Rust Foundation Final Exam | Quiz | — |

---

## Part 3 · Ultimate Rust: Foundations - Next Steps

`@@part 3 · Ultimate Rust: Foundations - Next Steps` · Herbert Wolverson ·
code: `thebracket/ArdanUltimateRust-5Days` (`05-Server`, link only)

### Section 0 — `Introduction`
| display | title | type | video |
|---|---|---|---|
| 3.0.1 | Ultimate Rust Foundations: Next Steps | Text | — |

### Section 1 — `Ultimate Rust: Foundations - Next Steps`
| display | title | type | video |
|---|---|---|---|
| 3.1.1 | 1.0 - Bi-Directional Communication | Video | 12:16 |
| 3.1.2 | 1.1 - Sending Commands & Prevent Unbounded Growth | Video | 0:43 |
| 3.1.3 | 1.2 - Giving the Collector a Diet | Video | 16:39 |
| 3.1.4 | 1.3 - Optimizing File Reading | Video | 6:56 |
| 3.1.5 | 1.4 - Adapting Files In Flight | Video | 14:09 |
| 3.1.6 | Rust Foundations - Next Steps - Review | PDF | — |
| 3.1.7 | Quiz - Rust Foundations - Next Steps | Quiz | — |

---

## Part 4 · Ultimate Rust Best Practices

`@@part 4 · Ultimate Rust Best Practices` · Herbert Wolverson ·
no public exercise repo found (topics get our own exercises)

### Section 0 — `Introduction`
| display | title | type | video |
|---|---|---|---|
| 4.0.1 | Ultimate Rust Best Practices | Text | — |
| 4.0.2 | The Best Practices Intro | Video | 1:12 |

### Section 1 — `Module 1: Tooling`
| display | title | type | video |
|---|---|---|---|
| 4.1.1 | 1.0 - Introduction - Formatting | Video | 8:28 |
| 4.1.2 | 1.1 - Clippy (the linter) | Video | 6:07 |
| 4.1.3 | 1.2 - Documentation | Video | 9:08 |
| 4.1.4 | 1.3 - Understanding Dependencies | Video | 4:57 |
| 4.1.5 | 1.4 - Managing Your Own Dependencies | Video | 5:06 |
| 4.1.6 | 1.5 - Checking for Vulnerabilities | Video | 1:45 |
| 4.1.7 | 1.6 - Check for Outdated Dependencies | Video | 1:42 |
| 4.1.8 | 1.7 - Denying Dependencies by Licensing | Video | 5:04 |
| 4.1.9 | Module 1 - Review | PDF | — |
| 4.1.10 | Module 1 - Quiz | Quiz | — |

### Section 2 — `Module 2 : Code Best Practices`
| display | title | type | video |
|---|---|---|---|
| 4.2.1 | 2.0 - Favor Iterators | Video | 8:07 |
| 4.2.2 | 2.1 - Minimize Cloning (we have an alternative name as well Cloning Can Be a Code Smell) | Video | 2:16 |
| 4.2.3 | 2.2 - Don’t Emulate OOP | Video | 5:08 |
| 4.2.4 | 2.3 - Favor Small Functions | Video | 3:21 |
| 4.2.5 | 2.4 - Clever Code | Video | 5:23 |
| 4.2.6 | 2.5 - Floating Point Numbers | Video | 4:15 |
| 4.2.7 | 2.6 - Platform & Feature Specific Code | Video | 6:20 |
| 4.2.8 | Module 2 - Review | PDF | — |
| 4.2.9 | Module 2 - Quiz | Quiz | — |

### Section 3 — `Module 3 : General Best Practices`
| display | title | type | video |
|---|---|---|---|
| 4.3.1 | 3.0 - TANSTAAFL (There Ain’t No Such Thing As A Free Lunch) | Video | 3:58 |
| 4.3.2 | 3.1 - YAGNI : You Ain’t Gonna Need It | Video | 4:34 |
| 4.3.3 | 3.2 - Domain Boundaries | Video | 4:50 |
| 4.3.4 | 3.3 - Taming Compile Times | Video | 2:50 |
| 4.3.5 | Module 3 - Quiz | Quiz | — |
| 4.3.6 | Module 3 - Review | PDF | — |

### Section 4 — `Outro`
| display | title | type | video |
|---|---|---|---|
| 4.4.1 | The Best Practices Outro | Video | 4:31 |

### Section 5 — `Full Course Review`
| display | title | type | video |
|---|---|---|---|
| 4.5.1 | Full Final Review | PDF | — |
| 4.5.2 | Ultimate Rust Best Practices Final Exam | Quiz | — |

---

## Part 5 · Ultimate Rust 2: Intermediate Concepts

`@@part 5 · Ultimate Rust 2: Intermediate Concepts` · Nathan Stocks ·
exercises: `CleanCut/ultimate_rust2` (MIT) · game engine: `CleanCut/rusty_engine`

### Section 0 — `Ultimate Rust 2: Intermediate Concepts`
| display | title | type | video |
|---|---|---|---|
| 5.0.1 | Ultimate Rust 2: Intermediate Concepts | Text | — |

### Section 1 — `Module 1: Introduction`
| display | title | type | video |
|---|---|---|---|
| 5.1.1 | Lesson 1.1 - Introduction | Video | 2:08 |
| 5.1.2 | Lesson 1.2 - Exercises Overview | Video | 1:10 |
| 5.1.3 | Module 1 - Review | PDF | — |
| 5.1.4 | Module 1 - Quiz | Quiz | — |

### Section 2 — `Module 2: Lectures & Exercises`
| display | title | type | video |
|---|---|---|---|
| 5.2.1 | Lesson 2.1 - Idiomatic Code | Video | 6:30 |
| 5.2.2 | Lesson 2.2 - Exercise - Idiomatic Code | Video | 9:18 |
| 5.2.3 | Lesson 2.3 - Documentation | Video | 7:08 |
| 5.2.4 | Lesson 2.4 - Exercise - Documentation | Video | 10:06 |
| 5.2.5 | Lesson 2.5 - Publishing | Video | 5:50 |
| 5.2.6 | Lesson 2.6 - Exercise - Publishing | Video | 0:49 |
| 5.2.7 | Lesson 2.7 - Iterators | Video | 10:34 |
| 5.2.8 | Lesson 2.8 - Closures | Video | 3:17 |
| 5.2.9 | Lesson 2.9 - Exercise - Closures & Iterators | Video | 9:50 |
| 5.2.10 | Lesson 2.10 - Common Traits | Video | 13:26 |
| 5.2.11 | Lesson 2.11 - Exercise - Traits | Video | 10:20 |
| 5.2.12 | Lesson 2.12 - Creating Errors | Video | 8:28 |
| 5.2.13 | Lesson 2.13 - Handling Errors | Video | 11:44 |
| 5.2.14 | Lesson 2.14 - Exercise - Errors | Video | 7:50 |
| 5.2.15 | Lesson 2.15 - Unit Tests | Video | 10:40 |
| 5.2.16 | Lesson 2.16 - Integration Tests | Video | 2:26 |
| 5.2.17 | Lesson 2.17 - Benchmarks | Video | 6:29 |
| 5.2.18 | Lesson 2.18 - Exercise - Testing | Video | 8:49 |
| 5.2.19 | Lesson 2.19 - Logging | Video | 7:21 |
| 5.2.20 | Lesson 2.20 - Exercise - Logging | Video | 7:17 |
| 5.2.21 | Lesson 2.21 - Multithreading | Video | 8:40 |
| 5.2.22 | Lesson 2.22 - Channels | Video | 10:55 |
| 5.2.23 | Lesson 2.23 - Exercise - Threads & Channels | Video | 13:52 |
| 5.2.24 | Module 2 - Review | PDF | — |
| 5.2.25 | Module 2 - Quiz | Quiz | — |

### Section 3 — `Module 3: Project - Learn Rusty Engine`
| display | title | type | video |
|---|---|---|---|
| 5.3.1 | Lesson 3.1 - Project Overview | Video | 1:55 |
| 5.3.2 | Lesson 3.2 - Configuration | Video | 2:54 |
| 5.3.3 | Lesson 3.3 - Engine Initialization | Video | 1:04 |
| 5.3.4 | Lesson 3.4 - Game State | Video | 2:41 |
| 5.3.5 | Lesson 3.5 - Game Logic Function | Video | 2:12 |
| 5.3.6 | Lesson 3.6 - Sprites | Video | 6:45 |
| 5.3.7 | Lesson 3.7 - Colliders | Video | 10:04 |
| 5.3.8 | Lesson 3.8 - Keyboard Input | Video | 3:24 |
| 5.3.9 | Lesson 3.9 - Mouse Input | Video | 2:51 |
| 5.3.10 | Lesson 3.10 - Text | Video | 4:45 |
| 5.3.11 | Lesson 3.11 - Audio | Video | 3:27 |
| 5.3.12 | Lesson 3.12 - Timer | Video | 3:07 |
| 5.3.13 | Lesson 3.13 - Engine & Game Structs | Video | 6:36 |
| 5.3.14 | Module 3 - Review | PDF | — |
| 5.3.15 | Module 3 - Quiz | Quiz | — |

### Section 4 — `Module 4: Game - Road Race`
| display | title | type | video |
|---|---|---|---|
| 5.4.1 | Lesson 4.1 - Common Setup | Video | 1:54 |
| 5.4.2 | Lesson 4.2 - Road Race | Video | 14:32 |
| 5.4.3 | Module 4 - Review | PDF | — |
| 5.4.4 | Module 4 - Quiz | Quiz | — |

### Section 5 — `Module 5: Game - Car Shoot`
| display | title | type | video |
|---|---|---|---|
| 5.5.1 | Lesson 5.1 - Car Shoot pt. 1 | Video | 9:49 |
| 5.5.2 | Lesson 5.2 - Car Shoot pt. 2 | Video | 11:14 |
| 5.5.3 | Lesson 5.3 - Car Shoot pt. 3 | Video | 10:27 |
| 5.5.4 | Lesson 5.4 - Car Shoot pt. 4 | Video | 9:40 |
| 5.5.5 | Module 5 - Review | PDF | — |
| 5.5.6 | Module 5 - Quiz | Quiz | — |

### Section 6 — `Full Course Review`
| display | title | type | video |
|---|---|---|---|
| 5.6.1 | Full Final Review | PDF | — |
| 5.6.2 | Ultimate Rust 2 Intermediate Concepts - Final Exam | Quiz | — |

---

## Part 6 · Rust as a Service

`@@part 6 · Rust as a Service` · Herbert Wolverson ·
code + manual: `thebracket/ArdanRustService` (no licence — link only)

### Section 0 — `Rust as a Service`
| display | title | type | video |
|---|---|---|---|
| 6.0.1 | Rust as a Service | Text | — |

### Section 1 — `Introduction`
| display | title | type | video |
|---|---|---|---|
| 6.1.1 | 1. Introduction | Video | 5:58 |

### Section 2 — `2. Building a REST Service`
| display | title | type | video |
|---|---|---|---|
| 6.2.1 | 2.0 REST Service | Text | — |
| 6.2.2 | 2.1 Minimal HTTP Server | Video | 9:09 |
| 6.2.3 | 2.2 Understanding the Service Stack | Video | 6:58 |
| 6.2.4 | 2.3 Extractors | Text | — |
| 6.2.5 | 2.3.1 Path Extraction | Video | 5:43 |
| 6.2.6 | 2.3.2 Query Extraction | Video | 4:10 |
| 6.2.7 | 2.3.3 Header Extraction & 2.3.4 More Extractors | Video | 6:29 |
| 6.2.8 | 2.4 Add a Simple Tower Layer: State | Video | 7:19 |
| 6.2.9 | 2.5 Add a Simple Tower Layer (Mutable State) | Video | 6:17 |
| 6.2.10 | 2.6 Multiple States | Video | 8:56 |
| 6.2.11 | 2.7 Quick Recap on State and Layers | Video | 2:44 |
| 6.2.12 | 2.8 Nesting Multiple Routers | Video | 5:37 |
| 6.2.13 | 2.9 Nested Routers with State | Video | 5:36 |
| 6.2.14 | 2.10 Calling Other Services | Video | 5:35 |
| 6.2.15 | 2.11 Returning Status Codes | Video | 2:44 |
| 6.2.16 | 2.12 Using IntoResponse and 2.13 Error Handling with IntoResponse | Video | 11:51 |
| 6.2.17 | 2.14 Quick Recap on Nesting, Making Calls and Responses | Video | 2:24 |
| 6.2.18 | 2.15 Serving Static Content with Tower | Video | 4:52 |
| 6.2.19 | 2.16 Simple Header-Based Authentication | Video | 4:13 |
| 6.2.20 | 2.17 Simple Header-Based Auth with Middleware | Video | 8:02 |
| 6.2.21 | 2.18 Middleware Auth with Injection | Video | 5:53 |
| 6.2.22 | 2.19 Selectively Applying Layers | Video | 2:17 |
| 6.2.23 | 2.20 Router Layers | Video | 7:05 |
| 6.2.24 | 2.21 Layer Recap | Video | 1:58 |

### Section 3 — `3. Tracing`
| display | title | type | video |
|---|---|---|---|
| 6.3.1 | 3.0 Tracing | Text | — |
| 6.3.2 | 3.1 Minimal Example | Video | 5:48 |
| 6.3.3 | 3.2 Logging Axum/Tower | Video | 3:14 |
| 6.3.4 | 3.3 Timing Spans | Video | 5:29 |
| 6.3.5 | 3.4 Axum Spans | Video | 5:51 |
| 6.3.6 | 3.5 Logging to a File | Video | 5:12 |
| 6.3.7 | 3.6 Structured Logging to JSON | Video | 2:19 |
| 6.3.8 | 3.7 OpenTelemetry & 3.7.1 Hello Telemetry | Video | 18:28 |

### Section 4 — `4. OpenAPI Documentation`
| display | title | type | video |
|---|---|---|---|
| 6.4.1 | 4.0 OpenAPI Documentation | Video | 11:40 |

### Section 5 — `5. Handling Service Configuration`
| display | title | type | video |
|---|---|---|---|
| 6.5.1 | 5.0 Handling Service Configuration | Text | — |
| 6.5.2 | 5.1 Environment Variables with .env | Video | 8:34 |
| 6.5.3 | 5.2 The Config Crate | Video | 11:38 |
| 6.5.4 | 5.3 Loading Config via HTTP | Video | 7:16 |
| 6.5.5 | 5.4 CLI configuration with Clap | Video | 10:07 |

### Section 6 — `6. gRPC`
| display | title | type | video |
|---|---|---|---|
| 6.6.1 | 6.0 gRPC | Text | — |
| 6.6.2 | 6.1 Hello Tonic | Video | 6:34 |
| 6.6.3 | 6.2 Hello Tonic - Project Definition and Build | Video | 8:25 |
| 6.6.4 | 6.3 Hello Tonic - The Server | Video | 8:28 |
| 6.6.5 | 6.4 Hello Tonic - The Client | Video | 5:03 |
| 6.6.6 | 6.5 gRPC Streaming and 6.6 Protocol Definition | Video | 4:18 |
| 6.6.7 | 6.7 gRPC Streaming - The Server | Video | 5:03 |
| 6.6.8 | 6.8 gRPC Streaming - The Client | Video | 3:24 |
| 6.6.9 | 6.9 Recap So Far | Video | 1:36 |
| 6.6.10 | 6.10 Authentication | Video | 8:09 |
| 6.6.11 | 6.11 Tracing | Video | 4:01 |
| 6.6.12 | 6.12 When to use gRPC | Video | 2:56 |

### Section 7 — `7. Web Sockets`
| display | title | type | video |
|---|---|---|---|
| 6.7.1 | 7.0 Web Sockets | Text | — |
| 6.7.2 | 7.1 Minimal Echo Server | Video | 8:52 |
| 6.7.3 | 7.2 A native WS client | Video | 3:11 |
| 6.7.4 | 7.3 JSON | Video | 7:12 |

### Section 8 — `8. Service Deployment`
| display | title | type | video |
|---|---|---|---|
| 6.8.1 | 8.0 Service Deployment | Video | 5:52 |
| 6.8.2 | 8.1 Build a Test Service & 8.2 Native Host Deployment | Video | 9:17 |
| 6.8.3 | 8.3 Docker Deployment | Video | 7:54 |

### Section 9 — `9. Service Design`
| display | title | type | video |
|---|---|---|---|
| 6.9.1 | 9.0 Service Design | Video | 5:08 |
| 6.9.2 | 9.1 Understanding Your Company Architecture | Video | 9:54 |
| 6.9.3 | 9.2 Designing Individual Services (9.2.1, 9.2.2, 9.2.3, 9.2.4, 9.2.5) | Video | 14:11 |
| 6.9.4 | 9.3 Combining Services into a Modular Monolith | Video | 3:42 |
| 6.9.5 | 9.4 Service Exposure | Video | 3:42 |
| 6.9.6 | 9.5 Scaling Out | Video | 7:32 |

### Section 10 — `10. Wrap up`
| display | title | type | video |
|---|---|---|---|
| 6.10.1 | 10.0 Wrap Up | Video | 0:54 |

---

## Part 7 · Rust Webinars

`@@part 7 · Rust Webinars` · Herbert Wolverson

| display | section (`@@section`) | title | type | video | companion repo |
|---|---|---|---|---|---|
| 7.1.1 | Introducing Rust into your Company Ecosystem | Introducing Rust into your Company Ecosystem | Video | 56:10 | `thebracket/ArdanLabs_RustInYourEnterprise` |
| 7.2.1 | Fearless Concurrency with Rust | Webinar with Herbert (Aug 27, 2024) | Video | 1:37:06 | — |
| 7.3.1 | 1 Hour Dive into Asynchronous Rust | Webinar with Herbert (Aug 2nd, 2023) | Video | 1:14:11 | `thebracket/Ardan-1HourAsync` |
| 7.4.1 | Rust Foundations Highlights Webinar | Rust Foundations: Herbert Wolverson | Video | 1:02:52 | — (Ardan slug: optimizing Rust for embedded devices) |
| 7.5.1 | Debugging & Optimizing Rust | Webinar with Herbert Wolverson (January, 2023) | Video | 1:00:36 | `thebracket/ArdanRustDebugOptimize` |

---

## Totals

| part | chapters |
|---|---|
| 0 Start Here | 4 |
| 1 Crash Course | 45 |
| 2 Foundations | 89 |
| 3 Next Steps | 8 |
| 4 Best Practices | 30 |
| 5 Ultimate Rust 2 | 57 |
| 6 Rust as a Service | 66 |
| 7 Webinars | 5 |
| **total** | **304** |
