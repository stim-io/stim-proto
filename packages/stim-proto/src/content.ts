export interface LayoutHint {
  layout_family?: string;
  min_height_px?: number;
  max_height_px?: number;
  vertical_pressure?: string;
  metadata?: Record<string, unknown>;
}

export interface MessageContent {
  parts: ContentPart[];
  layout_hint?: LayoutHint;
}

interface BaseContentPart {
  part_id: string;
  revision: number;
  metadata?: Record<string, unknown>;
}

export interface TextPart extends BaseContentPart {
  type: "text";
  text: string;
}

export interface AssetRefPart extends BaseContentPart {
  type: "asset_ref";
  asset_ref: string;
  mime_type?: string;
}

export interface StimDomFragmentPayload {
  format: "stim-dom-fragment/v1";
  tree: Record<string, unknown>;
  bindings?: Record<string, unknown>;
}

export interface RawHtmlDomFragmentPayload {
  format: "raw_html";
  html: string;
  bindings?: Record<string, unknown>;
}

export interface DomFragmentPart extends BaseContentPart {
  type: "dom_fragment";
  payload: StimDomFragmentPayload | RawHtmlDomFragmentPayload;
}

export interface CapabilityRefPart extends BaseContentPart {
  type: "capability_ref";
  capability_kind: string;
  capability_id: string;
  input_schema_ref?: string;
  resource_ref?: string;
}

export type ContentPart =
  | TextPart
  | AssetRefPart
  | DomFragmentPart
  | CapabilityRefPart;

export interface PatchOperation {
  index: number;
  merge: Record<string, unknown>;
}

export interface InsertOperation {
  index: number;
  part: ContentPart;
}
