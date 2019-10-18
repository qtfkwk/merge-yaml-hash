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
    "apple: 1\nbanana: 2\ncherry:\n  sweet: 1\n  tart: 2"
);

// Merge YAML data from file
// * Note that insertion order is maintained
hash.merge("tests/c.yaml"); // "banana: 3"
assert_eq!(
    hash.to_string(),
    "apple: 1\ncherry:\n  sweet: 1\n  tart: 2\nbanana: 3"
);
```

# Changelog

* 0.2.0 (2019-10-18)
    * make `data` field public
* 0.1.1-3 (2019-10-15)
    * minor fixes
* 0.1.0 (2019-10-15)
    * initial release

# Legal

```text
Copyright 2019 qtfkwk

Permission is hereby granted, free of charge, to any person obtaining a copy of 
this software and associated documentation files (the "Software"), to deal in 
the Software without restriction, including without limitation the rights to 
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies 
of the Software, and to permit persons to whom the Software is furnished to do 
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all 
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR 
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE 
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER 
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, 
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE 
SOFTWARE.
```

[MIT License](https://opensource.org/licenses/MIT)

# Tests

```text
$ cargo test

running 17 tests
test tests::debug ... ok
test tests::debug_pretty ... ok
test tests::display ... ok
test tests::merge_file ... ok
test tests::merge_multiple_file_file_no_conflicts ... ok
test tests::merge_multiple_file_file_with_conflict ... ok
test tests::merge_multiple_file_string_no_conflicts ... ok
test tests::merge_multiple_file_string_with_conflict ... ok
test tests::merge_multiple_string_file_no_conflicts ... ok
test tests::merge_multiple_string_file_with_conflict ... ok
test tests::merge_multiple_string_string_no_conflicts ... ok
test tests::merge_string ... ok
test tests::merge_multiple_string_string_with_conflict ... ok
test tests::merge_multiple_string_string_no_conflicts_deep ... ok
test tests::to_string ... ok
test tests::merge_multiple_string_string_with_conflict_deep ... ok
test tests::read_data_fields ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 1 test
test integration_read_data_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 7 tests
test src/lib.rs - MergeYamlHash::merge (line 339) ... ok
test src/lib.rs - MergeYamlHash::merge (line 332) ... ok
test src/lib.rs -  (line 10) ... ok
test src/lib.rs - MergeYamlHash::merge_vec (line 383) ... ok
test src/lib.rs - MergeYamlHash::merge_vec (line 396) ... ok
test src/lib.rs - MergeYamlHash::new (line 302) ... ok
test src/lib.rs - MergeYamlHash::to_string (line 314) ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

