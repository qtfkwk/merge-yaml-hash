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
fn merge_multiple_string_string_with_conflict_2() {
    let mut hash = MergeYamlHash::new();
    let yaml1 = "apple: 1\nbanana: 2".to_string();
    let yaml2 = "apple: 3".to_string();
    let result = "apple: 3\nbanana: 2";
    hash.merge_vec(vec![yaml1, yaml2]);
    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_string_string_with_conflict_3() {
    let mut hash = MergeYamlHash::new();
    let yaml1 = "apple: 1\nbanana: 2\ncherry: 3".to_string();
    let yaml2 = "banana: 4".to_string();
    let result = "apple: 1\nbanana: 4\ncherry: 3";
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
