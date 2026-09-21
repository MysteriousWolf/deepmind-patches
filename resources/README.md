# Resources

Sources a host reads through `index.toml`. Nothing here is rendered in this
repository; a host draws what it needs from the bits and the hex values.

- `icons/categories/<Category>.toml`: the 7x7 icon a patch falls back to when
  it has none of its own. One per category, named as the instrument writes it.
- `icons/badges/<name>.toml`: 7x7 icons for derived facts: `arp`, `seq`,
  `unison`, `fx`.
- `icons/genre/`, `icons/mood/`, `icons/timbre/`, `icons/role/`: one 7x7 icon
  per term in `taxonomy.toml`. CI refuses a term without an icon and an icon
  without a term.
- `palette.toml`: named colours for the display, the categories and the axes.

Icon format: seven strings of seven characters, `#` lit, `.` unlit. See
`docs/format.md`. A text preview of every icon is kept in `docs/icons.md` and
regenerated with `cargo run -p validate -- icons --write docs/icons.md`.
