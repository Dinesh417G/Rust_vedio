# Ardan exercise map

**Mapped rows: 254.** This equals the number of rows in `CURRICULUM.md` whose type is `Video` or `Text` in parts 1, 2, 3, 4, 5, 6 and 7 (33 + 77 + 6 + 22 + 45 + 66 + 5 = 254). PDF and Quiz rows are not mapped, and Part 0 has no Video or Text row, so it has no table. In Part 7 the type is the fourth column (that table has an extra section column); all 5 rows are Video.

Written 2026-10-09 by task t-5-exercise-map. Each lesson row of `CURRICULUM.md` is mapped to the exercise folder, code folder or companion markdown in the official Ardan-linked GitHub repos, cloned (shallow) to `D:\RustroverProjects\learn_rust\ardan\<repo>`.

## How to read the tables

- `display` and `title` are copied from `CURRICULUM.md`.
- `repo` is the folder name under `D:\RustroverProjects\learn_rust\ardan\` (`-` means no repo material).
- `path(s) in repo` are repo-relative with forward slashes, each in backticks, several separated by `<br>`. Every one of them exists in the local clone (checked against `git ls-files`). A `-` means there is no path.
- `licence` is what the repo actually carries (details and the file read are in the Licences section).
- `use` is one of:
  - `embed`: MIT or MIT/Apache material (starter code, exercise, example) that may be copied with attribution, as STYLE.md describes for Nathan Stocks's repos.
  - `link`: unlicensed repo (Herbert Wolverson): give the GitHub URL, the local path and which folder or file to open, in our own words. Never copy code or text.
  - `own`: no exercise material in a repo; the writer makes an own exercise. A path may still be listed as background reading (text only, no starter code).
- `note` is a short neutral description. Where a match is a guess it says so (UNVERIFIED or Guess), because the videos are not in the repos.
- All eight repos use the default branch `main`. File URL: `https://github.com/<owner>/<repo>/blob/main/<path>`, folder URL: `https://github.com/<owner>/<repo>/tree/main/<path>`. The owner is `CleanCut` for `ultimate_rust_crash_course`, `ultimate_rust2` and `rusty_engine`, and `thebracket` for the other five.

### Clones (snapshot of 2026-10-09)

