use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataClass {
    Public,
    Sensitive,
    Secret,
}

pub fn classify(text: &str) -> DataClass {
    if text.contains("SECRET") {
        DataClass::Secret
    } else if text.contains("SENSITIVE") {
        DataClass::Sensitive
    } else {
        DataClass::Public
    }
}
