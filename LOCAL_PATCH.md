# LOCAL_PATCH.md — Local Patch Notes

This branch `local/vietnamese-annot` contains the local integration of **PR #2066** (`Vietnamese and other non-Western text in Add Text, text box comments and form fields` by BrianDo35).

## Upstream Base
- **Base commit**: `88dad98dd478d255399b1405b33765f4da542704` (tip of `origin/main` as of Oct 22, 2026).
- **PR #2066 Source Commit**: `607b1161797ff00b3b7e546e1af51145bd2f60ff` (authored Oct 14, 2026).

## Scope & Implementation Details
1. **Font Support (PR #2066)**:
   - Embeds bundled `assets/fonts/Inter-Regular.ttf` (and `Inter-SemiBold.ttf`) as a Type0 CIDFont (`pdfcraft_fonts::unicode`) with `/ToUnicode` CMap.
   - Text stays completely **selectable and searchable** in all PDF viewers.
   - Automatically composes decomposed Vietnamese Unicode (NFD -> NFC) via `pdfcraft_fonts::compose_vietnamese` to ensure proper accent rendering without floating combining marks.
2. **Annotation Appearance**:
   - FreeText / TextBox / Callout comments containing non-WinAnsi characters (Vietnamese, Greek, Cyrillic, AEC symbols like `≥`, `≤`) are drawn using Inter (`/PCUni0`).
   - Pure ASCII / WinAnsi text continues to use standard Helvetica Type1 (keeping files under 2 KB when Unicode is not needed).
   - Annotations rendered with Inter are NOT marked with `/PCUndrawn`.
   - `doc_flatten` succeeds cleanly on annotated documents without any data-loss refusal.
3. **Add Text & Form Fields**:
   - Add Text (`crates/edit/src/added.rs`) and AcroForm text fields (`crates/forms/src/appearance.rs`) also draw non-WinAnsi text using Inter stand-in.
4. **Split Document from Thumbnail Context Menus (Commit `bf09cc13`)**:
   - Adds "Split document…" button to both the left Pages panel thumbnail strip (`crates/ui-egui/src/panels.rs`) and the Organize grid (`crates/ui-egui/src/canvas.rs`).
   - Introduces `ViewAction::SplitBeforeSelected` in `canvas.rs` and matches it in `editing.rs` to set `split_draft.mode = SplitMode::Selection`.
   - Fixes edge case: when right-clicking the current page with an empty selection, `panels.rs` explicitly seeds `view.select_pages(&[i])` so the dialog defaults to `Selection` instead of unexpectedly falling back to `Pages`.
   - Comprehensive test suite in `crates/ui-egui/tests/editing.rs` and `viewer.rs` covering all 4 edge cases with `egui_kittest` and `PointerButton::Secondary`.

## Verification Checklist Passed
- [x] `cargo test -p pdfcraft-fonts` (79/79 passed)
- [x] `cargo test -p pdfcraft-annot` (46/46 passed)
- [x] `cargo test -p pdfcraft-forms` (124/124 passed)
- [x] `cargo test -p pdfcraft-edit` (196/196 passed)
- [x] `cargo test -p pdfcraft-ui-egui` (all 480 unit + 59 viewer + 148 editing passed)
- [x] Mutation test on edge case: removing `view.select_pages(&[i])` causes `split_from_pages_panel_context_menu_on_current_page_without_selection` to fail with `left: Pages, right: Selection`, and restoring it passes.
- [x] `pdfcraft-cli split "Hồ sơ nghiệm thu.pdf" --before 2`: splits 2-page document into 2 files preserving form fields and UTF-8 filenames.
