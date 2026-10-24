# LOCAL_PATCH.md — Local Patch Notes

This branch `local/vietnamese-annot` contains the local integration of **PR #2066** (`Vietnamese and other non-Western text in Add Text, text box comments and form fields` by BrianDo35) on top of the latest upstream `main`.

## Status Summary

### 1. Merged into Upstream
- **PR #4211 (Split document from thumbnail context menus)** by @so-sai:
  - Merged into upstream `main` via Integration Batch PR [#4391](https://github.com/storytold/pdfcraft/pull/4391) on 2026-10-24.
  - The local cherry-pick commit has been dropped from this branch because the feature is now natively present in upstream `main` (`crates/ui-egui/src/canvas.rs`, `editing.rs`, `panels.rs`, and tests).

### 2. Still Local (Pending Upstream Merge)
- **PR #2066 (Vietnamese Unicode Inter Font Support)** by BrianDo35:
  - Rebased and conflict-resolved on top of upstream `main` (`b75cbd92`).
  - Embeds bundled `assets/fonts/Inter-Regular.ttf` (and `Inter-SemiBold.ttf`) as Type0 CIDFont (`pdfcraft_fonts::unicode`) with `/ToUnicode` CMap.
  - Text stays selectable, copyable, and searchable in all PDF viewers.
  - Automatically composes decomposed Vietnamese Unicode (NFD -> NFC) via `pdfcraft_fonts::compose_vietnamese`.
  - FreeText, TextBox, Callout, Add Text, and AcroForm text fields render Vietnamese without `/PCUndrawn` or `?`.

## Upstream Base
- **Base commit**: `b75cbd92` (Merge pull request #4394 from aiongg/r3-batch-c, includes #4391).
- **PR #2066 Commit**: `7359ec38` (authored by BrianDo35).

## Verification Checklist Passed
- [x] `git branch -r --contains 12b5e670` confirms PR #4211 is part of `origin/main`.
- [x] `cargo test -p pdfcraft-fonts`
- [x] `cargo test -p pdfcraft-annot` (including Vietnamese textbox and upstream sticky note tests)
- [x] `cargo test -p pdfcraft-forms` (including Vietnamese form field test)
- [x] `cargo test -p pdfcraft-edit` (including Vietnamese added text test)
- [x] `cargo test -p pdfcraft-ui-egui --test it split_from` (5/5 passed from upstream code)
