use serde::{Deserialize, Serialize};
use goni_classifier::DataClass;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedactionProfile {
    pub fail_closed: bool,
}

pub fn redact(text: &str, class: DataClass, profile: &RedactionProfile) -> Result<String, String> {
    match class {
        DataClass::Secret => {
            if profile.fail_closed {
                Ok("[REDACTED]".into())
            } else {
                Err("secret_requires_redaction".into())
            }
        }
        _ => Ok(text.to_string()),
    }
}
