**deepmind_patches > derived**

# Module: derived

## Contents

**Structs**

- [`Derived`](#derived) - Facts a host filters and styles by, read out of the 242 parameters.

---

## deepmind_patches::derived::Derived

*Struct*

Facts a host filters and styles by, read out of the 242 parameters.

**Fields:**
- `category: Option<crate::category::Category>` - The category the program is stored with, if it is one of the twelve.
- `category_value: u8` - The raw category byte, for the message when it is not one of the twelve.
- `fx_mode: String` - `Insert`, `Send` or `Bypass`. The instrument always has an algorithm
- `effects: Vec<String>` - Full names of the algorithms in the four effect engines, in engine
- `arp: bool` - Whether the arpeggiator is on.
- `arp_mode: Option<String>` - The arpeggiator mode, when it is on.
- `polyphony: String` - The polyphony mode as the instrument labels it: `Poly`, `Unison 4`, `Mono`.
- `unison: u8` - Voices stacked per note. 1 unless a unison mode is set.
- `routings: u8` - How many of the eight modulation routings have a source.
- `sequencer: bool` - Whether the control sequencer is enabled.
- `transpose: i8` - Program transpose in semitones.

**Methods:**

- `fn of(program: &Program) -> Self` - Reads the facts out of a program.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



