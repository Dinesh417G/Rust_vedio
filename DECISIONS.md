# Decisions

Every non-obvious choice, with the reasoning that drove it. Newest at the
bottom of each phase.

## Stack

- **React + TypeScript, not Leptos/Dioxus.** A 100% Rust UI is attractive and
  the argument for it is real: one language, one toolchain, no `serde` boundary
  to keep in sync. It was still the wrong call here. This app leans hard on
  three things — `@dnd-kit` for drag-reorder across a nested tree, Shiki for
  syntax highlighting, and the `remark`/`rehype` pipeline for sanitised
  markdown. In the Rust UI ecosystem all three are either absent, immature, or
  a WASM wrapper around the JavaScript library anyway. Building them from
  scratch would have consumed the entire budget for the features that actually
  matter. The trade accepted: DTOs are declared twice, in `models.rs` and in
  `src/lib/types.ts`, and must be kept in step by hand.

- **`rustclass-core` is a separate crate with no Tauri dependency.** All
  storage, filesystem and domain logic lives there. `src-tauri` is a thin
  adapter: parse arguments, take the lock, call core, emit an event. This is
  what lets `cargo test` exercise the entire product — migrations, ordering,
  block state, dedupe, bundle round trip — on any platform, with no window and
  no webview. Tauri commands themselves stay too thin to hold a bug.

- **Hand-rolled `PRAGMA user_version` ladder instead of `rusqlite_migration`.**
  The spec allows either. The ladder is thirty lines, has no version churn, and
  the "refuse to open a database from a newer build" rule is easier to state
  directly than to configure. Tested up from empty, for idempotency, and for
  the newer-database refusal.

- **No `chrono`/`time` dependency.** The only calendar arithmetic in the app is
  the `yyyy-mm` media bucket. That is Howard Hinnant's `civil_from_days`, about
  fifteen lines, and it is directly unit-testable including the leap-day and
  pre-epoch cases. A dependency for this would be larger than the code.

- **No `base64` dependency.** The only base64 in the app is the JPEG a
  `<canvas>` hands back for a thumbnail. A 25-line decoder that also tolerates
  a whole `data:` URL is smaller than the crate and is covered by tests.

## Data model

- **`BlockState` is computed on every read, never stored.** The spec forbids
  storing it and the reason is worth restating: the moment a file is deleted
  outside the app, a stored state is a lie. `state::compute` is the single
  decision point, takes `(block, asset status, importing)` and is exhaustively
  tested across every kind × payload combination. An integration test asserts
  the schema contains no state column at all.

- **`Anchor` makes placement explicit rather than overloading `None`.**
  `after_id: None` cannot mean the same thing for creation and for movement.
  Creating a chapter or block with no anchor means "put it at the end" — that
  is what "new chapter" does. Dragging an item with no anchor means "put it at
  the front" — there is nothing for it to go after. Leaving both as a bare
  `Option` produced a real bug: scaffolded chapter templates came out in
  reverse order, because every template block was created with `None` and each
  one prepended. An internal `Anchor { Start, End, After(id) }` now names the
  intent, and the two public entry points map `None` differently and say so.
  Inserting above the very first block is create-then-reorder-to-front, one
  extra call on one rare path, wrapped in a single mutation so the UI never
  sees the intermediate state.

- **Soft delete keeps blocks attached to their deleted chapter.** Deleting a
  chapter marks only the chapter row. Its blocks are left alone, so a restore
  brings the whole page back exactly as it was, assets included. Only `purge`
  cascades.

- **Fractional index rebalance is checked before the insert, not after.**
  `key_after` projects the key it is about to hand out into the existing list
  and rebalances if that projection would collapse a gap. Checking afterwards
  would mean writing a bad key first and repairing it, which is a worse failure
  mode if the transaction is interrupted.

- **An unknown `kind` string degrades to `text` rather than failing the read.**
  A hand-edited or future-schema database should cost one odd-looking block,
  not a chapter that will not open.

## Assets

- **Import runs in four phases so the database lock is never held over IO.**
  probe (hash) → dedupe check → place (copy) → record. Phases 1 and 3 run on
  `spawn_blocking` with no lock; phases 2 and 4 take the lock for microseconds.
  This is what keeps the UI responsive while a 2 GB file streams in, with one
  connection as the spec requires.

