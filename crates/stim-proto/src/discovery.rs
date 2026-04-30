use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Address, EndpointId, NodeId, ProtocolVersion};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct EndpointDeclaration {
    pub endpoint_id: EndpointId,
    pub node_id: NodeId,
    pub display_label: Option<String>,
    pub endpoint_kind: Option<String>,
    pub supported_protocol_versions: Vec<ProtocolVersion>,
    pub supported_carriers: Vec<String>,
    pub content_capabilities: Vec<String>,
    pub security_capabilities: Vec<String>,
    pub declared_features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct DiscoveryRecord {
    pub node_id: NodeId,
    pub endpoint_declaration: EndpointDeclaration,
    pub carrier_kind: String,
    pub addresses: Vec<Address>,
    pub protocol_versions: Vec<ProtocolVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct DeliveryTarget {
    pub node_id: NodeId,
    pub carrier_kind: String,
    pub selected_address: Address,
    pub protocol_version: ProtocolVersion,
}
