use osl_domain::Gender;
use osl_importer::canonical::{store, validator::CanonicalValidator};

mod common;

#[test]
fn classed_men_and_unclassed_women_survive_csv_preparation() {
    let men = common::men_80(vec![common::athlete("Alex", "Example")]);
    let mut woman = common::athlete("Sam", "Example");
    woman.gender = Gender::F;
    let mut women = common::men_80(vec![woman]);
    women.gender = Gender::F;
    women.weight_class_slug = None;
    let canonical = common::competition("partially-classed", vec![men, women]);
    let path = std::env::temp_dir().join(format!("osl-partially-classed-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&path).unwrap();

    store::write(&path, &canonical).unwrap();
    let before = std::fs::read_to_string(path.join("entries.csv")).unwrap();
    assert!(before.starts_with("Sex,WeightClassKg,"));
    assert!(before.contains("\nM,80,Alex,Example,"));
    assert!(before.contains("\nF,,Sam,Example,"));

    let read = store::read(&path).unwrap();
    CanonicalValidator::validate(&read).unwrap();
    assert_eq!(read.categories.len(), 2);
    let men = read
        .categories
        .iter()
        .find(|c| c.gender == Gender::M)
        .unwrap();
    let women = read
        .categories
        .iter()
        .find(|c| c.gender == Gender::F)
        .unwrap();
    assert_eq!(men.bounds().1, Some(common::decimal("80")));
    assert_eq!(women.bounds(), (None, None));
    assert_eq!(women.athletes[0].gender, Gender::F);
    store::write(&path, &read).unwrap();
    assert_eq!(
        before,
        std::fs::read_to_string(path.join("entries.csv")).unwrap()
    );
    std::fs::remove_dir_all(path).unwrap();
}
