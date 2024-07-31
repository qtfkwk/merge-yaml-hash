# About

YAML Hash with merge/update capabilities

Wrapper around `yaml_rust::yaml::Hash`, which is a type alias for
`linked_hash_map::LinkedHashMap`

**NOTE: Highly recommend using [`yaml-hash`] instead of this crate since it uses [`yaml-rust2`]
versus the unmaintained [`yaml-rust`].
Also, it provides additional functionality, including recursive get value via dotted key.**

[`yaml-hash`]: https://crates.io/crates/yaml-hash
[`yaml-rust`]: https://crates.io/crates/yaml-rust
[`yaml-rust2`]: https://crates.io/crates/yaml-rust2

# Example

```
use merge_yaml_hash::{MergeYamlHash, Yaml};

let mut hash = MergeYamlHash::new();

// Merge YAML data from strings
hash.merge("apple: 1\nbanana: 2");
hash.merge("cherry:\n  sweet: 1\n  tart: 2");
assert_eq!(
    hash.to_string(),
    "apple: 1\nbanana: 2\ncherry:\n  sweet: 1\n  tart: 2",
);

// Merge YAML data from file
// * Note that insertion order is maintained
hash.merge("tests/c.yaml"); // "banana: 3"
assert_eq!(
    hash.to_string(),
    "apple: 1\nbanana: 3\ncherry:\n  sweet: 1\n  tart: 2",
);
```

!inc:../CHANGELOG.md

