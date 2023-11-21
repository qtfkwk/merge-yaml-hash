# About

YAML Hash with merge/update capabilities

Wrapper around `yaml_rust::yaml::Hash`, which is a type alias for
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
    "apple: 1\nbanana: 3\ncherry:\n  sweet: 1\n  tart: 2",
);
```

!inc:../CHANGELOG.md

