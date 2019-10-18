use merge_yaml_hash::{MergeYamlHash, Yaml};

#[test]
fn integration_read_data_fields() {
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
