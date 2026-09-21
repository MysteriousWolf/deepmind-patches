# Patch format

A patch is two files with the same stem, side by side:

```
presets/Bass/Acid Growl - nyx.syx
presets/Bass/Acid Growl - nyx.toml
```

## The `.syx`

Exactly one program dump, stored as bank A, program 1. The slot means nothing
here; every librarian asks where to put a patch. The category byte inside the
program is the folder the file sits in, and the name stored in the program is
the `name` in the `.toml`.

## The `.toml`

Only what nothing else can know. Every field the `.syx` already answers is
derived by CI and refused as a field.

```toml
name    = "Acid Growl"          # the program's stored name, 16 characters at most
author  = "nyx"                 # a person or a handle, not an email address
version = 2                     # bump whenever the .syx bytes change, start at 1
about   = "Resonant 303-ish bass that opens under velocity. Play it low and short."
licence = "CC-BY-4.0"           # SPDX identifier, or exactly "All rights reserved"

genre  = ["techno", "industrial"]
mood   = ["dark", "aggressive"]
timbre = ["gritty", "analogue"]
role   = ["bass"]

# Optional from here down.

icon = [                        # 7 rows of 7, '#' lit, '.' unlit
  ".......",
  "..###..",
  ".#...#.",
  ".#...#.",
  ".#####.",
  ".#...#.",
  ".......",
]

created  = 2026-09-21           # a TOML date, not a string
model    = "DeepMind 12"        # DeepMind 6, 6X, 12, 12X, 12D or 12XD
firmware = "1.1.5"              # what it was made and checked on
source   = "Reworked from the factory A24, with permission."
tags     = ["303", "velocity"]  # free text; searched, never styled

[[demo]]                        # demos/Bass/Acid Growl - nyx.mp3
about = "Dry, C2 to C4."

[[demo]]                        # demos/Bass/Acid Growl - nyx (mod wheel).mp3
variant = "mod wheel"
about   = "Wheel fully up."
```

### Required

| Field | Rule |
| --- | --- |
| `name` | Equals the name stored in the program. Printable ASCII, 16 characters at most. |
| `author` | Not empty, no `@`. |
| `version` | Integer, 1 or more. Bumped whenever the `.syx` bytes change. |
| `about` | Not empty, 400 characters at most. |
| `licence` | An SPDX identifier, or `All rights reserved`. |
| one of `genre`, `mood`, `timbre`, `role` | At least one term, every term in `taxonomy.toml`. |

### Optional

| Field | Rule |
| --- | --- |
| `icon` | Seven strings of seven `#` or `.`. Not blank. Without it, the category icon is used. |
| `created` | A TOML date. |
| `model` | One of the six model names as Behringer writes them. |
| `firmware` | Digits and dots, like `1.1.5`. |
| `source` | Free text. |
| `tags` | Free text, each 32 characters at most. |
| `demo` | Up to four tables. At most one without `variant`. |

### Demos

The file name is computed: the patch stem, then ` (variant)` when there is
one, under `demos/` in the same folder tree as the patch. `variant` is at most
24 characters, no parentheses. The first entry is what a host plays by default.

### Icons

`#` is lit, `.` is unlit, row one is the top, the first character is the left.
Category icons in `resources/icons/categories/` use the same format. Hosts
receive the bits and draw them however they like; nothing here renders.

## Derived, never typed

CI reads these from the program and writes them to the index:

- category
- effect mode, and the algorithms in the four engines
- arpeggiator on, and its mode
- polyphony mode and unison voice count
- how many of the eight modulation routings are used
- control sequencer on
- transpose
- the SHA-256 of the file
- the fingerprint of the sound

A field for any of these is refused as unknown.

## Filenames

The stem is `{name} - {author}` with `/ \ : * ? " < > |` and control characters
replaced by `_`, runs of whitespace collapsed to one space, and the ends
trimmed. CI computes the stem from the `.toml` and compares; it never parses a
filename back into fields.