- **The asset protocol scope is `$APPDATA/media/**`, not
  `$APPDATA/<identifier>/media/**`.** The specification's config snippet has
  the identifier in the path twice. In Tauri v2 `$APPDATA` already resolves to
  the *application's* data directory — `%APPDATA%\in.electronix.rustclass` on
  Windows — so adding the identifier again produces a path that can never
  match. The symptom is silent: no error in the UI, just a broken image icon
  and a video that will not load. It cost an hour to find and is written down
  here so it is never reintroduced.

- **Dedupe pre-filters on byte length before it hashes.** Hashing up front
  would let an import skip the copy entirely when the file is already in the
  library, but it costs a full read of *every* import — doubling the time for
  the common case, a file that is not a duplicate. Byte length is an index
  lookup and rules out a duplicate outright. Only when some existing asset is
  exactly the same size is a full hash worth paying for; otherwise the copy
  hashes as it goes and a duplicate is caught at the end, where the wasted copy
  is deleted. A 1.2 GB import now reads the file once instead of twice, and
  progress runs continuously from 0 to 100 rather than sitting at zero through
  a silent hashing pass.

- **`reference` imports are hashed too.** Nothing forces it — the bytes are not
  being copied. Doing it anyway means dedupe and integrity checks behave
  identically for both modes, and a reference whose original was swapped out is
  detectable.

- **A bundle import turns every `reference` asset into a managed copy.** An
  absolute path from another machine is meaningless on this one. On import the
  asset either carries its bytes (and becomes a copy) or shows as `Missing`
  with the relink flow available. Silently keeping a dead absolute path would
  be worse than either.

- **GC is the only code that deletes bytes, and `reference` files are exempt.**
  A referenced file belongs to the user, not to the app; GC drops the row and
  leaves the file alone. Managed copies are deleted only after a dry run has
  listed them.

- **`sha2` and the compression crates are optimised even in debug builds.**
  Unoptimised, SHA-256 runs roughly an order of magnitude slower, which makes a
  debug build look broken on a multi-gigabyte import even though the release
  build is fine. Five `[profile.dev.package.*]` entries cost nothing and keep
  development honest about how the app actually performs.

- **Zip entries for media are `Stored`, not `Deflated`.** An MP4 is already
  compressed. Deflating a 2 GB file to save nothing would add minutes to every
  export. The manifest itself is deflated.

## Errors and safety

- **`AppError` carries `detail` only for storage and IO failures.** Those are
  never the user's fault and never self-explanatory, so the raw cause rides
  along for `/health` and the log. A validation message is already a sentence
  and does not need a debug dump appended.

- **FTS5 queries are tokenised and re-quoted, never interpolated.** A stray `"`
  or `*` in the search box is a syntax error to FTS5. `to_match_expr` strips
  everything that is not alphanumeric and rebuilds the query as quoted prefix
  terms, so the palette can never throw at the user.

- **`guard_inside` is checked before every write and every unlink**, using
  lexical normalisation rather than `canonicalize` — the target usually does
  not exist yet at the point the check matters.

## Shipping

- **Long chapters use `content-visibility`, not a windowing library.** A
  hundred-block chapter has to scroll at 60 fps, but every windowing library
  removes off-screen rows from the DOM — which breaks `@dnd-kit`'s sortable
  context, in-page search and anchor links, all of which this app relies on.
  `content-visibility: auto` with `contain-intrinsic-size` gets the browser to
  skip layout, style and paint for off-screen blocks while leaving every node
  where it is. It is disabled on the row being dragged, which has to stay
  measurable. Video blocks additionally mount their player only once the block
  is near the viewport, so opening such a chapter does not create a hundred
  media elements. Measured at 250 scroll ticks in 1.07 s over 120 blocks.

- **`crate-type = ["rlib"]`, dropping `staticlib` and `cdylib`.** The stock
  Tauri template lists all three to support mobile targets. RustClass is a
  Windows desktop application, and a `cdylib` of this size exceeds the number
  of symbols mingw's linker can export — a Windows cross-build fails at the
  link step for a reason that has nothing to do with the app. The binary links
  from `main.rs` against the rlib either way.

- **The Windows installer was cross-built with the GNU toolchain here, but MSVC
  is what should ship.** `cargo-xwin` could not fetch the Microsoft SDK in this
  environment, so the NSIS installer was produced through
  `x86_64-pc-windows-gnu` to prove the bundle configuration end to end. On a
  Windows host `npm run tauri build` uses MSVC by default, which is Tauri's
  supported toolchain, and produces the same installer.
