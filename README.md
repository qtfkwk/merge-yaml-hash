# About

Merge YAML Hash

# Changelog

* 0.1.0 (2019-10-15)
    * initial release

# Legal

```
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

running 16 tests
test tests::debug_pretty ... ok
test tests::debug ... ok
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

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 6 tests
test src/lib.rs - MergeYamlHash::merge (line 233) ... ok
test src/lib.rs - MergeYamlHash::merge (line 226) ... ok
test src/lib.rs - MergeYamlHash::merge_vec (line 277) ... ok
test src/lib.rs - MergeYamlHash::merge_vec (line 290) ... ok
test src/lib.rs - MergeYamlHash::new (line 196) ... ok
test src/lib.rs - MergeYamlHash::to_string (line 208) ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

