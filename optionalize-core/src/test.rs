
use super::OptionalizeTrait;

struct TestStruct {
    pub id: i32,
    pub name: String,
}

struct TestStructOptional {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl OptionalizeTrait for TestStruct {
    type Optional = TestStructOptional;
}

#[test]
fn test_optionalize_trait() {
    use crate::OptionalizeTrait;
    let original = TestStruct {
        id: 1,
        name: "example".to_string(),
    };

    let optionalized: <TestStruct as OptionalizeTrait>::Optional = TestStructOptional {
        id: Some(original.id),
        name: Some(original.name),
    };

    assert_eq!(optionalized.id, Some(1));
    assert_eq!(optionalized.name, Some("example".to_string()));
}
