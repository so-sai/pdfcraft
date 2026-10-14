//! Text the standard 14 fonts can't encode, drawn with Inter: Vietnamese (`ạ ế ơ ư đ`), Central
//! European, Greek, Cyrillic. WinAnsiEncoding stops at Western European letters, so such text used
//! to be refused ("only Western European characters can be written there for now") or drawn as `?`.
//!
//! Inter is the bundled interface face (`assets/fonts`, OFL, listed in `ATTRIBUTION.toml`); typed
//! signatures already draw with it. It is embedded whole as a Type0 font through
//! [`crate::embedding::embed`] (one program per document, shared by every item that uses it), with a
//! ToUnicode map, so the text stays selectable and searchable. Text is composed to NFC first
//! ([`crate::compose_vietnamese`]), since a PDF text show positions no combining marks.

use pdfcraft_cos::{Dict, Document, Object};
use skrifa::instance::{LocationRef, Size};
use skrifa::{FontRef, MetadataProvider};

use crate::pdf::Metrics;

/// Inter SemiBold, for bold text.
pub static INTER_SEMIBOLD: &[u8] = include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf");

/// The Inter face for a weight.
pub fn face(bold: bool) -> &'static [u8] {
    if bold { INTER_SEMIBOLD } else { crate::INTER_REGULAR }
}

/// Does Inter draw every character of `text` (line breaks aside)?
pub fn covers(text: &str) -> bool {
    crate::embedding::covers(crate::INTER_REGULAR, &crate::compose_vietnamese(text))
}

/// Text that needs Inter: something WinAnsi can't encode, all of which Inter draws.
pub fn needed_for(text: &str) -> bool {
    crate::first_undrawable(text).is_some() && covers(text)
}

/// Width of `text` set in Inter at `size` points.
pub fn width(text: &str, size: f64, bold: bool) -> f64 {
    let Ok(font) = FontRef::new(face(bold)) else { return crate::helvetica_width(text, size) };
    let upem = f64::from(font.head().map(|h| h.units_per_em()).unwrap_or(1000).max(1));
    let cmap = font.charmap();
    let adv = font.glyph_metrics(Size::unscaled(), LocationRef::default());
    let units: f64 = crate::compose_vietnamese(text).chars().map(|c| cmap.map(c).and_then(|g| adv.advance_width(g)).map_or(0.0, f64::from)).sum();
    let w = units / upem * size;
    if w.is_finite() { w } else { 0.0 }
}

use skrifa::raw::TableProvider as _;

/// Inter embedded in a document, under a resource name of its own.
pub struct Embedded {
    /// The font resource name (`{prefix}{n}`, free in the `taken` resources it was made for).
    pub resource: String,
    /// The Type0 font object (a reference), for a `/Font` resource dictionary.
    pub font: Object,
    metrics: Metrics,
}

impl Embedded {
    /// Embed Inter (SemiBold when `bold`) in `doc`, named `{prefix}{n}` with the first `n` that
    /// `taken` (the resources it will join) doesn't use.
    pub fn new(doc: &mut Document, taken: &Dict, prefix: &str, bold: bool) -> Result<Embedded, String> {
        let mut probe = taken.clone();
        let family = if bold { "Inter-SemiBold" } else { "Inter-Regular" };
        let name = crate::embedding::embed(doc, &mut probe, face(bold), family)?;
        let font = probe.get(name.as_bytes()).cloned().ok_or("the embedded font went missing")?;
        let dict = doc.resolve(&font).as_dict().cloned().ok_or("the embedded font is not a dictionary")?;
        let metrics = Metrics::from_dict(doc, &dict);
        let mut n = 0usize;
        while taken.contains(format!("{prefix}{n}").as_bytes()) {
            n = n.saturating_add(1);
        }
        Ok(Embedded { resource: format!("{prefix}{n}"), font, metrics })
    }

    /// The hex string operand (`<…>`) that shows `text`, or `None` if Inter lacks a character.
    pub fn show(&self, text: &str) -> Option<Vec<u8>> {
        let bytes = self.metrics.encode(&crate::compose_vietnamese(text))?;
        let mut out = Vec::with_capacity(bytes.len() * 2 + 2);
        out.push(b'<');
        for b in bytes {
            out.extend_from_slice(format!("{b:02X}").as_bytes());
        }
        out.push(b'>');
        Some(out)
    }

    /// The width of `text` at `size`, from the embedded font's own widths.
    pub fn width(&self, text: &str, size: f64) -> f64 {
        let Some(bytes) = self.metrics.encode(&crate::compose_vietnamese(text)) else { return 0.0 };
        self.metrics.codes(&bytes).into_iter().map(|(c, _)| self.metrics.width(c)).sum::<f64>() * size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vietnamese_needs_and_gets_inter() {
        assert!(needed_for("Tiếng Việt"));
        assert!(needed_for("Ti\u{00EA}\u{0301}ng"), "decomposed is composed first");
        assert!(!needed_for("café"), "WinAnsi text keeps the standard fonts");
        assert!(!needed_for("日本"), "Inter has no Han");
        assert!(width("Việt", 10.0, false) > width("Vi", 10.0, false));
        assert!(width("ế", 10.0, true) > 0.0);
    }

    #[test]
    fn embedded_inter_shows_vietnamese_and_reads_back() {
        let mut doc = Document::new_empty();
        let mut taken = Dict::new();
        taken.set(b"PCUni0".to_vec(), Object::Null);
        let e = Embedded::new(&mut doc, &taken, "PCUni", false).unwrap();
        assert_eq!(e.resource, "PCUni1", "a free name");
        let shown = e.show("Đơn vị: ữ").unwrap();
        assert!(shown.starts_with(b"<") && shown.ends_with(b">"));
        // The codes decode back to the text through the font's ToUnicode.
        let dict = doc.resolve(&e.font).as_dict().cloned().unwrap();
        let m = Metrics::from_dict(&doc, &dict);
        let hex = std::str::from_utf8(&shown[1..shown.len() - 1]).unwrap();
        let bytes: Vec<u8> = (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect();
        assert_eq!(m.decode(&bytes), "Đơn vị: ữ");
        assert!((e.width("Việt", 10.0) - width("Việt", 10.0, false)).abs() < 0.05);
        // A second item shares the one embedded program.
        let mut taken2 = taken.clone();
        taken2.set(e.resource.as_bytes().to_vec(), e.font.clone());
        let _ = Embedded::new(&mut doc, &taken2, "PCUni", true).unwrap();
        let programs = doc.scan_objects().filter(|(_, o)| matches!(o.as_ref(), Object::Stream(s) if s.dict.get(b"PCFullFont").is_some())).count();
        assert_eq!(programs, 2, "one Regular and one SemiBold program");
    }
}
