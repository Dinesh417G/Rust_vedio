# RustClass

An offline-first desktop digital classroom for Windows. Author a course, then
study it. No accounts, no server, no telemetry, no network calls at runtime.

The behaviour the whole app is built around:

> **Structure exists independently of content.** Plan the entire course first —
> Chapter 4 exists, its video block exists, its heading and "what this video is
> about" description exist — even though the MP4 will not be recorded for
> another two weeks. The page renders, the outline shows it, the block shows a
> clear "not uploaded yet" state, and you fill the file in later.

Two modes, one app. **Author** gives you outline editing, drag-reorder, a block
palette, upload dropzones and an inspector. **Learn** gives you a clean reading
and watching view with progress tracking and notes, and no editing chrome.

---

## Building

### What you need

| | |
|---|---|
| Rust | 1.77 or newer (stable) |
| Node.js | 18 or newer, with npm |
| Windows | 10 or 11, x64. WebView2 ships with Windows 11 and current Windows 10; the installer bootstraps it otherwise. |

Tauri's own prerequisites are listed at
<https://tauri.app/start/prerequisites/>. On Windows that means the MSVC build
tools and the WebView2 runtime.

### Run it

```bash
npm install
npm run tauri dev
```

### Build an installer

```bash
npm run tauri build
```

This produces an NSIS installer under
`src-tauri/target/release/bundle/nsis/`. It installs per-machine and carries
the WebView2 bootstrapper, so a machine without the runtime gets it during
setup.

### Check it

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test                 # unit tests plus integration tests over a real database
npm run build              # type-checks and bundles the frontend
```

The Rust side has no dependency on Tauri, so all of it runs on any platform —
no window, no webview. There is also a Windows cross-check that catches
platform-specific code paths (long paths, in particular) from a Linux box:

```bash
rustup target add x86_64-pc-windows-msvc
cargo clippy -p rustclass-core --target x86_64-pc-windows-msvc --all-targets -- -D warnings
```

---

## Where your data lives

Everything is under one folder:

```
%APPDATA%\in.electronix.rustclass\
├── rustclass.db          the course: chapters, blocks, progress, notes
├── rustclass.db-wal      recent writes, until SQLite checkpoints them
├── media\
│   ├── 2026-09\          imported files, named by asset id
│   └── thumbs\           video thumbnails
└── logs\
    └── rustclass.log.<date>
```

Settings → Storage shows the exact paths on your machine and will open the
folder for you. The Health page (Settings → Health report) lists anything the
course references that is not where it should be.

### Backing up

Two ways, and they are not equivalent:

1. **Copy the whole folder above.** Complete, and the app can be pointed at it
   on another machine — every path in the database is relative to the media
   root, so the folder moves intact.
2. **Settings → Export bundle.** Writes a single `.rcbundle` file: a ZIP
   holding the full course as JSON plus, optionally, the media. This is the
   portable format — hand it to someone else and they import it into their own
   RustClass.

Export *without* media when you only want the structure. The course imports
fine, and every video, image and attachment shows as missing until you relink
it — which is a reasonable way to send someone the shape of a course without
sending twelve gigabytes.

### Restoring

Settings → Import bundle, then choose **Merge** (adds the bundle's chapters
after what is already there, with fresh IDs, touching nothing) or **Replace**
(clears the current chapters and blocks first). Neither one deletes a media
file — only storage cleanup does that, and only after showing you the list.

---

## A few things worth knowing

**Importing a file copies it, by default.** The original is never touched,
moved or deleted. Once imported you can move or delete your copy freely.
Settings → Media lets you switch to *reference* mode, which copies nothing and
records where the file is instead — faster and smaller, but moving the original
breaks the link, and the Health page will tell you so.

**The same file imported twice is stored once.** Files are content-hashed, so
importing `lesson.mp4` and `lesson-final-v2.mp4` with identical contents leaves
one copy on disk and points both blocks at it.

**Deleting is reversible for 30 days.** Chapters and blocks go to
Settings → Trash and can be restored with their content intact. Deleting
permanently removes the rows and still does not touch a single media file.

**Media files are only ever deleted by storage cleanup**, from
Settings → Storage, and only after it has listed exactly what will go and you
have confirmed it. Files you imported by *reference* are never deleted even
then — those belong to you, not to the app.

**Codecs are the webview's, not ours.** The built-in player handles H.264/AAC
in MP4 and VP8/VP9/Opus in WebM, which is what WebView2 supports. MKV, HEVC and
AV1 generally will not play. When one does not, the player says so and offers
to open the file in your default player instead. If you are recording a course
from scratch, H.264 in an MP4 container is the safe choice.

---

## Keyboard

Press `?` anywhere for the full sheet. The ones worth memorising:

| | |
|---|---|
| `Ctrl` `K` | Search the course |
| `Ctrl` `S` | Save everything pending |
| `Ctrl` `E` | Switch between Author and Learn |
| `Ctrl` `B` | Show or hide the outline |
| `Alt` `←` / `→` | Previous / next chapter |

In the video player: `Space` play/pause, `J`/`L` skip ten seconds, `←`/`→` skip
five, `,`/`.` step a frame, `Shift+,`/`Shift+.` change speed, `F` fullscreen,
`M` mute, `0`–`9` jump to that percent.

---

## How it is put together

```
crates/core/     rustclass-core — all storage, filesystem and domain logic,
                 with no dependency on Tauri, so the whole product is
                 testable with `cargo test` on any platform
src-tauri/       the Tauri shell: command handlers, events, app state.
                 Thin by design — parse, lock, call core, emit.
src/             React + TypeScript frontend
```

Two rules hold the design together and are worth stating outright:

**Block state is computed on every read, never stored.** A block is `Empty`,
`Ready`, `Missing` or `Importing`, and that is decided in one function from the
block's payload and the filesystem as it is right now. Storing it would make it
a lie the moment a file is deleted outside the app.

**Every block renderer goes through `BlockShell`, which demands all four
states.** A new block kind cannot be added without deciding what each state
looks like. That is the architecture enforcing the point of the product.

`DECISIONS.md` records every non-obvious choice and why. `PROGRESS.md` tracks
what is done, what is next, and the open risks.
