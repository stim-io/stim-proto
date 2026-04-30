import type { Address, EndpointId, NodeId, ProtocolVersion } from "./ids.js";

export interface EndpointDeclaration {
  endpoint_id: EndpointId;
  node_id: NodeId;
  display_label?: string;
  endpoint_kind?: string;
  supported_protocol_versions: ProtocolVersion[];
  supported_carriers: string[];
  content_capabilities: string[];
  security_capabilities: string[];
  declared_features: string[];
}

export interface DiscoveryRecord {
  node_id: NodeId;
  endpoint_declaration: EndpointDeclaration;
  carrier_kind: string;
  addresses: Address[];
  protocol_versions: ProtocolVersion[];
}

export interface DeliveryTarget {
  node_id: NodeId;
  carrier_kind: string;
  selected_address: Address;
  protocol_version: ProtocolVersion;
}