The map is valid for exactly these commits (shallow clones, `--depth 1`, under `D:\RustroverProjects\learn_rust\ardan\`):

| repo | owner | HEAD (12 chars) | rows mapped into it |
|---|---|---|---|
| `ultimate_rust_crash_course` | CleanCut | `4fa069c94c7d` | 17 |
| `ultimate_rust2` | CleanCut | `5e852ca5d535` | 20 |
| `rusty_engine` | CleanCut | `95ef835fc69b` | 19 |
| `ArdanUltimateRust-5Days` | thebracket | `1c5ef96681d0` | 81 |
| `ArdanRustService` | thebracket | `dcf3aca4d0b0` | 66 |
| `Ardan-1HourAsync` | thebracket | `e162ca7ac1a0` | 3 |
| `ArdanLabs_RustInYourEnterprise` | thebracket | `9a370caf9a96` | 1 |
| `ArdanRustDebugOptimize` | thebracket | `b11c1e989545` | 1 |

### Rows per part and use

| part | rows | embed | link | own |
|---|---|---|---|---|
| 1 | 33 | 12 | 0 | 21 |
| 2 | 77 | 0 | 77 | 0 |
| 3 | 6 | 0 | 6 | 0 |
| 4 | 22 | 0 | 0 | 22 |
| 5 | 45 | 35 | 0 | 10 |
| 6 | 66 | 0 | 66 | 0 |
| 7 | 5 | 0 | 3 | 2 |
| total | 254 | 47 | 152 | 55 |

### Best Practices search (Part 4)

Three bounded searches on 2026-10-09, no repo found, so Part 4 rows are `own`:

1. `gh api users/thebracket/repos` (all public repos of the author, sorted by push date): no repo is named or described as best practices. Closest by description: `rusttraining` (Apache-2.0, described as code for blogs and training) and the older `ArdanUltimateRustFoundations` (Foundations material). Neither was cloned or opened.
2. `gh search repos "rust best practices" --owner thebracket`: no results.
3. `gh search repos "ultimate rust best practices ardan"` (all of GitHub): no results.

## Part 1 · Ultimate Rust Crash Course

Repo `ultimate_rust_crash_course` (MIT). Exercises A-K are in `exercise/`, lecture crate in `examples/hello`.

| display | title | repo | path(s) in repo | licence | use | note |
|---|---|---|---|---|---|---|
| 1.0.1 | Ultimate Rust Crash Course | `ultimate_rust_crash_course` | `README.md` | MIT | own | Course intro page. README.md lists exercises A-K and the capstone; text only, no starter code. |
| 1.1.1 | Lesson 1.1 - Introduction | - | - | - | own | Intro video; no repo material. |
| 1.1.2 | Lesson 1.2 - Exercises Overview | `ultimate_rust_crash_course` | `README.md` | MIT | own | README section Exercises: open one exercise folder, run cargo there, follow the numbered comments in src/main.rs. Text only. |
| 1.1.3 | Lesson 1.3 - Installation | `ultimate_rust_crash_course` | `README.md` | MIT | own | README section Install Rust (rustup, version check). Text only; write the Windows steps in own words. |
| 1.1.4 | Lesson 1.4 - Editor/IDE | `ultimate_rust_crash_course` | `HowToLearnRust.md`<br>`README.md` | MIT | own | HowToLearnRust.md (IDE / Editor, Tools) and the README checklist Prepare Your Development Environment. Text only. |
| 1.2.1 | Lesson 2.1 - Cargo | `ultimate_rust_crash_course` | `examples/hello/Cargo.toml`<br>`README.md` | MIT | own | No exercise. Reference only: examples/hello/Cargo.toml (one dependency, rand) and the README mini task (cargo new message, cargo run, edit, run again). |
| 1.2.2 | Lesson 2.2 - Variables | - | - | - | own | Concept lesson; exercise A comes in 1.2.5. |
| 1.2.3 | Lesson 2.3 - Scope | - | - | - | own | Concept lesson; exercise A comes in 1.2.5. |
| 1.2.4 | Lesson 2.4 - Memory Safety | - | - | - | own | Concept lesson; no repo material. |
| 1.2.5 | Lesson 2.5 - Exercise A - Variables | `ultimate_rust_crash_course` | `exercise/a_variables`<br>`exercise/a_variables/README.md`<br>`exercise/a_variables/ANSWERS.md` | MIT | embed | README-only exercise: the folder has no Cargo project, the learner runs cargo new variables. ANSWERS.md is the solution; keep it out of the starter. |
| 1.2.6 | Lesson 2.6 - Functions | - | - | - | own | Concept lesson; exercise B comes in 1.2.7. |
| 1.2.7 | Lesson 2.7 - Exercise B - Functions | `ultimate_rust_crash_course` | `exercise/b_functions`<br>`exercise/b_functions/src/main.rs` | MIT | embed | Four numbered steps: shadowing, a scope fix, writing an add function, late initialisation. Instructions are comments in src/main.rs. |
| 1.2.8 | Lesson 2.8 - Module System | `ultimate_rust_crash_course` | `examples/hello`<br>`examples/hello/src/lib.rs`<br>`examples/hello/src/russian.rs`<br>`examples/hello/src/russian/authors.rs`<br>`examples/hello/src/spanish.rs` | MIT | embed | Lecture crate for modules: lib.rs declares english, russian and spanish; russian has a nested authors module. The exercise follows in 1.2.9. |
| 1.2.9 | Lesson 2.9 - Exercise C - Module System | `ultimate_rust_crash_course` | `exercise/c_modules`<br>`exercise/c_modules/src/main.rs` | MIT | embed | Two steps: move constants into a library (lib.rs, pub, use), then create a sound module for the animal functions. |
| 1.3.1 | Lesson 3.1 - Scalar Types | - | - | - | own | Concept lesson; exercise D comes in 1.3.3. |
| 1.3.2 | Lesson 3.2 - Compound Types | - | - | - | own | Concept lesson; exercise D comes in 1.3.3. |
| 1.3.3 | Lesson 3.3 - Exercise D - Simple Types | `ultimate_rust_crash_course` | `exercise/d_simple_types`<br>`exercise/d_simple_types/src/main.rs` | MIT | embed | Five steps on tuples, arrays and scalar arguments. Instructions are comments in src/main.rs. |
| 1.3.4 | Lesson 3.4 - Control Flow | - | - | - | own | Concept lesson; exercise E comes in 1.3.5. |
| 1.3.5 | Lesson 3.5 - Exercise E - Control Flow | `ultimate_rust_crash_course` | `exercise/e_control_flow`<br>`exercise/e_control_flow/src/main.rs` | MIT | embed | Four steps: loop with break, for over a range, while, if / else if / else. |
| 1.3.6 | Lesson 3.6 - Strings | - | - | - | own | Concept lesson; exercise F comes in 1.3.8. |
| 1.3.7 | Lesson 3.7 - String Literals | - | - | - | own | Concept lesson; exercise F comes in 1.3.8. |
| 1.3.8 | Lesson 3.8 - Exercise F - Strings | `ultimate_rust_crash_course` | `exercise/f_strings`<br>`exercise/f_strings/src/main.rs` | MIT | embed | Three steps on string literals, unicode escapes and newlines. No README in this folder; all text is in src/main.rs. |
| 1.4.1 | Lesson 4.1 - Ownership | - | - | - | own | Concept lesson; exercise G comes in 1.4.3. |
| 1.4.2 | Lesson 4.2 - References & Borrowing | - | - | - | own | Concept lesson; exercise G comes in 1.4.3. |
| 1.4.3 | Lesson 4.3 - Exercise G - Ownership & References | `ultimate_rust_crash_course` | `exercise/g_ownership_references`<br>`exercise/g_ownership_references/src/main.rs` | MIT | embed | Write three functions: one takes a shared reference, one a mutable reference, one takes ownership. Needs a command-line argument, e.g. cargo run apple. |
| 1.5.1 | Lesson 5.1 - Structs | - | - | - | own | Concept lesson; exercise H comes in 1.5.2. |
| 1.5.2 | Lesson 5.2 - Exercise H - Structs | `ultimate_rust_crash_course` | `exercise/h_structs`<br>`exercise/h_structs/src/main.rs` | MIT | embed | Define a Polygon struct, then an impl block with a constructor and methods. No README in this folder. |
| 1.5.3 | Lesson 5.3 - Traits | - | - | - | own | Concept lesson; exercise I comes in 1.5.4. |
| 1.5.4 | Lesson 5.4 - Exercise I - Traits | `ultimate_rust_crash_course` | `exercise/i_traits`<br>`exercise/i_traits/src/main.rs` | MIT | embed | Define a trait, implement it for a struct and for i32, then replace a helper function with a trait method. |
| 1.5.5 | Lesson 5.5 - Collections | - | - | - | own | Concept lesson; exercise J comes in 1.5.6. |
| 1.5.6 | Lesson 5.6 - Exercise J - Collections | `ultimate_rust_crash_course` | `exercise/j_collections`<br>`exercise/j_collections/src/main.rs` | MIT | embed | Work with a Vec of Strings: create, len, sort, loop over it. |
| 1.5.7 | Lesson 5.7 - Enums | - | - | - | own | Concept lesson; exercise K comes in 1.5.8. |
| 1.5.8 | Lesson 5.8 - Exercise K - Enums | `ultimate_rust_crash_course` | `exercise/k_enums`<br>`exercise/k_enums/src/main.rs`<br>`exercise/z_final_project`<br>`exercise/z_final_project/src/main.rs` | MIT | embed | k_enums: Option basics, a function using if let, a loop, then an enum Snack with a price method. ALSO carries the course capstone exercise/z_final_project (crate mirage: image processing, uses the image crate, sample images included). Attached here because this is the last Video topic of Part 1 and 1.6.x (Full Course Review) are PDF and Quiz rows with no topic page; add it at the end of this topic as an optional Final project. |

## Part 2 · Ultimate Rust Foundations

Repo `ArdanUltimateRust-5Days` (no licence, link only). Pages are `01-GettingStarted` ... `05-Server`, code is in `code/`.

| display | title | repo | path(s) in repo | licence | use | note |
|---|---|---|---|---|---|---|
| 2.0.1 | Ultimate Rust Foundations | `ArdanUltimateRust-5Days` | `README.md` | none | link | Text page for the five-session class; the repo README only introduces the sessions. No code. |
| 2.1.1 | 1.0 - Introduction | `ArdanUltimateRust-5Days` | `01-GettingStarted/ClassIntro.md`<br>`01-GettingStarted/README.md` | none | link | Class intro and the hour-by-hour outline of session 1. No code. |
| 2.1.2 | 1.1 - Setup & Update Rust | `ArdanUltimateRust-5Days` | `01-GettingStarted/SetupRust.md` | none | link | rustup install and update, toolchains, installing Clippy. No code folder. |
| 2.1.3 | 1.2 - Setup Dev Environment | `ArdanUltimateRust-5Days` | `01-GettingStarted/HelloWorld.md`<br>`01-GettingStarted/Workspaces.md`<br>`code/hello_world` | none | link | Assumed from the README order (cargo init and hello world, then workspaces): Ardan has no lesson of its own for those two pages. Check against the video. |
| 2.1.4 | 1.3 - IDE Configuration | `ArdanUltimateRust-5Days` | `01-GettingStarted/SetupEditor.md` | none | link | IDE extensions and settings (VS Code in the source). No code. |
| 2.1.5 | 1.4 - Rust Fundamentals | `ArdanUltimateRust-5Days` | `01-GettingStarted/RustFundamentals.md`<br>`code/vars_mut_shadow`<br>`code/function_returns`<br>`code/borrow_reference` | none | link | One code folder per page section: variables and shadowing, functions and returns, borrowing. The md names all three. |
| 2.1.6 | 1.5 - Console Text Input | `ArdanUltimateRust-5Days` | `01-GettingStarted/TextInput.md`<br>`code/text_input` | none | link | The md names the code folder. |
| 2.1.7 | 1.6 - Create a Library | `ArdanUltimateRust-5Days` | `01-GettingStarted/CreateLibrary.md`<br>`code/auth`<br>`code/login` | none | link | The md's own code link points at a folder that is not in the repo. auth and login hold the later login-system state (see 2.1.8), not the first greeting-function step. |
| 2.1.8 | 1.7- World’s Simplest Login System | `ArdanUltimateRust-5Days` | `01-GettingStarted/SimpleLogin.md`<br>`code/auth`<br>`code/login` | none | link | The md names the auth library and the login app. |
| 2.1.9 | 1.8 - Enumerations | `ArdanUltimateRust-5Days` | `01-GettingStarted/Enumerations.md`<br>`code/auth_enum`<br>`code/login_enum`<br>`code/auth_enum_data`<br>`code/login_enum_data`<br>`code/auth_enum_option`<br>`code/login_enum_option` | none | link | Three snapshots (plain enum, enum with data, Option); each is an auth_ library plus a login_ app. |
| 2.1.10 | 1.9 - Structures | `ArdanUltimateRust-5Days` | `01-GettingStarted/Structures.md`<br>`code/auth_struct`<br>`code/login_struct` | none | link | The md names the code folder. |
| 2.1.11 | 1.10 - Vectors | `ArdanUltimateRust-5Days` | `01-GettingStarted/Vectors.md`<br>`code/auth_vec`<br>`code/login_vec`<br>`code/vector_growth` | none | link | vector_growth is the side project for the growth section. |
| 2.1.12 | 1.11 - HashMaps | `ArdanUltimateRust-5Days` | `01-GettingStarted/HashMaps.md`<br>`code/auth_hashmap`<br>`code/login_hashmap`<br>`code/hash_vec_bench`<br>`code/hash_vec_search` | none | link | hash_vec_bench and hash_vec_search time Vec against HashMap; run them with --release. |
| 2.1.13 | 1.12 - Serialization/Deserialization | `ArdanUltimateRust-5Days` | `01-GettingStarted/Serialization.md`<br>`code/auth_json`<br>`code/login_json` | none | link | The md names the code folder. |
| 2.1.14 | 1.13 - Hashing Passwords | `ArdanUltimateRust-5Days` | `01-GettingStarted/Hashing.md`<br>`code/auth_hash`<br>`code/login_hash` | none | link | The md names the code folder. |
| 2.1.15 | 1.14 - Start a CLI App | `ArdanUltimateRust-5Days` | `01-GettingStarted/Cli.md`<br>`code/auth_login_manager`<br>`code/login_manager` | none | link | Login manager CLI (add, delete, change password); the md builds the project step by step. |
| 2.2.1 | 2.0 - Introduction | `ArdanUltimateRust-5Days` | `02-SystemThreads/Intro.md`<br>`02-SystemThreads/README.md` | none | link | Class intro and outline of session 2. No code. |
| 2.2.2 | 2.1 - System Threads: Overview | `ArdanUltimateRust-5Days` | `02-SystemThreads/SystemThreadOverview.md` | none | link | Concept page. No code. |
| 2.2.3 | 2.2 - Create Your First Thread | `ArdanUltimateRust-5Days` | `02-SystemThreads/FirstThread.md`<br>`code/02_threads/first_thread` | none | link | The md names the code folder. |
| 2.2.4 | 2.3 - Spawning Threads with Parameters and Closures | `ArdanUltimateRust-5Days` | `02-SystemThreads/ThreadClosures.md`<br>`code/02_threads/thread_closures` | none | link | The md names the code folder. |
| 2.2.5 | 2.4 - Returning Data from Threads | `ArdanUltimateRust-5Days` | `02-SystemThreads/ReturnFromThreads.md`<br>`code/02_threads/thread_return` | none | link | The md names the code folder. |
| 2.2.6 | 2.5 - Dividing Workloads | `ArdanUltimateRust-5Days` | `02-SystemThreads/DividingWorkloads.md`<br>`code/02_threads/divide_workload` | none | link | The md names the code folder. |
| 2.2.7 | 2.6 - The ThreadBuilder Pattern | `ArdanUltimateRust-5Days` | `02-SystemThreads/ThreadBuilder.md`<br>`code/02_threads/thread_builder` | none | link | The md names the code folder. |
| 2.2.8 | 2.7 - Scoped Threads for Easy Local Data Sharing | `ArdanUltimateRust-5Days` | `02-SystemThreads/ScopedThreads.md`<br>`code/02_threads/scoped_threads` | none | link | The md names the code folder. |
| 2.2.9 | 2.8 - Sharing Data with Atomics | `ArdanUltimateRust-5Days` | `02-SystemThreads/Atomics.md`<br>`code/02_threads/data_race`<br>`code/02_threads/atomic_counter`<br>`code/02_threads/atomic_counter_timed` | none | link | data_race shows the problem, the other two the fix and a timed comparison. |
| 2.2.10 | 2.9 - Sharing Data with Mutexes | `ArdanUltimateRust-5Days` | `02-SystemThreads/Mutexes.md`<br>`code/02_threads/mutex`<br>`code/02_threads/mutex_timed` | none | link | mutex_timed measures how fast mutexes are. |
| 2.2.11 | 2.10 - Read/Write Locks | `ArdanUltimateRust-5Days` | `02-SystemThreads/ReadWriteLocks.md`<br>`code/02_threads/rwlock` | none | link | The md names the code folder. |
| 2.2.12 | 2.11 - Deadlocks, Panics and Poisoning | `ArdanUltimateRust-5Days` | `02-SystemThreads/Deadlocks.md`<br>`code/02_threads/deadlocks`<br>`code/02_threads/mutex_poisoning` | none | link | The md names the code folder. |
| 2.2.13 | 2.12 - Sharing Data with Lock-Free Structures | `ArdanUltimateRust-5Days` | `02-SystemThreads/LockFree.md`<br>`code/02_threads/lockfree_map` | none | link | The md names the code folder. |
| 2.2.14 | 2.13 - Parking Threads | `ArdanUltimateRust-5Days` | `02-SystemThreads/ParkingThreads.md`<br>`code/02_threads/parking` | none | link | The md names the code folder. |
| 2.2.15 | 2.14 - Sending Data Between Threads with Channels | `ArdanUltimateRust-5Days` | `02-SystemThreads/Channels.md`<br>`02-SystemThreads/ChannelOwnership.md`<br>`code/02_threads/mpsc`<br>`code/02_threads/mpsc_ownership`<br>`code/02_threads/mpsc_ownership_timed` | none | link | ChannelOwnership.md has no Ardan lesson of its own; attached here by README order. The two mpsc_ownership folders are matched by name, the md names no folder. |
| 2.2.16 | 2.15 - Sending Functions to Worker Threads | `ArdanUltimateRust-5Days` | `02-SystemThreads/SendingFunctions.md`<br>`code/02_threads/sending_functions`<br>`code/02_threads/sending_commands_and_functions` | none | link | The md names the code folder. |
| 2.2.17 | 2.16 - Let’s build a work queue with a thread pool | `ArdanUltimateRust-5Days` | `02-SystemThreads/WorkQueue.md`<br>`code/02_threads/work_queue` | none | link | The md names the code folder. |
| 2.2.18 | 2.17 - Thread CPU/Core Affinity | `ArdanUltimateRust-5Days` | `02-SystemThreads/ThreadAffinity.md`<br>`code/02_threads/thread_affinity` | none | link | The md names the code folder. |
| 2.2.19 | 2.18 - Thread Priority | `ArdanUltimateRust-5Days` | `02-SystemThreads/ThreadPriority.md`<br>`code/02_threads/thread_priorities` | none | link | The md names the code folder. |
| 2.2.20 | 2.19 - Making it Easy with Rayon | `ArdanUltimateRust-5Days` | `02-SystemThreads/Rayon.md`<br>`code/02_threads/rayon_par_iter` | none | link | The md names the code folder. |
| 2.2.21 | 2.20 - Scopes and Pooled Threads with Rayon | `ArdanUltimateRust-5Days` | `02-SystemThreads/RayonScopes.md`<br>`code/02_threads/rayon_scopes`<br>`code/02_threads/rayon_nested_pools`<br>`code/02_threads/rayon_broadcast`<br>`code/02_threads/rayon_join` | none | link | The md names the code folder. |
| 2.3.1 | 3.0 - Introduction / Async Overview | `ArdanUltimateRust-5Days` | `03-Async/AsyncAwaitOverview.md` | none | link | Concept page (async versus system threads). No code. |
| 2.3.2 | 3.1 - Hello Async/Await | `ArdanUltimateRust-5Days` | `03-Async/HelloAsync.md`<br>`code/03_async/hello_async_futures`<br>`code/03_async/hello_async_spawn_futures` | none | link | The md names the code folder. |
| 2.3.3 | 3.2 - Getting Started with Tokio | `ArdanUltimateRust-5Days` | `03-Async/Executors.md`<br>`03-Async/TokioIntro.md`<br>`code/03_async/tokio_single_thread_manual`<br>`code/03_async/tokio_single_thread_macro`<br>`code/03_async/tokio_mt_manual`<br>`code/03_async/tokio_mt_macro` | none | link | TokioIntro.md has four sections (single-threaded manual and macro, multi-threaded manual and macro); the tokio_mt folders are matched by name. Executors.md is 11 lines with no Ardan lesson of its own; attached by README order. |
| 2.3.4 | 3.3 - Working with Tokio Futures: Awaiting, Yielding and Spawning | `ArdanUltimateRust-5Days` | `03-Async/TokioFutures.md`<br>`code/03_async/tokio_await` | none | link | The md names the code folder. |
| 2.3.5 | 3.4 - Blocking Tasks | `ArdanUltimateRust-5Days` | `03-Async/Blocking.md`<br>`code/03_async/tokio_thread_sleep`<br>`code/03_async/tokio_spawn_blocking` | none | link | The md names the code folder. |
| 2.3.6 | 3.5 - Unit Testing Tokio | `ArdanUltimateRust-5Days` | `03-Async/TokioTesting.md`<br>`code/03_async/tokio_unit_tests` | none | link | The md names no folder; tokio_unit_tests is matched by name. |
| 2.3.7 | 3.6 - Handling Errors | `ArdanUltimateRust-5Days` | `03-Async/ErrorHandling.md`<br>`code/03_async/rust_errors1`<br>`code/03_async/rust_errors2`<br>`code/03_async/rust_errors3`<br>`code/03_async/rust_errors_async` | none | link | The md names the code folder. |
| 2.3.8 | 3.7 - File I/O | `ArdanUltimateRust-5Days` | `03-Async/FileIO.md`<br>`code/03_async/buffered_reader` | none | link | The md names the code folder. |
| 2.3.9 | 3.8 - Basic Network I/O | `ArdanUltimateRust-5Days` | `03-Async/BasicNetworkIO.md`<br>`code/03_async/weather`<br>`code/03_async/tcp_echo`<br>`code/03_async/tcp_echo_client` | none | link | The md names the code folder. |
| 2.3.10 | 3.9 - Async Channels (Tokio) | `ArdanUltimateRust-5Days` | `03-Async/AsyncChannels.md`<br>`code/03_async/sync_channel`<br>`code/03_async/sync_channel_reply`<br>`code/03_async/broadcast` | none | link | The md names the code folder. |
| 2.3.11 | 3.10 - Shared State (Tokio) | `ArdanUltimateRust-5Days` | `03-Async/SharedState.md`<br>`code/03_async/async_mutex` | none | link | The md names the code folder. |
| 2.3.12 | 3.11 - Selecting Futures | `ArdanUltimateRust-5Days` | `03-Async/SelectingFutures.md`<br>`code/03_async/select_timeout`<br>`code/03_async/select_channels` | none | link | The md names the code folder. |
| 2.3.13 | 3.12 - Pinning | `ArdanUltimateRust-5Days` | `03-Async/Pinning.md`<br>`code/03_async/recursion`<br>`code/03_async/pinning` | none | link | The md names the code folder. |
| 2.3.14 | 3.13 - Tokio Tracing | `ArdanUltimateRust-5Days` | `03-Async/TokioTracing.md`<br>`code/03_async/tokio_tracing`<br>`code/03_async/tokio_console_demo` | none | link | The md names the code folder. |
| 2.3.15 | 3.14 - Working with Databases | `ArdanUltimateRust-5Days` | `03-Async/Databases.md`<br>`code/03_async/database` | none | link | The md names the code folder. |
| 2.3.16 | 3.15 - Axum - A Web Framework built on Tokio | `ArdanUltimateRust-5Days` | `03-Async/Axum.md`<br>`code/03_async/hello_web` | none | link | The md names the code folder. |
| 2.3.17 | 3.16 - Let’s Build a Thumbnail Server | `ArdanUltimateRust-5Days` | `03-Async/ThumbnailServer.md`<br>`code/03_async/thumbnail_server` | none | link | The md names the code folder. |
| 2.4.1 | 4.0 - Why Haven’t We Manually Managed Any Memory Yet? | `ArdanUltimateRust-5Days` | `04-Memory/ManualMemoryManagement.md` | none | link | Concept page. No code. |
| 2.4.2 | 4.1 - The unsafe Keyword | `ArdanUltimateRust-5Days` | `04-Memory/Unsafe.md` | none | link | Concept page. No code folder. |
| 2.4.3 | 4.2 - Low-Level Memory Management | `ArdanUltimateRust-5Days` | `04-Memory/MemoryAllocFree.md`<br>`code/04_mem/libc_malloc` | none | link | The md names the code folder. |
| 2.4.4 | 4.3 - The Drop Trait & RAII (Resource Acquisition is Initialization) | `ArdanUltimateRust-5Days` | `04-Memory/DropTrait.md`<br>`code/04_mem/drop_trait`<br>`code/04_mem/smart_ptr` | none | link | The md names the code folder. |
| 2.4.5 | 4.4 - Reference Counting | `ArdanUltimateRust-5Days` | `04-Memory/ReferenceCounting.md`<br>`code/04_mem/refcount`<br>`code/04_mem/atomicrefcount`<br>`code/04_mem/external_arc_mutex`<br>`code/04_mem/internal_arc_mutex`<br>`code/04_mem/refcell` | none | link | The md names the code folder. |
| 2.4.6 | 4.5 - Lifetimes | `ArdanUltimateRust-5Days` | `04-Memory/Lifetimes.md` | none | link | No code folder; the md carries the examples. |
| 2.4.7 | 4.6 - Traits | `ArdanUltimateRust-5Days` | `04-Memory/Traits.md`<br>`code/04_mem/make_trait`<br>`code/04_mem/operator_overload` | none | link | The md names the code folder. |
| 2.4.8 | 4.7 - Generics | `ArdanUltimateRust-5Days` | `04-Memory/Generics.md`<br>`code/04_mem/trait_generic`<br>`code/04_mem/hashmap_bucket` | none | link | The md names the code folder. |
| 2.4.9 | 4.8 - Iterators | `ArdanUltimateRust-5Days` | `04-Memory/Iterators.md`<br>`code/04_mem/iterator_generator`<br>`code/04_mem/iterator_hashbucket` | none | link | The md names the code folder. |
| 2.4.10 | 4.9 - Cycles and the Difficulty of Linked Lists | `ArdanUltimateRust-5Days` | `04-Memory/Cycles.md`<br>`code/04_mem/linked_list`<br>`code/04_mem/linked_list_weak` | none | link | The md names the code folder. |
| 2.4.11 | 4.10 - Memory Fragmentation, Allocators and Arenas | `ArdanUltimateRust-5Days` | `04-Memory/MemoryFragmentation.md`<br>`code/04_mem/arena_bump`<br>`code/04_mem/arena_slab` | none | link | The md names the code folder. |
| 2.4.12 | 4.11 - Packing, Reordering & Mangling | `ArdanUltimateRust-5Days` | `04-Memory/Packing.md`<br>`code/04_mem/packing` | none | link | The md names no folder; packing is matched by name. |
| 2.4.13 | 4.12 - From Bytes to Types | `ArdanUltimateRust-5Days` | `04-Memory/BytesToTypes.md`<br>`code/04_mem/save_bytes`<br>`code/04_mem/save_dynamic_bytes` | none | link | The md names the code folder. |
| 2.4.14 | 4.13 - Safely Interacting with Other Languages & Surprise: Memory Leaks are Safe! | `ArdanUltimateRust-5Days` | `04-Memory/FFI.md`<br>`04-Memory/MemoryLeaks.md`<br>`code/04_mem/c_rust`<br>`code/04_mem/rust_c` | none | link | One Ardan lesson covers both pages. c_rust compiles a C source file from build.rs, so it needs a C compiler. |
| 2.5.1 | 5.0 - Introduction & Planning Our Project | `ArdanUltimateRust-5Days` | `05-Server/README.md`<br>`05-Server/Planning.md` | none | link | Project intro and plan. No code. |
| 2.5.2 | 5.1 - Shared Data Structures | `ArdanUltimateRust-5Days` | `05-Server/SharedDataStructures.md`<br>`code/05_server/shared_v1` | none | link | The md names the code folder. |
| 2.5.3 | 5.2 - Collection Daemon Mk 1 | `ArdanUltimateRust-5Days` | `05-Server/CollectionDaemon1.md`<br>`code/05_server/collector_v1` | none | link | The md names the code folder. |
| 2.5.4 | 5.3 - Collection Server Mk 1 | `ArdanUltimateRust-5Days` | `05-Server/CollectionServer1.md`<br>`code/05_server/server_v1` | none | link | The md names the code folder. |
| 2.5.5 | 5.4 - Error Handling in the Collector | `ArdanUltimateRust-5Days` | `05-Server/CollectionDaemon2.md`<br>`code/05_server/collector_v2`<br>`code/05_server/shared_v2` | none | link | The md names the code folder. |
| 2.5.6 | 5.5 - Setting the Collector ID | `ArdanUltimateRust-5Days` | `05-Server/CollectorId.md`<br>`code/05_server/collector_v2` | none | link | Same collector_v2 folder as 2.5.5. |
| 2.5.7 | 5.6 - Web Service Mk 1 | `ArdanUltimateRust-5Days` | `05-Server/WebService1.md`<br>`code/05_server/server_v2`<br>`code/05_server/shared_v2` | none | link | Uses sqlx and a sqlite database file; the server_v2 folder also holds the migration. |
| 2.5.8 | 5.7 - Web Server | `ArdanUltimateRust-5Days` | `05-Server/WebServer1.md`<br>`code/05_server/server_v2` | none | link | The md names no folder; server_v2 holds web.rs, index.html and collector.html, which this page builds (matched by file name). |
| 2.5.9 | 5.8 - Let’s Use Less Bandwidth | `ArdanUltimateRust-5Days` | `05-Server/CollectionDaemon3.md`<br>`code/05_server/shared_v3` | none | link | The md names the code folder. |

## Part 3 · Ultimate Rust: Foundations - Next Steps

Repo `ArdanUltimateRust-5Days`, folder `05-Server` (no licence, link only). Two rows use `Ardan-1HourAsync` instead, see their notes.

| display | title | repo | path(s) in repo | licence | use | note |
|---|---|---|---|---|---|---|
| 3.0.1 | Ultimate Rust Foundations: Next Steps | `ArdanUltimateRust-5Days` | `05-Server/README.md` | none | link | Text page; the 05-Server README describes the capstone server project. No code. |
| 3.1.1 | 1.0 - Bi-Directional Communication | `ArdanUltimateRust-5Days` | `05-Server/BiDirectional.md`<br>`code/05_server/collector_v3`<br>`code/05_server/server_v3`<br>`code/05_server/shared_v3` | none | link | The md names collector_v3 and server_v3; shared_v3 is the library both use. |
| 3.1.2 | 1.1 - Sending Commands & Prevent Unbounded Growth | `ArdanUltimateRust-5Days` | `05-Server/SendingCommands.md`<br>`05-Server/CollectionDaemon4.md`<br>`code/05_server/shared_v3`<br>`code/05_server/server_v3`<br>`code/05_server/collector_v4` | none | link | SendingCommands.md names no folder; its command types (RequestWork and friends) are defined in shared_v3 and handled in server_v3 (found by symbol search). collector_v4 is from the unbounded-growth page. |
| 3.1.3 | 1.2 - Giving the Collector a Diet | `ArdanUltimateRust-5Days` | `05-Server/SmallerCollector.md`<br>`code/05_server/collector_nouuid` | none | link | Release-profile size tricks and cargo bloat; the md names collector_nouuid. |
| 3.1.4 | 1.3 - Optimizing File Reading | `Ardan-1HourAsync` | `README.md`<br>`tokio_stream_file` | none | link | No page of this name in 05-Server. Best guess by topic (UNVERIFIED): the webinar repo's demo that streams a file line by line. Check against the video. |
| 3.1.5 | 1.4 - Adapting Files In Flight | `Ardan-1HourAsync` | `README.md`<br>`stream_adapt` | none | link | No page of this name in 05-Server. Best guess (UNVERIFIED): the webinar repo's stream_adapt demo, whose README says it adapts a stream in flight; its code wraps a file's line stream in an adapter. Check against the video. |

## Part 4 · Ultimate Rust Best Practices

No public exercise repo found for Best Practices (searches listed above), so every row is `own`.

| display | title | repo | path(s) in repo | licence | use | note |
|---|---|---|---|---|---|---|
| 4.0.1 | Ultimate Rust Best Practices | - | - | - | own | No public repo (searched, see top). Course intro page. |
| 4.0.2 | The Best Practices Intro | - | - | - | own | No public repo (searched, see top). Intro video. |
| 4.1.1 | 1.0 - Introduction - Formatting | - | - | - | own | No public repo (searched, see top). Overlap to avoid: ultimate_rust2 exercise/idiomatic starts with cargo fmt and is already used in 5.2.2. |
| 4.1.2 | 1.1 - Clippy (the linter) | - | - | - | own | No public repo (searched, see top). Overlap to avoid: ultimate_rust2 exercise/idiomatic step 3 is cargo clippy and is already used in 5.2.2. |
| 4.1.3 | 1.2 - Documentation | - | - | - | own | No public repo (searched, see top). Overlap to avoid: ultimate_rust2 exercise/docs is already used in 5.2.4. |
| 4.1.4 | 1.3 - Understanding Dependencies | - | - | - | own | No public repo (searched, see top). Tooling lesson. |
| 4.1.5 | 1.4 - Managing Your Own Dependencies | - | - | - | own | No public repo (searched, see top). Tooling lesson. |
| 4.1.6 | 1.5 - Checking for Vulnerabilities | - | - | - | own | No public repo (searched, see top). Tooling lesson. |
| 4.1.7 | 1.6 - Check for Outdated Dependencies | - | - | - | own | No public repo (searched, see top). Tooling lesson. |
| 4.1.8 | 1.7 - Denying Dependencies by Licensing | - | - | - | own | No public repo (searched, see top). Tooling lesson. |
| 4.2.1 | 2.0 - Favor Iterators | - | - | - | own | No public repo (searched, see top). Overlap to avoid: ultimate_rust2 exercise/closures_iterators is already used in 5.2.9. |
| 4.2.2 | 2.1 - Minimize Cloning (we have an alternative name as well Cloning Can Be a Code Smell) | - | - | - | own | No public repo (searched, see top). |
| 4.2.3 | 2.2 - Don’t Emulate OOP | - | - | - | own | No public repo (searched, see top). |
| 4.2.4 | 2.3 - Favor Small Functions | - | - | - | own | No public repo (searched, see top). |
| 4.2.5 | 2.4 - Clever Code | - | - | - | own | No public repo (searched, see top). |
| 4.2.6 | 2.5 - Floating Point Numbers | - | - | - | own | No public repo (searched, see top). |
| 4.2.7 | 2.6 - Platform & Feature Specific Code | - | - | - | own | No public repo (searched, see top). |
| 4.3.1 | 3.0 - TANSTAAFL (There Ain’t No Such Thing As A Free Lunch) | - | - | - | own | No public repo (searched, see top). |
| 4.3.2 | 3.1 - YAGNI : You Ain’t Gonna Need It | - | - | - | own | No public repo (searched, see top). |
| 4.3.3 | 3.2 - Domain Boundaries | - | - | - | own | No public repo (searched, see top). |
| 4.3.4 | 3.3 - Taming Compile Times | - | - | - | own | No public repo (searched, see top). |
| 4.4.1 | The Best Practices Outro | - | - | - | own | No public repo (searched, see top). Outro video. |

## Part 5 · Ultimate Rust 2: Intermediate Concepts

Repos `ultimate_rust2` (MIT) for lessons 5.1 and 5.2, `rusty_engine` (MIT OR Apache-2.0) for modules 3-5.

| display | title | repo | path(s) in repo | licence | use | note |
|---|---|---|---|---|---|---|
| 5.0.1 | Ultimate Rust 2: Intermediate Concepts | `ultimate_rust2` | `README.md` | MIT | own | Course intro page. README only explains how to use the exercises and lists the 8 folders; text only. |
| 5.1.1 | Lesson 1.1 - Introduction | - | - | - | own | Intro video; no repo material. |
| 5.1.2 | Lesson 1.2 - Exercises Overview | `ultimate_rust2` | `README.md` | MIT | own | README section Exercises (same workflow as Part 1) with the list of the exercise folders. Text only. |
| 5.2.1 | Lesson 2.1 - Idiomatic Code | - | - | - | own | Concept lesson; the exercise follows in 5.2.2. No lecture example identified in the repo. |
| 5.2.2 | Lesson 2.2 - Exercise - Idiomatic Code | `ultimate_rust2` | `exercise/idiomatic`<br>`exercise/idiomatic/src/main.rs` | MIT | embed | Deliberately ugly code: run cargo fmt, add blank lines, make cargo clippy silent, then a free-form challenge. |
| 5.2.3 | Lesson 2.3 - Documentation | `ultimate_rust2` | `example/puzzles/src/lib.rs`<br>`example/hello/src/lib.rs` | MIT | embed | Lecture code: doc comments with a heading and intra-doc links on the Puzzle items (puzzles), a doc-test (hello). Matched by content, check against the video. |
| 5.2.4 | Lesson 2.4 - Exercise - Documentation | `ultimate_rust2` | `exercise/docs`<br>`exercise/docs/src/lib.rs` | MIT | embed | Document a small Pumpkin library (module docs, an image, struct, method, constant) and find the cargo doc option for private items. |
| 5.2.5 | Lesson 2.5 - Publishing | - | - | - | own | Concept lesson. No publishing material anywhere in the repo. |
| 5.2.6 | Lesson 2.6 - Exercise - Publishing | - | - | - | own | The repo has 8 exercise folders and none about publishing (the README lists 8; a search for publish finds only the licence text). Write an own exercise, e.g. cargo package or cargo publish --dry-run on a small crate. |
| 5.2.7 | Lesson 2.7 - Iterators | - | - | - | own | Concept lesson; the iterators exercise is in 5.2.9. |
| 5.2.8 | Lesson 2.8 - Closures | - | - | - | own | Concept lesson; the closures exercise is in 5.2.9. |
| 5.2.9 | Lesson 2.9 - Exercise - Closures & Iterators | `ultimate_rust2` | `exercise/closures_iterators`<br>`exercise/closures_iterators/src/main.rs` | MIT | embed | Four steps plus a challenge: a squaring closure, a map over tuples, an iterator over a mutable vector, a vector of words. |
| 5.2.10 | Lesson 2.10 - Common Traits | `ultimate_rust2` | `example/puzzles/src/lib.rs` | MIT | embed | Lecture code: Puzzle implements Default, PartialEq, From / Into and derives Clone and Debug. Matched by content. |
| 5.2.11 | Lesson 2.11 - Exercise - Traits | `ultimate_rust2` | `exercise/traits`<br>`exercise/traits/src/main.rs` | MIT | embed | Five steps plus two challenges on common traits for a Cake enum and a Party struct: Debug, fixing a moved value, struct update syntax, PartialEq, then conversions with From. |
| 5.2.12 | Lesson 2.12 - Creating Errors | `ultimate_rust2` | `example/puzzles/src/lib.rs` | MIT | embed | Lecture code: PuzzleError built with thiserror; comments number the guidelines (enum, grouped, only own errors, non_exhaustive, Debug + Display + Error). |
| 5.2.13 | Lesson 2.13 - Handling Errors | `ultimate_rust2` | `example/puzzle_game/src/main.rs`<br>`example/puzzles/src/lib.rs` | MIT | embed | Lecture code: anyhow Result and Context in puzzle_game; commented question-mark versus match chains in puzzles. |
| 5.2.14 | Lesson 2.14 - Exercise - Errors | `ultimate_rust2` | `exercise/errors`<br>`exercise/errors/src/lib.rs`<br>`exercise/errors/src/main.rs` | MIT | embed | Start in lib.rs: define the error enum (thiserror); then main.rs uses anyhow and the question-mark operator. |
| 5.2.15 | Lesson 2.15 - Unit Tests | `ultimate_rust2` | `example/hello/src/lib.rs` | MIT | embed | Lecture code: a cfg(test) module with a normal test, a should_panic test and a Result-returning test. |
| 5.2.16 | Lesson 2.16 - Integration Tests | `ultimate_rust2` | `example/hello/tests/anything.rs` | MIT | embed | Lecture code: an integration test calling the library from outside. |
| 5.2.17 | Lesson 2.17 - Benchmarks | `ultimate_rust2` | `example/hello/benches/snuggle_speed.rs`<br>`example/hello/Cargo.toml` | MIT | embed | Lecture code: a criterion benchmark; Cargo.toml shows the dev-dependency and harness = false. |
| 5.2.18 | Lesson 2.18 - Exercise - Testing | `ultimate_rust2` | `exercise/testing`<br>`exercise/testing/src/lib.rs` | MIT | embed | Library crate, five steps plus a benchmark challenge: a cfg(test) module with unit tests for sploosh and splish, then an integration test in a new tests/more_tests.rs. Run with cargo test. |
| 5.2.19 | Lesson 2.19 - Logging | `ultimate_rust2` | `example/kitchen/src/main.rs`<br>`example/kitchen/Cargo.toml` | MIT | embed | Lecture code: log macros with env_logger, controlled by RUST_LOG. The same crate serves 5.2.21. |
| 5.2.20 | Lesson 2.20 - Exercise - Logging | `ultimate_rust2` | `exercise/logging`<br>`exercise/logging/src/lib.rs`<br>`exercise/logging/src/main.rs` | MIT | embed | Add log and env_logger, put log macros into a Frog library, run with different RUST_LOG levels. |
| 5.2.21 | Lesson 2.21 - Multithreading | `ultimate_rust2` | `example/kitchen/src/main.rs` | MIT | embed | Lecture code: thread::spawn and join in the kitchen crate (same crate as 5.2.19). |
| 5.2.22 | Lesson 2.22 - Channels | `ultimate_rust2` | `example/cafeteria/src/main.rs` | MIT | embed | Lecture code: crossbeam channels with two worker threads. |
| 5.2.23 | Lesson 2.23 - Exercise - Threads & Channels | `ultimate_rust2` | `exercise/threads_channels`<br>`exercise/threads_channels/src/main.rs` | MIT | embed | Five steps plus a challenge: spawn a child thread and collect its result, send data over a channel between threads, join the threads (needs the crossbeam crate). |
| 5.3.1 | Lesson 3.1 - Project Overview | `rusty_engine` | `README.md`<br>`tutorial/src/00-welcome.md`<br>`tutorial/src/02-quick-start.md` | MIT OR Apache-2.0 | own | Overview and quick start docs; text only. The clone is rusty_engine 7.0.0 (Bevy 0.18), newer than the video: check the API before reusing any code. |
| 5.3.2 | Lesson 3.2 - Configuration | `rusty_engine` | `tutorial/src/05-config.md`<br>`tutorial/src/10-assets.md` | MIT OR Apache-2.0 | own | Configuration and the asset pack download; text only. |
| 5.3.3 | Lesson 3.3 - Engine Initialization | `rusty_engine` | `tutorial/src/15-init.md`<br>`examples/sprite.rs` | MIT OR Apache-2.0 | embed | Prelude, GameState with derive Resource, Game::new and run. examples/sprite.rs is the smallest complete program. |
| 5.3.4 | Lesson 3.4 - Game State | `rusty_engine` | `tutorial/src/20-game-state.md`<br>`examples/game_state.rs` | MIT OR Apache-2.0 | embed | GameState passed to run; the example keeps a timer and a turning flag in it. |
| 5.3.5 | Lesson 3.5 - Game Logic Function | `rusty_engine` | `tutorial/src/25-game-logic-function.md`<br>`examples/game_state.rs` | MIT OR Apache-2.0 | embed | Logic function taking the engine and the game state; same example as 5.3.4. |
| 5.3.6 | Lesson 3.6 - Sprites | `rusty_engine` | `tutorial/src/50-sprite.md`<br>`tutorial/src/55-sprite-creation.md`<br>`tutorial/src/60-sprite-placement.md`<br>`examples/sprite.rs`<br>`examples/placement.rs`<br>`examples/layer.rs` | MIT OR Apache-2.0 | embed | Creating, placing, scaling, rotating and layering sprites. |
| 5.3.7 | Lesson 3.7 - Colliders | `rusty_engine` | `tutorial/src/65-sprite-collider.md`<br>`examples/collider.rs`<br>`examples/collision.rs` | MIT OR Apache-2.0 | embed | collider.rs is a helper tool that takes an image path (its header comment gives the command); collision.rs shows collision events. |
| 5.3.8 | Lesson 3.8 - Keyboard Input | `rusty_engine` | `tutorial/src/100-input.md`<br>`tutorial/src/105-keyboard-state.md`<br>`tutorial/src/110-keyboard-events.md`<br>`examples/keyboard_state.rs`<br>`examples/keyboard_events.rs` | MIT OR Apache-2.0 | embed | Keyboard state (is a key held) versus keyboard events. |
| 5.3.9 | Lesson 3.9 - Mouse Input | `rusty_engine` | `tutorial/src/115-mouse-state.md`<br>`tutorial/src/120-mouse-events.md`<br>`examples/mouse_state.rs`<br>`examples/mouse_events.rs` | MIT OR Apache-2.0 | embed | Mouse state (location, buttons) versus mouse events. |
| 5.3.10 | Lesson 3.10 - Text | `rusty_engine` | `tutorial/src/150-text.md`<br>`tutorial/src/155-text-creation.md`<br>`tutorial/src/160-text-attributes.md`<br>`tutorial/src/165-text-placement.md`<br>`examples/text.rs` | MIT OR Apache-2.0 | embed | Creating text, its value, font and size, and placing it. |
| 5.3.11 | Lesson 3.11 - Audio | `rusty_engine` | `tutorial/src/200-audio.md`<br>`tutorial/src/205-music.md`<br>`tutorial/src/210-sfx.md`<br>`examples/music.rs`<br>`examples/music_sampler.rs`<br>`examples/sfx.rs`<br>`examples/sfx_sampler.rs`<br>`examples/sound.rs` | MIT OR Apache-2.0 | embed | Music and sound-effect presets plus playing a file by path. Four bundled music tracks carry attribution rules (see Licences). |
| 5.3.12 | Lesson 3.12 - Timer | `rusty_engine` | `tutorial/src/250-timer.md`<br>`examples/game_state.rs`<br>`examples/text.rs` | MIT OR Apache-2.0 | embed | Found by searching for Timer: game_state.rs and text.rs tick a Timer each frame; sfx_sampler.rs (see 5.3.11) also uses timers. |
| 5.3.13 | Lesson 3.13 - Engine & Game Structs | `rusty_engine` | `tutorial/src/400-engine.md`<br>`tutorial/src/450-game.md`<br>`examples/window.rs` | MIT OR Apache-2.0 | embed | Engine fields and Game methods (window settings, adding logic, running); window.rs shows custom window settings. |
| 5.4.1 | Lesson 4.1 - Common Setup | `rusty_engine` | `scenarios/README.md` | MIT OR Apache-2.0 | embed | Common Setup section: Cargo.toml and asset pack steps plus the skeleton main.rs that every scenario starts from. |
| 5.4.2 | Lesson 4.2 - Road Race | `rusty_engine` | `scenarios/road_race.md`<br>`examples/scenarios/road_race.rs` | MIT OR Apache-2.0 | embed | Step-by-step scenario (Easy). road_race.rs is the finished reference: a spoiler, show it only after the learner has tried. |
| 5.5.1 | Lesson 5.1 - Car Shoot pt. 1 | `rusty_engine` | `scenarios/car_shoot.md`<br>`examples/scenarios/car_shoot.rs` | MIT OR Apache-2.0 | embed | Medium scenario. The doc has Engine Initialization, Game Setup, Game Logic and Challenges; how the four videos split it is not in the repo. Guess: pt.1 = Engine Initialization + Game Setup. car_shoot.rs is the finished reference (spoiler). |
| 5.5.2 | Lesson 5.2 - Car Shoot pt. 2 | `rusty_engine` | `scenarios/car_shoot.md`<br>`examples/scenarios/car_shoot.rs` | MIT OR Apache-2.0 | embed | Guess: pt.2 = first Game Logic items (gun follows the mouse, fire marbles, move and clean up marbles). Same files as 5.5.1. |
| 5.5.3 | Lesson 5.3 - Car Shoot pt. 3 | `rusty_engine` | `scenarios/car_shoot.md`<br>`examples/scenarios/car_shoot.rs` | MIT OR Apache-2.0 | embed | Guess: pt.3 = spawn cars with a random timer, move the cars. Same files as 5.5.1. |
| 5.5.4 | Lesson 5.4 - Car Shoot pt. 4 | `rusty_engine` | `scenarios/car_shoot.md`<br>`examples/scenarios/car_shoot.rs` | MIT OR Apache-2.0 | embed | Guess: pt.4 = collisions and sound effects, then the Challenges list for extra work. Same files as 5.5.1. |

## Part 6 · Rust as a Service

Repo `ArdanRustService` (no licence, link only). Pages are in `manual/src/`, code in `code/` and `no_workspace/`.

| display | title | repo | path(s) in repo | licence | use | note |
|---|---|---|---|---|---|---|
| 6.0.1 | Rust as a Service | `ArdanRustService` | `manual/src/SUMMARY.md` | none | link | Text page; the manual's table of contents mirrors the course outline. No code. |
| 6.1.1 | 1. Introduction | `ArdanRustService` | `manual/src/intro.md` | none | link | Intro page (overview, trainer, resources). No code. |
| 6.2.1 | 2.0 REST Service | `ArdanRustService` | `manual/src/rest_service.md` | none | link | Section intro. No code. |
| 6.2.2 | 2.1 Minimal HTTP Server | `ArdanRustService` | `manual/src/simple_http_server.md`<br>`code/rest_service/simple_http_server` | none | link | The md names the code folder. |
| 6.2.3 | 2.2 Understanding the Service Stack | `ArdanRustService` | `manual/src/axum_hyper_tower_tokio.md` | none | link | Concept page on how Axum, Hyper, Tower and Tokio stack. No code. |
| 6.2.4 | 2.3 Extractors | `ArdanRustService` | `manual/src/axum_extractors.md` | none | link | Section intro for extractors. No code. |
| 6.2.5 | 2.3.1 Path Extraction | `ArdanRustService` | `manual/src/axum_extractors_path.md`<br>`code/rest_service/axum_extract_path` | none | link | The md's code pointer has a typo; the real folder is axum_extract_path. |
| 6.2.6 | 2.3.2 Query Extraction | `ArdanRustService` | `manual/src/axum_extractors_query.md`<br>`code/rest_service/axum_extract_query` | none | link | The md names the code folder. |
| 6.2.7 | 2.3.3 Header Extraction & 2.3.4 More Extractors | `ArdanRustService` | `manual/src/axum_extractors_headers.md`<br>`manual/src/axum_extractors_more.md`<br>`code/rest_service/axum_extract_headers` | none | link | One Ardan lesson covers header extraction and the More Extractors page; only the headers page has a code folder (named in the code workspace Cargo.toml, not in the md). |
| 6.2.8 | 2.4 Add a Simple Tower Layer: State | `ArdanRustService` | `manual/src/simple_tower_layer.md`<br>`code/rest_service/simple_tower_server` | none | link | The md names the code folder. |
| 6.2.9 | 2.5 Add a Simple Tower Layer (Mutable State) | `ArdanRustService` | `manual/src/simple_tower_layer_mut.md`<br>`code/rest_service/simple_tower_server_mut` | none | link | The md names the code folder. |
| 6.2.10 | 2.6 Multiple States | `ArdanRustService` | `manual/src/simple_tower_layer_multi_state.md`<br>`code/rest_service/simple_tower_server_multi_state` | none | link | The md names the code folder. |
| 6.2.11 | 2.7 Quick Recap on State and Layers | `ArdanRustService` | `manual/src/simple_tower_recap.md` | none | link | Recap page. No code. |
| 6.2.12 | 2.8 Nesting Multiple Routers | `ArdanRustService` | `manual/src/nesting.md`<br>`code/rest_service/simple_nested` | none | link | The md names the code folder. |
| 6.2.13 | 2.9 Nested Routers with State | `ArdanRustService` | `manual/src/nesting_state.md`<br>`code/rest_service/simple_nested_state` | none | link | The md names the code folder. |
| 6.2.14 | 2.10 Calling Other Services | `ArdanRustService` | `manual/src/calling_other_services.md`<br>`code/rest_service/hyper_client` | none | link | The md's code pointer has a typo; the real folder is hyper_client. |
| 6.2.15 | 2.11 Returning Status Codes | `ArdanRustService` | `manual/src/status_codes.md`<br>`code/rest_service/status_codes` | none | link | The md names the code folder. |
| 6.2.16 | 2.12 Using IntoResponse and 2.13 Error Handling with IntoResponse | `ArdanRustService` | `manual/src/into_response.md`<br>`manual/src/error_handling.md`<br>`code/rest_service/axum_error_handling` | none | link | One Ardan lesson covers two pages; the md's code pointer has a typo, the real folder is axum_error_handling. |
| 6.2.17 | 2.14 Quick Recap on Nesting, Making Calls and Responses | `ArdanRustService` | `manual/src/error_handling_recap.md` | none | link | Recap page. No code. |
| 6.2.18 | 2.15 Serving Static Content with Tower | `ArdanRustService` | `manual/src/static_content.md`<br>`code/rest_service/static_content` | none | link | The folder includes a web directory with demo.html. |
| 6.2.19 | 2.16 Simple Header-Based Authentication | `ArdanRustService` | `manual/src/header_auth1.md`<br>`code/rest_service/axum_headers` | none | link | The md names the code folder. |
| 6.2.20 | 2.17 Simple Header-Based Auth with Middleware | `ArdanRustService` | `manual/src/header_auth2.md`<br>`code/rest_service/axum_header_layer` | none | link | The md names the code folder. |
| 6.2.21 | 2.18 Middleware Auth with Injection | `ArdanRustService` | `manual/src/header_auth3.md`<br>`code/rest_service/axum_header_layer_inject` | none | link | The md itself points at axum_header_layer; the workspace list has the separate axum_header_layer_inject folder for this page. |
| 6.2.22 | 2.19 Selectively Applying Layers | `ArdanRustService` | `manual/src/layer_targets.md` | none | link | The md names a code folder (axum_merge) that is not in the repo, so no runnable code. Two stub pages (layer_router_merge.md, layer_selective_router.md) are not in the manual's SUMMARY. Nearest demo of route_layer: axum_header_layer (see 6.2.20). |
| 6.2.23 | 2.20 Router Layers | `ArdanRustService` | `manual/src/layer_router.md`<br>`code/rest_service/axum_compression`<br>`code/rest_service/axum_service_builder` | none | link | The md names the code folder. |
| 6.2.24 | 2.21 Layer Recap | `ArdanRustService` | `manual/src/layer_recap.md` | none | link | Recap page. No code. |
| 6.3.1 | 3.0 Tracing | `ArdanRustService` | `manual/src/axum_tracing.md` | none | link | Section intro. No code. |
| 6.3.2 | 3.1 Minimal Example | `ArdanRustService` | `manual/src/axum_tracing_minimal.md`<br>`code/tracing/axum_tracing_minimal` | none | link | The md names the code folder. |
| 6.3.3 | 3.2 Logging Axum/Tower | `ArdanRustService` | `manual/src/axum_tracing_tower.md`<br>`code/tracing/axum_tracing_tower` | none | link | The md names the code folder. |
| 6.3.4 | 3.3 Timing Spans | `ArdanRustService` | `manual/src/axum_spans_own.md`<br>`code/tracing/axum_spans_own` | none | link | The md names the code folder. |
| 6.3.5 | 3.4 Axum Spans | `ArdanRustService` | `manual/src/axum_spans.md`<br>`code/tracing/axum_spans_own` | none | link | The workspace list says axum_spans_own serves both this page and 6.3.4. |
| 6.3.6 | 3.5 Logging to a File | `ArdanRustService` | `manual/src/axum_file_log.md`<br>`code/tracing/trace_to_file` | none | link | The md names the code folder. |
| 6.3.7 | 3.6 Structured Logging to JSON | `ArdanRustService` | `manual/src/axum_log_json.md`<br>`code/tracing/trace_to_json` | none | link | The md names the code folder. |
| 6.3.8 | 3.7 OpenTelemetry & 3.7.1 Hello Telemetry | `ArdanRustService` | `manual/src/otel_top.md`<br>`manual/src/otel_hello.md`<br>`code/tracing/otel_minimal` | none | link | One Ardan lesson covers the OpenTelemetry intro page and Hello Telemetry. |
| 6.4.1 | 4.0 OpenAPI Documentation | `ArdanRustService` | `manual/src/openapi.md`<br>`code/openapi/axum_openapi` | none | link | The md names the code folder. |
| 6.5.1 | 5.0 Handling Service Configuration | `ArdanRustService` | `manual/src/config_top.md` | none | link | Section intro. No code. |
| 6.5.2 | 5.1 Environment Variables with .env | `ArdanRustService` | `manual/src/config_dot_env.md`<br>`code/config/envfile` | none | link | The md names the code folder. |
| 6.5.3 | 5.2 The Config Crate | `ArdanRustService` | `manual/src/config_crate.md`<br>`code/config/config_minimal` | none | link | The md names the code folder. |
| 6.5.4 | 5.3 Loading Config via HTTP | `ArdanRustService` | `manual/src/config_http.md`<br>`code/config/config_http` | none | link | The md names the code folder. |
| 6.5.5 | 5.4 CLI configuration with Clap | `ArdanRustService` | `manual/src/config_clap.md`<br>`manual/src/config_recap1.md`<br>`code/config/config_clap` | none | link | config_recap1.md (section recap) has no Ardan lesson of its own; attached here as the last page of the section. |
| 6.6.1 | 6.0 gRPC | `ArdanRustService` | `manual/src/grpc_intro.md` | none | link | Section intro. No code. |
| 6.6.2 | 6.1 Hello Tonic | `ArdanRustService` | `manual/src/grpc_hello.md`<br>`code/grpc/hello_tonic/proto/hello.proto` | none | link | The page is the protocol definition. gRPC builds need the protoc compiler (see How to run). |
| 6.6.3 | 6.2 Hello Tonic - Project Definition and Build | `ArdanRustService` | `manual/src/grpc_hello2.md`<br>`code/grpc/hello_tonic/Cargo.toml`<br>`code/grpc/hello_tonic/build.rs` | none | link | Files matched by the page title (project definition and build); they belong to the hello_tonic project. |
| 6.6.4 | 6.3 Hello Tonic - The Server | `ArdanRustService` | `manual/src/grpc_hello3.md`<br>`code/grpc/hello_tonic/src/server.rs` | none | link | File matched by the page title (the server); it is a binary of hello_tonic. |
| 6.6.5 | 6.4 Hello Tonic - The Client | `ArdanRustService` | `manual/src/grpc_hello4.md`<br>`code/grpc/hello_tonic/src/client.rs` | none | link | File matched by the page title (the client); it is a binary of hello_tonic. |
| 6.6.6 | 6.5 gRPC Streaming and 6.6 Protocol Definition | `ArdanRustService` | `manual/src/grpc_stream.md`<br>`manual/src/grpc_stream2.md`<br>`code/grpc/tonic_stream/proto/streaming.proto` | none | link | One Ardan lesson covers the streaming intro and its protocol definition. |
| 6.6.7 | 6.7 gRPC Streaming - The Server | `ArdanRustService` | `manual/src/grpc_stream3.md`<br>`code/grpc/tonic_stream/src/server.rs` | none | link | File matched by the page title (the server); it is a binary of tonic_stream. |
| 6.6.8 | 6.8 gRPC Streaming - The Client | `ArdanRustService` | `manual/src/grpc_stream4.md`<br>`code/grpc/tonic_stream/src/client.rs` | none | link | File matched by the page title (the client); it is a binary of tonic_stream. |
| 6.6.9 | 6.9 Recap So Far | `ArdanRustService` | `manual/src/grpc_recap1.md` | none | link | Recap page. No code. |
| 6.6.10 | 6.10 Authentication | `ArdanRustService` | `manual/src/grpc_auth.md`<br>`code/grpc/tonic_auth` | none | link | The md names the code folder. |
| 6.6.11 | 6.11 Tracing | `ArdanRustService` | `manual/src/grpc_tracing.md`<br>`code/grpc/tonic_tracing` | none | link | The md names the code folder. |
| 6.6.12 | 6.12 When to use gRPC | `ArdanRustService` | `manual/src/grpc_conclude.md` | none | link | Closing page on when gRPC fits. No code. |
| 6.7.1 | 7.0 Web Sockets | `ArdanRustService` | `manual/src/ws_intro.md` | none | link | Section intro. No code. |
| 6.7.2 | 7.1 Minimal Echo Server | `ArdanRustService` | `manual/src/ws_echo_server.md`<br>`code/wss/ws_echo` | none | link | The md's code pointer has a typo (wss_echo); the real folder is ws_echo. |
| 6.7.3 | 7.2 A native WS client | `ArdanRustService` | `manual/src/ws_client.md`<br>`code/wss/ws_client` | none | link | The md names the code folder. |
| 6.7.4 | 7.3 JSON | `ArdanRustService` | `manual/src/ws_json.md`<br>`code/wss/ws_json` | none | link | The md names the code folder. |
| 6.8.1 | 8.0 Service Deployment | `ArdanRustService` | `manual/src/svc_deploy_intro.md` | none | link | Section intro. No code. |
| 6.8.2 | 8.1 Build a Test Service & 8.2 Native Host Deployment | `ArdanRustService` | `manual/src/svc_deploy_test.md`<br>`manual/src/svc_deploy_native.md`<br>`no_workspace/deploy_bookstore` | none | link | deploy_bookstore sits outside the code workspace on purpose (no_workspace). One Ardan lesson covers two pages. |
| 6.8.3 | 8.3 Docker Deployment | `ArdanRustService` | `manual/src/svc_deploy_docker.md`<br>`no_workspace/deploy_bookstore/Dockerfile`<br>`no_workspace/deploy_bookstore/compose.yaml` | none | link | Needs Docker Desktop. |
| 6.9.1 | 9.0 Service Design | `ArdanRustService` | `manual/src/design_intro.md` | none | link | Section intro. No code. |
| 6.9.2 | 9.1 Understanding Your Company Architecture | `ArdanRustService` | `manual/src/design_company_arch.md` | none | link | Concept page. No code. |
| 6.9.3 | 9.2 Designing Individual Services (9.2.1, 9.2.2, 9.2.3, 9.2.4, 9.2.5) | `ArdanRustService` | `manual/src/design_svc_intro.md`<br>`manual/src/design_svc_layout.md`<br>`manual/src/design_svc_config.md`<br>`manual/src/design_svc_db.md`<br>`manual/src/design_svc_layer.md`<br>`manual/src/design_svc_module.md`<br>`no_workspace/deploy_bookstore/src/auth`<br>`no_workspace/deploy_bookstore/src/bookstore` | none | link | One Ardan lesson covers the five sub-pages. Their code is the auth and bookstore modules of deploy_bookstore (layout and module: mod.rs and web_service.rs, config: configuration.rs, database: db.rs and migrations, shared layers: auth_layers.rs). |
| 6.9.4 | 9.3 Combining Services into a Modular Monolith | `ArdanRustService` | `manual/src/design_svc_modular_monolith.md`<br>`no_workspace/deploy_bookstore/src/main.rs`<br>`no_workspace/deploy_bookstore/src/service_config.rs` | none | link | The md walks through main.rs and the service_config module (service_config.rs); it names deploy_bookstore but no file path. |
| 6.9.5 | 9.4 Service Exposure | `ArdanRustService` | `manual/src/design_svc_exposure.md` | none | link | Concept page. No code. |
| 6.9.6 | 9.5 Scaling Out | `ArdanRustService` | `manual/src/design_svc_scaling_out.md` | none | link | Concept page. No code. |
| 6.10.1 | 10.0 Wrap Up | `ArdanRustService` | `manual/src/wrap.md` | none | link | Closing page. No code. |

## Part 7 · Rust Webinars

Companion repos of three webinars (no licence, link only). Two webinars have no repo.

| display | title | repo | path(s) in repo | licence | use | note |
|---|---|---|---|---|---|---|
| 7.1.1 | Introducing Rust into your Company Ecosystem | `ArdanLabs_RustInYourEnterprise` | `README.md`<br>`SlideDeck.pdf`<br>`helloworld_webservice`<br>`hellojson_webservice`<br>`hellodb_webservice`<br>`hellodb_timed_client`<br>`timed_json_serialize`<br>`tcp_server`<br>`tcp_server_client`<br>`mypylib`<br>`rustgo`<br>`crust` | none | link | Section: Introducing Rust into your Company Ecosystem. README groups the demos: web services (plain, JSON, sqlite), timing clients, TCP, Python (mypylib), Go (rustgo), legacy C (crust). SlideDeck.pdf is Herbert's slides: link only. mypylib needs Python, rustgo needs Go, crust needs a C compiler. |
| 7.2.1 | Webinar with Herbert (Aug 27, 2024) | - | - | - | own | Section: Fearless Concurrency with Rust. No companion repo known. The thread material of Part 2 (2.2.x, 02-SystemThreads) covers the same topic. |
| 7.3.1 | Webinar with Herbert (Aug 2nd, 2023) | `Ardan-1HourAsync` | `README.md`<br>`1-Hour-Async-Deep-Dive.pdf`<br>`futures_block_on`<br>`tokio_block_on`<br>`hello_tokio`<br>`tokio_join`<br>`tokio_spawn`<br>`tokio_yield`<br>`tokio_select`<br>`tokio_blocking`<br>`tokio_spawn_blocking`<br>`tokio_channels`<br>`tokio_multi_channels`<br>`thread_to_tokio`<br>`tokio_stream_generator`<br>`tokio_stream_file`<br>`tracing_log`<br>`tracing_json`<br>`stream_adapt`<br>`tokio_streams`<br>`tracing_spans` | none | link | Section: 1 Hour Dive into Asynchronous Rust. Folders are in README order (17 demos), then tokio_streams and tracing_spans (workspace members the README does not list). The PDF is Herbert's slides: link only. |
| 7.4.1 | Rust Foundations: Herbert Wolverson | - | - | - | own | Section: Rust Foundations Highlights Webinar. No companion repo known. |
| 7.5.1 | Webinar with Herbert Wolverson (January, 2023) | `ArdanRustDebugOptimize` | `Outline.md`<br>`Day1.md`<br>`Day2.md`<br>`Day3.md`<br>`Day4.md`<br>`Day5.md`<br>`src` | none | link | Section: Debugging & Optimizing Rust. The repo is the in-development five-day class: README.md is a placeholder, Outline.md is the agenda, src is a workspace of small demo crates. The one-hour webinar covers only part of it, so pick folders by the video. Day1 debugging, Day2 avoiding bugs, Day3 to Day5 are short outlines. |

## Licences

What each repo actually carries, and the file it was read from. Checked in the local clones on 2026-10-09 and against the GitHub API (`license` field).

| repo | licence | read from | consequence |
|---|---|---|---|
| `ultimate_rust_crash_course` | MIT, copyright 2018 Nathan Stocks | `LICENSE`; GitHub API says MIT | `embed` with the copyright and licence notice |
| `ultimate_rust2` | MIT, copyright 2018 Nathan Stocks | `LICENSE`; GitHub API says MIT. The README claims MIT plus Apache-2.0 and points to `license/APACHE` and `license/MIT`, but that folder is NOT in the clone; only the MIT text is confirmed | `embed` under MIT |
| `rusty_engine` software | MIT OR Apache-2.0 | `license/MIT`, `license/APACHE`, `Cargo.toml` (`license = "MIT OR Apache-2.0"`), README section Software License. GitHub API reports no licence only because the files are not named LICENSE | `embed` under MIT; keep the copyright line (Nathan Stocks, 2020) |
| `rusty_engine` assets | mostly CC0 1.0 (Kenney packs); four music tracks restricted | `assets/audio/sfx/README.md`, `assets/sprite/racing/README.md`, `assets/sprite/rolling/README.md` (CC0); `assets/audio/music/README.md`: the tracks Arcade Fantasy, Classy 8-Bit, Mysterious Magic and Whimsical Popsicle need attribution (soundimage.org rules) when distributed | do not copy assets into the library. The Car Shoot doc suggests `MusicPreset::Classy8Bit`, one of the restricted tracks |
| `ArdanUltimateRust-5Days` | none | no LICENSE, COPYING or NOTICE file at any depth (`git ls-files`); no `license` key in any Cargo.toml; GitHub API says none | all rights reserved by default: `link` only, never copy code or prose |
| `ArdanRustService` | none | same checks, same result | `link` only |
| `Ardan-1HourAsync` | none | same checks, same result (the repo also holds Herbert's slides as a PDF) | `link` only |
| `ArdanLabs_RustInYourEnterprise` | none | same checks, same result (slides PDF as well) | `link` only |
| `ArdanRustDebugOptimize` | none | same checks, same result | `link` only |

Version note: the clones are the current default branch (HEAD hashes are in the task artifact). `rusty_engine` is at 7.0.0 (Bevy 0.18, edition 2024); the course videos were recorded against an older release, so check the API before reusing its code. Herbert's repos pin the crate versions of their time (for example tonic 0.10 in `hello_tonic`); STYLE.md says how to state versions.

## How to run an exercise

Every exercise or demo is an ordinary Cargo project. Open and run ONE project folder at a time. PowerShell, Windows:

```powershell
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust_crash_course\exercise\b_functions
cargo run
```

```powershell
cd D:\RustroverProjects\learn_rust\ardan\ultimate_rust2\exercise\testing
cargo test
```

- **Crash Course** (`ultimate_rust_crash_course`): each folder in `exercise/` is its own project, so open the single folder, not the repo root. Follow the numbered comments in `src/main.rs`. Exercise A has no project: create one outside the clone with `cargo new variables` and follow `exercise\a_variables\README.md`. `g_ownership_references` needs an argument (`cargo run apple`). `z_final_project` downloads the `image` crate on first build, is meant to run with `--release` and takes arguments (usage is in the header comment of its `src/main.rs`).
- **Ultimate Rust 2** (`ultimate_rust2`): `example/*` and `exercise/*` are members of one workspace (root `Cargo.toml`), so running inside a folder works. `docs`, `errors`, `logging` and `testing` contain a library: use the command named in the comments of each file (`cargo doc --no-deps --open`, `cargo build --lib`, `cargo test`). `example\hello` has tests and a criterion benchmark (`cargo test`, `cargo bench`).
- **Rusty Engine** (`rusty_engine`): from the repo root, `cargo run --release --example sprite` (any file name in `examples/`); scenarios are `--example road_race` and `--example car_shoot`. The first build compiles Bevy and takes minutes. Scenario work starts in a NEW project, see `scenarios/README.md` (Common Setup).
- **Herbert's repos** (`ArdanUltimateRust-5Days`, `ArdanRustService`, the three webinar repos): `cd` into the one project folder, for example `...\ArdanUltimateRust-5Days\code\02_threads\mutex`, then `cargo run`. The `code` folder of `ArdanUltimateRust-5Days` is one workspace with over a hundred projects: do not run `cargo run --all` at its root (Herbert's own `Workspaces.md` warns that it tries to run every program) and do not build the whole workspace. Build output goes to a shared `target` folder in the workspace root and gets large.
- **gRPC projects** (`ArdanRustService\code\grpc\*`) have two binaries: `cargo run --bin helloworld-server` in one window and `cargo run --bin helloworld-client` in another (streaming: `streaming-server` / `streaming-client`, auth: `auth-server` / `auth-client`). They use tonic-build, which relies on the protobuf compiler `protoc` (found on PATH or through the PROTOC variable): install it first.
- **Needs extras**: `mypylib` needs Python, `rustgo` needs Go, `crust` and `c_rust` compile C code and need a C compiler, the `deploy_bookstore` Docker steps need Docker Desktop, the database demos use sqlx with a sqlite file.
- The commands above were taken from the repos' own READMEs, code comments and Cargo.toml files. They were not executed when this map was written (the clones were only read, never built).

