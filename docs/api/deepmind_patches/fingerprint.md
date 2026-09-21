**deepmind_patches > fingerprint**

# Module: fingerprint

## Contents

**Structs**

- [`Fingerprint`](#fingerprint) - SHA-256 over what makes a program sound the way it does.

**Functions**

- [`sha256_hex`](#sha256_hex) - SHA-256 of a whole file, for download integrity. Hex.

---

## deepmind_patches::fingerprint::Fingerprint

*Struct*

SHA-256 over what makes a program sound the way it does.

**Tuple Struct**: `()`

**Methods:**

- `fn of(program: &Program) -> Self` - Fingerprints a program.
- `fn as_bytes(self: &Self) -> &[u8; 32]` - The raw digest.
- `fn to_hex(self: &Self) -> String` - Lowercase hex, 64 characters.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FromStr**
  - `fn from_str(text: &str) -> Result<Self, <Self as >::Err>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## deepmind_patches::fingerprint::sha256_hex

*Function*

SHA-256 of a whole file, for download integrity. Hex.

```rust
fn sha256_hex(bytes: &[u8]) -> String
```



