# About

YAML Hash with merge/update capabilities

Wrapper around `yaml_rust2::yaml::Hash`, which is a type alias for
`linked_hash_map::LinkedHashMap`

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
    "apple: 1\ncherry:\n  sweet: 1\n  tart: 2\nbanana: 3",
);
```

# Changelog

* 0.1.0 (2019-10-15): initial release
* 0.1.1-3 (2019-10-15): minor fixes
* 0.2.0 (2019-10-18): Make `data` field public
* 0.3.0 (2023-11-21): Resolve [issue #1]; update edition and dependencies; apply
  clippy suggestions; modernize

[issue #1]: https://github.com/qtfkwk/merge-yaml-hash/issues/1

