# Progress

## Done

### Phase 0 — Skeleton
Tauri v2 + React 18 + TypeScript + Vite + Tailwind. Window configured with
`dragDropEnabled`, all six plugins registered, CSP and the asset protocol
scoped to the media root. App icon generated and wired into the bundle config.

### Phase 1 — Data layer
`rustclass-core` holds every line of storage, filesystem and domain logic with
no Tauri dependency, so the whole product is exercisable by `cargo test` on any
platform. SQLite via bundled `rusqlite` in WAL mode, a forward-only
`user_version` migration ladder, the full schema from the spec, fractional
indexing with automatic rebalance, soft delete, and `BlockState` computed on
every read. All course/chapter/block CRUD commands, plus assets, progress,
markers, notes, search, integrity, GC and bundles.

**Checkpoint: `cargo test` green** — 41 unit tests, 31 integration tests over a
real database and a real media tree. `cargo fmt` and `cargo clippy -- -D
warnings` are clean.

### Phase 2 — Authoring UI with zero files
Sidebar outline with per-chapter progress and per-block state dots, chapter
pages, the block palette behind every "+" insert bar, all eight block editors,
all four visual states behind a shared `BlockShell`, drag-reorder for both
chapters and blocks, 600 ms debounced autosave with `Ctrl+S` flush, the
inspector, and Trash with restore and permanent delete.

**Checkpoint: verified in a running window.** A 10-chapter course with 24
typed blocks was built from a pasted markdown outline without uploading a
single file; every block rendered as a designed placeholder with its heading
and description intact; the whole structure survived a full application
restart. Screenshots taken at each step.

## Next

- Phase 3 — verify the media pipeline against a real multi-gigabyte file:
  import progress, cancel, dedupe, thumbnail capture, and Range-request seeking.
- Phase 4 — verify Learn mode end to end: progress tracking, resume, markers,
  notes and the `Ctrl+K` palette.
- Phase 5 — verify bundle export/import round trip and `asset_gc` in the UI.
- Phase 6 — `npm run tauri build` producing an NSIS installer, and the README.

## Deferred

Nothing is deferred yet. Every command in §4 of the specification is
implemented and reachable from the UI.

## Proposed

Ideas that are **not** in the specification and have **not** been built. Logged
here rather than added.

- A per-chapter "record list" view: every empty video block across the course
  in one place, as a shooting schedule.
- Export a chapter to standalone HTML for sharing a single lesson.
- Marker import from a chapter file so timestamps can be drafted alongside a
  script.

## Open risks

- **The DTO boundary is duplicated by hand.** `crates/core/src/models.rs` and
  `src/lib/types.ts` describe the same structures in two languages. A field
  renamed on one side and not the other compiles cleanly and fails at runtime.
  The integration tests cover the Rust side; the TypeScript side is guarded
  only by `tsc`. Generating the TypeScript from the Rust types would remove the
  risk and is the first thing to do if this bites once.
- **Codec support is WebView2's, not ours.** MKV, HEVC and AV1 will not play in
  the built-in player on any Windows machine. The fallback banner and "open in
  default player" make that survivable, but a course recorded in HEVC will be a
  frustrating experience and no amount of application code changes that.
- **Verification here ran under Xvfb on Linux**, not on Windows 11 with
  WebView2. The Rust layer is platform-independent and fully tested; the
  Windows-specific paths that have *not* been exercised on real hardware are
  the NSIS installer, the WebView2 bootstrapper, `\\?\` long-path handling, and
  Explorer's "reveal in folder".
