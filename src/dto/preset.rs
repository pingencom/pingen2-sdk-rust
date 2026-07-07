use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct PresetRelationship {
    pub preset: PresetData,
}

#[derive(Debug, Clone, Serialize)]
pub struct PresetData {
    pub data: PresetRef,
}

#[derive(Debug, Clone, Serialize)]
pub struct PresetRef {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
}

impl PresetRelationship {
    pub fn new(preset_id: impl Into<String>) -> Self {
        Self {
            preset: PresetData {
                data: PresetRef {
                    id: preset_id.into(),
                    resource_type: "presets".to_string(),
                },
            },
        }
    }

    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).expect("PresetRelationship is always serializable")
    }
}
