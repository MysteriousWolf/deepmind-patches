**deepmind_patches > library > Library**

# Module: library::Library

## Contents

**Functions**

- [`open`](#open) - Reads and validates a checkout, refusing one with errors.
- [`scan`](#scan) - Reads a checkout. Nothing is judged; see [`crate::validate`].

---

## deepmind_patches::library::Library::open

*Function*

Reads and validates a checkout, refusing one with errors.

# Errors

[`Error::Invalid`] with the count when validation fails; run
[`crate::validate::run`] on a scan to see the findings.

```rust
fn open<impl AsRef<Path>>(root: impl Trait) -> Result<Self>
```



## deepmind_patches::library::Library::scan

*Function*

Reads a checkout. Nothing is judged; see [`crate::validate`].

```rust
fn scan(root: &Path) -> Result<Scan>
```



