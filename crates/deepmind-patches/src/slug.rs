//! Filenames computed from names, never the other way round.
//!
//! The stem of a patch is `{name} - {author}`. CI computes it from the TOML and
//! compares; it never parses a stem back into fields, so an author called
//! `Bits - Pieces` is fine.

/// Characters replaced with `_` because some filesystem refuses them.
pub const FORBIDDEN: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

/// Makes text safe as part of a filename while keeping it readable.
///
/// Forbidden and control characters become `_`, runs of whitespace collapse to
/// one space, and the ends are trimmed. Nothing else changes.
#[must_use]
pub fn slug(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;
    for character in text.chars() {
        if character.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        if character.is_control() || FORBIDDEN.contains(&character) {
            out.push('_');
        } else {
            out.push(character);
        }
    }
    out
}

/// The filename stem of a patch: `slug("{name} - {author}")`.
#[must_use]
pub fn stem(name: &str, author: &str) -> String {
    slug(&format!("{name} - {author}"))
}

/// The filename stem of a demo: the patch stem, then ` (variant)` if any.
#[must_use]
pub fn demo_stem(patch_stem: &str, variant: Option<&str>) -> String {
    match variant {
        Some(variant) => format!("{patch_stem} ({})", slug(variant)),
        None => patch_stem.to_owned(),
    }
}

/// A lowercase, URL-safe identifier: `bass/acid-growl-nyx`.
///
/// Used for `id` in the index. Not a filename.
#[must_use]
pub fn id(category: &str, collection: Option<&str>, name: &str, author: &str) -> String {
    let mut parts = vec![kebab(category)];
    if let Some(collection) = collection {
        parts.push(kebab(collection));
    }
    parts.push(format!("{}-{}", kebab(name), kebab(author)));
    parts.join("/")
}

fn kebab(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut dash = false;
    for character in text.chars() {
        if character.is_ascii_alphanumeric() {
            if dash && !out.is_empty() {
                out.push('-');
            }
            dash = false;
            out.push(character.to_ascii_lowercase());
        } else {
            dash = true;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_keeps_names_readable() {
        assert_eq!(slug("Acid Growl"), "Acid Growl");
        assert_eq!(slug("  A   B\t"), "A B");
        assert_eq!(slug("a/b:c*d?e\"f<g>h|i\\j"), "a_b_c_d_e_f_g_h_i_j");
        assert_eq!(slug("x\u{7}y"), "x_y");
    }

    #[test]
    fn stems_and_ids() {
        assert_eq!(stem("Acid Growl", "nyx"), "Acid Growl - nyx");
        assert_eq!(
            demo_stem("Acid Growl - nyx", Some("mod wheel")),
            "Acid Growl - nyx (mod wheel)"
        );
        assert_eq!(demo_stem("Acid Growl - nyx", None), "Acid Growl - nyx");
        assert_eq!(id("SFX", None, "Acid Growl", "nyx"), "sfx/acid-growl-nyx");
        assert_eq!(
            id("Pad", Some("Aurora Pads"), "Aurora 1", "Bits - Pieces"),
            "pad/aurora-pads/aurora-1-bits-pieces"
        );
    }
}
