**deepmind_patches > slug**

# Module: slug

## Contents

**Functions**

- [`demo_stem`](#demo_stem) - The filename stem of a demo: the patch stem, then ` (variant)` if any.
- [`id`](#id) - A lowercase, URL-safe identifier: `bass/acid-growl-nyx`.
- [`slug`](#slug) - Makes text safe as part of a filename while keeping it readable.
- [`stem`](#stem) - The filename stem of a patch: `slug("{name} - {author}")`.

**Constants**

- [`FORBIDDEN`](#forbidden) - Characters replaced with `_` because some filesystem refuses them.

---

## deepmind_patches::slug::FORBIDDEN

*Constant*: `&[char]`

Characters replaced with `_` because some filesystem refuses them.



## deepmind_patches::slug::demo_stem

*Function*

The filename stem of a demo: the patch stem, then ` (variant)` if any.

```rust
fn demo_stem(patch_stem: &str, variant: Option<&str>) -> String
```



## deepmind_patches::slug::id

*Function*

A lowercase, URL-safe identifier: `bass/acid-growl-nyx`.

Used for `id` in the index. Not a filename.

```rust
fn id(category: &str, collection: Option<&str>, name: &str, author: &str) -> String
```



## deepmind_patches::slug::slug

*Function*

Makes text safe as part of a filename while keeping it readable.

Forbidden and control characters become `_`, runs of whitespace collapse to
one space, and the ends are trimmed. Nothing else changes.

```rust
fn slug(text: &str) -> String
```



## deepmind_patches::slug::stem

*Function*

The filename stem of a patch: `slug("{name} - {author}")`.

```rust
fn stem(name: &str, author: &str) -> String
```



