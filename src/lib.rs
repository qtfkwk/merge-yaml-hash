//! # About
//! 
//! YAML Hash with merge/update capabilities
//! 
//! Wrapper around `yaml_rust::yaml::Hash`, which is a type alias for
//! `linked_hash_map::LinkedHashMap`
//! 
//! # Example
//! 
//! ```
//! use merge_yaml_hash::{MergeYamlHash, Yaml};
//! 
//! let mut hash = MergeYamlHash::new();
//! 
//! // Merge YAML data from strings
//! hash.merge("apple: 1\nbanana: 2");
//! hash.merge("cherry:\n  sweet: 1\n  tart: 2");
//! assert_eq!(
//!     hash.to_string(),
//!     "apple: 1\nbanana: 2\ncherry:\n  sweet: 1\n  tart: 2"
//! );
//! 
//! // Merge YAML data from file
//! // * Note that insertion order is maintained
//! hash.merge("tests/c.yaml"); // "banana: 3"
//! assert_eq!(
//!     hash.to_string(),
//!     "apple: 1\ncherry:\n  sweet: 1\n  tart: 2\nbanana: 3"
//! );
//! ```
//! 
//! # Changelog
//! 
//! * 0.2.0 (2019-10-18)
//!     * make `data` field public
//! * 0.1.1-3 (2019-10-15)
//!     * minor fixes
//! * 0.1.0 (2019-10-15)
//!     * initial release
//! 
//! # Legal
//! 
//! ```text
//! Copyright 2019 qtfkwk
//! 
//! Permission is hereby granted, free of charge, to any person obtaining a copy of 
//! this software and associated documentation files (the "Software"), to deal in 
//! the Software without restriction, including without limitation the rights to 
//! use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies 
//! of the Software, and to permit persons to whom the Software is furnished to do 
//! so, subject to the following conditions:
//! 
//! The above copyright notice and this permission notice shall be included in all 
//! copies or substantial portions of the Software.
//! 
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR 
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE 
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER 
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, 
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE 
//! SOFTWARE.
//! ```
//! 
//! [MIT License](https://opensource.org/licenses/MIT)
//! 
//! # Tests
//! 
//! ```text
//! $ cargo test
//! 
//! running 17 tests
//! test tests::debug ... ok
//! test tests::debug_pretty ... ok
//! test tests::display ... ok
//! test tests::merge_file ... ok
//! test tests::merge_multiple_file_file_no_conflicts ... ok
//! test tests::merge_multiple_file_file_with_conflict ... ok
//! test tests::merge_multiple_file_string_no_conflicts ... ok
//! test tests::merge_multiple_file_string_with_conflict ... ok
//! test tests::merge_multiple_string_file_no_conflicts ... ok
//! test tests::merge_multiple_string_file_with_conflict ... ok
//! test tests::merge_multiple_string_string_no_conflicts ... ok
//! test tests::merge_string ... ok
//! test tests::merge_multiple_string_string_with_conflict ... ok
//! test tests::merge_multiple_string_string_no_conflicts_deep ... ok
//! test tests::to_string ... ok
//! test tests::merge_multiple_string_string_with_conflict_deep ... ok
//! test tests::read_data_fields ... ok
//! 
//! test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//! 
//! 
//! running 1 test
//! test integration_read_data_fields ... ok
//! 
//! test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//! 
//! 
//! running 7 tests
//! test src/lib.rs - MergeYamlHash::merge (line 339) ... ok
//! test src/lib.rs - MergeYamlHash::merge (line 332) ... ok
//! test src/lib.rs -  (line 10) ... ok
//! test src/lib.rs - MergeYamlHash::merge_vec (line 383) ... ok
//! test src/lib.rs - MergeYamlHash::merge_vec (line 396) ... ok
//! test src/lib.rs - MergeYamlHash::new (line 302) ... ok
//! test src/lib.rs - MergeYamlHash::to_string (line 314) ... ok
//! 
//! test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//! 
//! ```
//! 

/*

* Date: 2019-10-18
* Author: qtfkwk+mergeyamlhash@gmail.com

*/

// # Crates

use linked_hash_map::Entry;
use yaml_rust::yaml::Hash;
use yaml_rust::{YamlEmitter, YamlLoader};
pub use yaml_rust::Yaml;

// # Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_pretty() {
        let hash = MergeYamlHash::new();
        assert_eq!(format!("{:#?}", hash), "MergeYamlHash {\n    data: {},\n}");
    }

    #[test]
    fn debug() {
        let hash = MergeYamlHash::new();
        assert_eq!(format!("{:?}", hash), "MergeYamlHash { data: {} }");
    }

    #[test]
    fn display() {
        let hash = MergeYamlHash::new();
        assert_eq!(format!("{}", hash), "{}");
    }

    #[test]
    fn to_string() {
        let hash = MergeYamlHash::new();
        assert_eq!(hash.to_string(), "{}");
    }

    #[test]
    fn merge_string() {
        let mut hash = MergeYamlHash::new();
        let yaml = "apple: 1\nbanana: 2".to_string();
        hash.merge(&yaml);
        assert_eq!(hash.to_string(), yaml);
    }

    #[test]
    fn merge_file() {
        let mut hash = MergeYamlHash::new();
        hash.merge("tests/a.yaml");
        assert_eq!(hash.to_string(), "apple: 1\nbanana: 2");
    }

    #[test]
    fn merge_multiple_string_string_no_conflicts() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "apple: 1\nbanana: 2".to_string();
        let yaml2 = "cherry: 3".to_string();
        let result = "apple: 1\nbanana: 2\ncherry: 3";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_string_string_with_conflict() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "apple: 1\nbanana: 2".to_string();
        let yaml2 = "banana: 3".to_string();
        let result = "apple: 1\nbanana: 3";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_file_string_no_conflicts() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "tests/a.yaml".to_string();
        let yaml2 = "cherry: 3".to_string();
        let result = "apple: 1\nbanana: 2\ncherry: 3";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_file_string_with_conflict() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "tests/a.yaml".to_string();
        let yaml2 = "banana: 3".to_string();
        let result = "apple: 1\nbanana: 3";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_string_file_no_conflicts() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "apple: 1\nbanana: 2".to_string();
        let yaml2 = "tests/b.yaml".to_string();
        let result = "apple: 1\nbanana: 2\ncherry: 3";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_string_file_with_conflict() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "apple: 1\nbanana: 2".to_string();
        let yaml2 = "tests/c.yaml".to_string();
        let result = "apple: 1\nbanana: 3";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_file_file_no_conflicts() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "tests/a.yaml".to_string();
        let yaml2 = "tests/b.yaml".to_string();
        let result = "apple: 1\nbanana: 2\ncherry: 3";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_file_file_with_conflict() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "tests/a.yaml".to_string();
        let yaml2 = "tests/c.yaml".to_string();
        let result = "apple: 1\nbanana: 3";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_string_string_no_conflicts_deep() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "apple: 1\nbanana: 2\ncherry:\n  sweet: 1".to_string();
        let yaml2 = "cherry:\n  tart: 2".to_string();
        let result = "apple: 1\nbanana: 2\ncherry:\n  sweet: 1\n  tart: 2";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn merge_multiple_string_string_with_conflict_deep() {
        let mut hash = MergeYamlHash::new();
        let yaml1 = "apple: 1\nbanana: 2\ncherry:\n  sweet: 1".to_string();
        let yaml2 = "cherry:\n  sweet: 2".to_string();
        let result = "apple: 1\nbanana: 2\ncherry:\n  sweet: 2";
        hash.merge_vec(vec![yaml1, yaml2]);
        assert_eq!(hash.to_string(), result);
    }

    #[test]
    fn read_data_fields() {
        let mut hash = MergeYamlHash::new();
        let apple = Yaml::String("apple".to_string());
        let banana = Yaml::String("banana".to_string());
        let cherry = Yaml::String("cherry".to_string());
        let sweet = Yaml::String("sweet".to_string());
        hash.merge("apple: 1\nbanana: 2\ncherry:\n  sweet: 3");
        let cherries = hash.data[&cherry].as_hash().unwrap();
        assert_eq!(hash.data[&apple].as_i64().unwrap(), 1);
        assert_eq!(hash.data[&banana].as_i64().unwrap(), 2);
        assert_eq!(cherries[&sweet].as_i64().unwrap(), 3);
    }
}

// # Structures

/// YAML Hash with merge/update capabilities
///
/// Wrapper around `yaml_rust::yaml::Hash`, which is a type alias for
/// `linked_hash_map::LinkedHashMap`
#[derive(Debug)]
pub struct MergeYamlHash {
    pub data: Hash,
}

