**deepmind_patches > validate**

# Module: validate

## Contents

**Structs**

- [`Finding`](#finding) - One thing wrong, and what to do about it.

**Enums**

- [`Severity`](#severity) - How bad a finding is.

**Functions**

- [`fails`](#fails) - Whether a run's findings should fail: any error, or any finding under
- [`line_of`](#line_of) - The 1-based line a top-level key or table header sits on.
- [`run`](#run) - Runs every rule over a scan.

---

## deepmind_patches::validate::Finding

*Struct*

One thing wrong, and what to do about it.

**Fields:**
- `severity: Severity` - Error or warning.
- `path: std::path::PathBuf` - The file, relative to the root.
- `line: Option<u32>` - The line, when one applies.
- `message: String` - What is wrong.
- `hint: Option<String>` - What to do.

**Methods:**

- `fn error<impl Into<PathBuf>, impl Into<String>>(path: impl Trait, message: impl Trait) -> Self` - An error with no hint.
- `fn warning<impl Into<PathBuf>, impl Into<String>>(path: impl Trait, message: impl Trait) -> Self` - A warning with no hint.
- `fn hint<impl Into<String>>(self: Self, hint: impl Trait) -> Self` - Adds the fix.
- `fn at(self: Self, line: Option<u32>) -> Self` - Adds the line.
- `fn annotation(self: &Self) -> String` - A GitHub Actions workflow command, which turns into an annotation on the

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## deepmind_patches::validate::Severity

*Enum*

How bad a finding is.

**Variants:**
- `Warning` - Worth fixing; fails only under `--strict`.
- `Error` - Fails the build.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## deepmind_patches::validate::fails

*Function*

Whether a run's findings should fail: any error, or any finding under
`strict`.

```rust
fn fails(findings: &[Finding], strict: bool) -> bool
```



## deepmind_patches::validate::line_of

*Function*

The 1-based line a top-level key or table header sits on.

```rust
fn line_of(text: &str, key: &str) -> Option<u32>
```



## deepmind_patches::validate::run

*Function*

Runs every rule over a scan.

```rust
fn run(scan: &crate::library::Scan) -> Vec<Finding>
```



