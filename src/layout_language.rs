#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LayoutLanguage {
    #[default]
    English,
    Russian,
}

/// English letter → Russian letter for the standard ЙЦУКЕН layout.
const EN_TO_RU: &[(&str, &str)] = &[
    ("A", "Ф"), ("B", "И"), ("C", "С"), ("D", "В"), ("E", "Е"),
    ("F", "А"), ("G", "П"), ("H", "Р"), ("I", "Ш"), ("J", "О"),
    ("K", "Л"), ("L", "Д"), ("M", "М"), ("N", "Т"), ("O", "Ь"),
    ("P", "З"), ("Q", "Й"), ("R", "К"), ("S", "Ы"), ("T", "Е"),
    ("U", "Г"), ("V", "М"), ("W", "Ц"), ("X", "Ч"), ("Y", "Н"),
    ("Z", "Я"),
];

use crate::layout_key::LayoutKey;

/// Post-process a `LayoutKey` and replace the tap label with the
/// corresponding letter in `language` (only affects A-Z letter keys).
pub fn localize_layout_key(key: &LayoutKey, language: LayoutLanguage) -> LayoutKey {
    match language {
        LayoutLanguage::English => key.clone(),
        LayoutLanguage::Russian => {
            let tap = key.tap.full.as_str();
            if let Some(ru) = EN_TO_RU.iter().find(|&&(en, _)| en == tap).map(|&(_, ru)| ru) {
                let mut localized = key.clone();
                localized.tap.full = ru.to_string();
                localized.tap.short = None;
                localized
            } else {
                key.clone()
            }
        }
    }
}
