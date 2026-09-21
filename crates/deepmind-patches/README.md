# deepmind-patches

Reads, validates, fingerprints and indexes the shared DeepMind patch library at
[MysteriousWolf/deepmind-patches](https://github.com/MysteriousWolf/deepmind-patches).

```rust
use deepmind_patches::{Index, Fingerprint};

// From a release: browse, and match a program dumped from an instrument.
let index = Index::parse(&std::fs::read_to_string("index.toml")?)?;
for patch in index.in_category(deepmind_patches::Category::Bass) {
    println!("{} by {} v{}", patch.name, patch.author, patch.version);
}
let fingerprint = Fingerprint::of(&program);
if let Some(found) = index.lookup(&fingerprint) {
    println!("{} v{}{}", found.patch.name, found.version, if found.latest { "" } else { " (newer available)" });
}
```

With the `fetch` feature, `fetch::latest_index`, `fetch::update_for` and
`fetch::unpack` download releases without touching the GitHub API.

`Library::open` reads a checkout; `validate::run` on `Library::scan` gives
every finding CI would report.
