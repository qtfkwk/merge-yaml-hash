#![doc = include_str!("../README.md")]

use hashlink::linked_hash_map::Entry;
use yaml_rust2::yaml::Hash;
pub use yaml_rust2::Yaml;
use yaml_rust2::{YamlEmitter, YamlLoader};

#[cfg(test)]
mod tests;

/**

YAML Hash with merge/update capabilities

Wrapper around `yaml_rust2::yaml::Hash`, which is a type alias for `linked_hash_map::LinkedHashMap`

*/
#[derive(Debug, Default)]
pub struct MergeYamlHash {
    pub data: Hash,
}

impl MergeYamlHash {
    /**

    Create a new/empty hash

    ```
    use merge_yaml_hash::MergeYamlHash;
    let mut hash = MergeYamlHash::new();
    assert_eq!(format!("{:?}", hash), "MergeYamlHash { data: {} }");
    assert_eq!(format!("{:#?}", hash), "MergeYamlHash {\n    data: {},\n}");
    ```

    */
    pub fn new() -> MergeYamlHash {
        MergeYamlHash::default()
    }

    /**

    Merge YAML file or string

    ```
    use merge_yaml_hash::MergeYamlHash;
    let mut hash = MergeYamlHash::new();
    hash.merge("tests/a.yaml");
    assert_eq!(hash.to_string(), "apple: 1\nbanana: 2");
    ```

    ```
    use merge_yaml_hash::MergeYamlHash;
    let mut hash = MergeYamlHash::new();
    let yaml = "apple: 1\nbanana: 2".to_string();
    hash.merge(&yaml);
    assert_eq!(hash.to_string(), yaml);
    ```

    */
    pub fn merge(&mut self, file_or_str: &str) {
        let path = std::path::Path::new(&file_or_str);
        let yaml = if path.is_file() {
            std::fs::read_to_string(path).unwrap()
        } else {
            file_or_str.to_string()
        };
        for doc in YamlLoader::load_from_str(&yaml).unwrap() {
            if let Yaml::Hash(h) = doc {
                self.data = merge_hashes(&self.data, &h);
            }
        }
    }

    /**

    Merge multiple YAML files or strings in order

    No conflicts:

    ```
    use merge_yaml_hash::MergeYamlHash;
    let mut hash = MergeYamlHash::new();
    let yaml1 = "apple: 1\nbanana: 2".to_string();
    let yaml2 = "cherry: 3".to_string();
    let result = "apple: 1\nbanana: 2\ncherry: 3";
    hash.merge_vec(vec![yaml1, yaml2]);
    assert_eq!(hash.to_string(), result);
    ```

    With conflict; value in `yaml2.banana` simply overrides the value in
    `yaml1.banana`:

    ```
    use merge_yaml_hash::MergeYamlHash;
    let mut hash = MergeYamlHash::new();
    let yaml1 = "apple: 1\nbanana: 2".to_string();
    let yaml2 = "banana: 3".to_string();
    let result = "apple: 1\nbanana: 3";
    hash.merge_vec(vec![yaml1, yaml2]);
    assert_eq!(hash.to_string(), result);
    ```

    */
    pub fn merge_vec(&mut self, files_or_strings: Vec<String>) {
        for file_or_string in files_or_strings {
            self.merge(&file_or_string);
        }
    }
}

impl std::fmt::Display for MergeYamlHash {
    /**

    Serialize to YAML string

    ```
    use merge_yaml_hash::MergeYamlHash;
    let mut hash = MergeYamlHash::new();
    let yaml = "apple: 1\nbanana: 2".to_string();
    hash.merge(&yaml);
    assert_eq!(hash.to_string(), yaml);
    ```

    */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut r = String::new();
        let mut emitter = YamlEmitter::new(&mut r);
        let yaml = Yaml::Hash(self.data.clone());
        emitter.dump(&yaml).unwrap();
        r.replace_range(..4, ""); // remove "---\n" at beginning
        write!(f, "{r}")
    }
}

/**

Merge two YAML hashes

*/
fn merge_hashes(a: &Hash, b: &Hash) -> Hash {
    let mut r = a.clone();
    for (k, v) in b.iter() {
        if let Yaml::Hash(bh) = v {
            if let Entry::Occupied(e) = r.entry(k.clone()) {
                if let Yaml::Hash(rh) = e.get().clone() {
                    r.entry(k.clone())
                        .and_modify(|e| *e = Yaml::Hash(merge_hashes(&rh, bh)))
                        .or_insert_with(|| Yaml::Hash(merge_hashes(&rh, bh)));
                    continue;
                }
            }
        }
        r.entry(k.clone())
            .and_modify(|e| *e = v.clone())
            .or_insert_with(|| v.clone());
    }
    r
}
