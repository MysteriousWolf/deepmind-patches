**deepmind_patches > icon**

# Module: icon

## Contents

**Structs**

- [`Icon`](#icon) - A 7x7 one-bit picture. Row 0 is the top; bit 0 of a row is the left.

**Constants**

- [`LIT`](#lit) - Character for a lit pixel.
- [`UNLIT`](#unlit) - Character for an unlit pixel.

---

## deepmind_patches::icon::Icon

*Struct*

A 7x7 one-bit picture. Row 0 is the top; bit 0 of a row is the left.

**Methods:**

- `fn from_rows(rows: [u8; 7]) -> Self` - Builds an icon from packed rows, bit 0 leftmost.
- `fn parse<S>(rows: &[S]) -> Result<Self>` - Parses the seven text rows the TOML carries.
- `fn rows_text(self: &Self) -> [String; 7]` - The seven text rows, as the TOML writes them.
- `fn rows(self: &Self) -> &[u8; 7]` - Packed rows, bit 0 leftmost.
- `fn is_lit(self: &Self, x: usize, y: usize) -> bool` - Whether the pixel at `x`, `y` is lit. `false` outside the grid.
- `fn is_blank(self: &Self) -> bool` - Whether nothing is lit.
- `fn pixels(self: &Self) -> Pixels` - The same picture as the protocol crate's grid type.
- `fn from_pixels(pixels: &Pixels) -> Self` - Builds an icon from the protocol crate's grid type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(pixels: Pixels) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> std::result::Result<Self, <D as >::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> std::result::Result<<S as >::Ok, <S as >::Error>`



## deepmind_patches::icon::LIT

*Constant*: `char`

Character for a lit pixel.



## deepmind_patches::icon::UNLIT

*Constant*: `char`

Character for an unlit pixel.



