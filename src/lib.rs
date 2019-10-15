/*

* Date: 2019-10-15
* Author: qtfkwk+mergeyamlhash@gmail.com
* Legal:
    * Copyright 2019 by qtfkwk

```
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

*/

// # Crates

use linked_hash_map::Entry;
use yaml_rust::yaml::Hash;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

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
}

// # Structures

/// YAML Hash with merge capabilities
///
/// Wrapper around `yaml_rust::yaml::Hash`, which is a type alias for
/// `linked_hash_map::LinkedHashMap`
#[derive(Debug)]
pub struct MergeYamlHash {
    data: Hash,
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
