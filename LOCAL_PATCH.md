# LOCAL_PATCH.md — Local Patch Notes

This branch `local/vietnamese-annot` contains the local integration of **PR #2066** (`Vietnamese and other non-Western text in Add Text, text box comments and form fields` by BrianDo35).

## Upstream Base
- **Base commit**: `88dad98dd478d255399b1405b33765f4da542704` (tip of `origin/main` as of Oct 22, 2026).
- **PR #2066 Source Commit**: `607b1161797ff00b3b7e546e1af51145bd2f60ff` (authored Oct 14, 2026).

## Scope & Implementation Details
1. **Font Support**:
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

## Verification Checklist Passed
- [x] `cargo test -p pdfcraft-fonts` (79/79 passed)
- [x] `cargo test -p pdfcraft-annot` (46/46 passed)
- [x] `cargo test -p pdfcraft-forms` (124/124 passed)
- [x] `cargo test -p pdfcraft-edit added_vietnamese` (1/1 passed)
- [x] `pdfcraft-cli run` adding Vietnamese TextBox with technical symbols (`≥ 10mm ± 0.5mm, nhiệt độ 25°C, đường kính Ø100`):
  - No warnings emitted.
  - Appearance stream rendered with `/PCUni0`.
  - `pdfcraft-cli text` extracts full text identically.
  - `doc_flatten` succeeds with status `ok`.
  - ASCII-only annotation regression: file size remains 1.6 KB with Helvetica.
