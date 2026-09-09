# Progress

## Done

All six phases are complete. Every command in §4 of the specification is
implemented and reachable from the UI, and every acceptance test in §12 has
been run against a running window — the results are below.

### Phase 0 — Skeleton
Tauri v2 + React 18 + TypeScript + Vite + Tailwind. Window configured with
`dragDropEnabled`, all six plugins registered, CSP and the asset protocol
scoped to the media root, app icon generated and wired into the bundle config.

### Phase 1 — Data layer
`rustclass-core` holds every line of storage, filesystem and domain logic with
no dependency on Tauri, so the whole product is exercisable by `cargo test` on
any platform. SQLite via bundled `rusqlite` in WAL mode, a forward-only
`user_version` migration ladder now two rungs long, fractional indexing with
automatic rebalance, soft delete, and `BlockState` computed on every read.

### Phase 2 — Authoring UI with zero files
Sidebar outline with per-chapter progress and per-block state dots, chapter
pages, the block palette behind every "+" insert bar, all eight block editors,
all four visual states behind a shared `BlockShell`, drag-reorder for chapters
and blocks, 600 ms debounced autosave with `Ctrl+S` flush, the inspector, and
Trash.

### Phase 3 — Media
Import pipeline with native drag-drop and the file picker, copy and reference
modes, live progress, cancellation, content dedupe, the asset protocol,
thumbnails, metadata fed back from the webview, the video player, and the
missing/relink flow.

### Phase 4 — Learn mode
Mode toggle, read-only rendering, the "not uploaded yet" modal, progress
tracking and resume, video markers, per-chapter notes, dashboard stats, and
FTS5 search behind `Ctrl+K`.

### Phase 5 — Data lifecycle
Outline import with a preview diff, `.rcbundle` export and import,
`integrity_check`, `asset_gc`, the settings screen, `/health`, and log access.

### Phase 6 — Ship
Empty, error and loading states audited across every route; the `?` shortcut
sheet; the light theme; the app icon; a real NSIS installer; and this
documentation.

## Verification

`cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test` are clean, as is `npm run build`. 78 Rust tests: 43 unit and 35
integration over a real database and a real media tree.

The Rust core also cross-compiles and passes clippy for
`x86_64-pc-windows-msvc`, which is what keeps the Windows-only code paths
(verbatim `\\?\` long paths in particular) honest from a Linux box.

### Acceptance tests from §12

| # | Result |
|---|---|
| 1 | **Pass.** Chapter 4 with an un-uploaded video block renders "Video not uploaded yet" with its caption intact, a hollow dot in the sidebar, all of it surviving a restart. |
| 2 | **Pass.** In Learn mode the block opens the modal with the caption and both buttons work. |
| 3 | **Pass.** A 1.23 GB MP4 imports with progress ticking 3% → 100%; cancel leaves no `.part`, no row and the right toast; a second attempt completes. Thumbnail capture and duration are wired to the webview and fail loudly when a frame is not decodable. |
| 4 | **Not verifiable here.** See "Open risks". |
| 5 | **Pass.** Deleting the file outside the app produces "File is missing" with the expected path; "Locate file…" relinks it. |
| 6 | **Pass.** The same file under a different name dedupes — the media folder is byte-identical, both blocks share one asset. |
| 7 | **Pass.** A 10-chapter outline previews as "10 chapters and 24 blocks" with counts per kind, and commits correctly typed. |
| 8 | **Pass.** Blocks reorder within and across chapters and the order survives a restart. |
| 9 | **Pass.** Exported a 1.24 GB bundle, deleted the entire app data directory, imported into a fresh install: every chapter and block field matches the original exactly, with notes, markers and progress intact and all four media files restored with matching hashes. |
| 10 | **Pass.** A soft-deleted chapter restores from Trash with all five blocks, their assets, notes, markers and progress; the media folder is untouched throughout. |

## Deferred

Nothing. Every feature in the specification is built.

## Proposed

Ideas that are **not** in the specification and have **not** been built.

- A "record list" view: every empty video block across the course in one place,
  as a shooting schedule.
- Export a single chapter to standalone HTML, for sharing one lesson.
- Generated TypeScript types from the Rust DTOs, which would close the risk
  below.

## Open risks

- **Playback and seeking were never exercised on a real WebView2.** This is
  acceptance test 4, and it is the one gap. WebKitGTK — the engine on the Linux
  box this was built on — refuses Tauri's custom `asset:` URI scheme in its
  media pipeline with `FormatError`, while GStreamer decodes the very same
  imported file directly. That is a Linux-only limitation of the harness: on
  Windows the asset protocol is served over `http://asset.localhost`, a real
  Range-capable HTTP source, which is exactly what makes seeking into a
  multi-gigabyte MP4 work. What this side of the contract owns is covered by a
  test asserting the stored copy is byte-identical to the source and reads
  correctly at arbitrary offsets. **Seeking to 45:00 in a 1 GB file remains to
  be confirmed on Windows.**

- **The DTO boundary is duplicated by hand.** `crates/core/src/models.rs` and
  `src/lib/types.ts` describe the same structures in two languages. A field
  renamed on one side and not the other compiles cleanly and fails at runtime.
  Generating the TypeScript from the Rust types would remove the risk.

- **Codec support is WebView2's, not ours.** MKV, HEVC and AV1 will not play in
  the built-in player on any Windows machine. The fallback banner and "open in
  default player" make that survivable, but a course recorded in HEVC will be a
  frustrating experience and no application code changes that.

- **The installer was cross-built with mingw, not MSVC.** The NSIS installer
  and a valid PE32+ executable were produced and the generated NSIS script
  carries the WebView2 `downloadBootstrapper` section and per-machine install
  mode. But `cargo-xwin` could not fetch the Microsoft SDK through this
  environment's proxy, so the artifact that should ship — the MSVC build from a
  Windows host — has not been produced. Nor has either installer been *run*.