impl MergeYamlHash {
    /// Create a new/empty hash
    ///
    /// ```
    /// use merge_yaml_hash::MergeYamlHash;
    /// let mut hash = MergeYamlHash::new();
    /// assert_eq!(format!("{:?}", hash), "MergeYamlHash { data: {} }");
    /// assert_eq!(format!("{:#?}", hash), "MergeYamlHash {\n    data: {},\n}");
    /// ```
    pub fn new() -> MergeYamlHash {
        MergeYamlHash { data: Hash::new() }
    }

    /// Serialize to YAML string
    ///
    /// ```
    /// use merge_yaml_hash::MergeYamlHash;
    /// let mut hash = MergeYamlHash::new();
    /// let yaml = "apple: 1\nbanana: 2".to_string();
    /// hash.merge(&yaml);
    /// assert_eq!(hash.to_string(), yaml);
    /// ```
    fn to_string(&self) -> String {
        let mut r = String::new();
        let mut emitter = YamlEmitter::new(&mut r);
        let yaml = Yaml::Hash(self.data.clone());
        emitter.dump(&yaml).unwrap();
        r.replace_range(..4, ""); // remove "---\n" at beginning
        r
    }

    /// Merge YAML file or string
    ///
    /// ```
    /// use merge_yaml_hash::MergeYamlHash;
    /// let mut hash = MergeYamlHash::new();
    /// hash.merge("tests/a.yaml");
    /// assert_eq!(hash.to_string(), "apple: 1\nbanana: 2");
    /// ```
    ///
    /// ```
    /// use merge_yaml_hash::MergeYamlHash;
    /// let mut hash = MergeYamlHash::new();
    /// let yaml = "apple: 1\nbanana: 2".to_string();
    /// hash.merge(&yaml);
    /// assert_eq!(hash.to_string(), yaml);
    /// ```
    pub fn merge(&mut self, file_or_str: &str) {
        let path = std::path::Path::new(&file_or_str);
        let yaml: String;
        if path.is_file() {
            yaml = std::fs::read_to_string(&path).unwrap();
        } else {
            yaml = file_or_str.to_string();
        }
        for doc in YamlLoader::load_from_str(&yaml).unwrap() {
            if let Yaml::Hash(h) = doc {
                self.data = self.merge_hashes(&self.data, &h);
            }
        }
    }

    /// Merge two YAML hashes
    fn merge_hashes(&self, a: &Hash, b: &Hash) -> Hash {
        let mut r = a.clone();
        for (k, v) in b.iter() {
            if let Yaml::Hash(bh) = v {
                if let Entry::Occupied(e) = r.entry(k.clone()) {
                    if let Yaml::Hash(mut rh) = e.get().clone() {
                        rh = self.merge_hashes(&rh, bh);
                        r.insert(k.clone(), Yaml::Hash(rh));
                        continue;
                    }
                }
            }
            r.insert(k.clone(), v.clone());
        }
        r
    }

    /// Merge multiple YAML files or strings in order
    ///
    /// No conflicts:
    ///
    /// ```
    /// use merge_yaml_hash::MergeYamlHash;
    /// let mut hash = MergeYamlHash::new();
    /// let yaml1 = "apple: 1\nbanana: 2".to_string();
    /// let yaml2 = "cherry: 3".to_string();
    /// let result = "apple: 1\nbanana: 2\ncherry: 3";
    /// hash.merge_vec(vec![yaml1, yaml2]);
    /// assert_eq!(hash.to_string(), result);
    /// ```
    ///
    /// With conflict; value in `yaml2.banana` simply overrides the value in
    /// `yaml1.banana`:
    ///
    /// ```
    /// use merge_yaml_hash::MergeYamlHash;
    /// let mut hash = MergeYamlHash::new();
    /// let yaml1 = "apple: 1\nbanana: 2".to_string();
    /// let yaml2 = "banana: 3".to_string();
    /// let result = "apple: 1\nbanana: 3";
    /// hash.merge_vec(vec![yaml1, yaml2]);
    /// assert_eq!(hash.to_string(), result);
    /// ```
    pub fn merge_vec(&mut self, files_or_strings: Vec<String>) {
        for file_or_string in files_or_strings {
            self.merge(&file_or_string);
        }
    }
}

impl std::fmt::Display for MergeYamlHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
