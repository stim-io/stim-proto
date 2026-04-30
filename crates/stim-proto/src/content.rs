use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct LayoutHint {
    pub layout_family: Option<String>,
    pub min_height_px: Option<u32>,
    pub max_height_px: Option<u32>,
    pub vertical_pressure: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageContent {
    pub parts: Vec<ContentPart>,
    pub layout_hint: Option<LayoutHint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text(TextPart),
    AssetRef(AssetRefPart),
    DomFragment(DomFragmentPart),
    CapabilityRef(CapabilityRefPart),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TextPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct AssetRefPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub asset_ref: String,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "format", rename_all = "snake_case")]
pub enum DomFragmentPayload {
    #[serde(rename = "stim-dom-fragment/v1")]
    StimDomFragmentV1 {
        tree: Value,
        bindings: Option<Value>,
    },
    RawHtml {
        html: String,
        bindings: Option<Value>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct DomFragmentPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    #[serde(flatten)]
    pub payload: DomFragmentPayload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CapabilityRefPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub capability_kind: String,
    pub capability_id: String,
    pub input_schema_ref: Option<String>,
    pub resource_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchOperation {
    pub index: usize,
    pub merge: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct InsertOperation {
    pub index: usize,
    pub part: ContentPart,
}
