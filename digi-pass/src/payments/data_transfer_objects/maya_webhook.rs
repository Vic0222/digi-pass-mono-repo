use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MayaWebhookRequest {
    pub id: String,

    pub is_paid: bool,

    pub status: String,

    pub amount: String,

    pub currency: String,

    pub can_void: bool,

    pub can_refund: bool,

    pub can_capture: bool,

    pub created_at: String,

    pub updated_at: String,

    pub description: String,

    pub payment_token_id: String,

    pub fund_source: FundSource,

    pub receipt: Receipt,

    pub metadata: Metadata,

    pub approval_code: String,

    pub receipt_number: String,

    pub request_reference_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundSource {
    #[serde(rename = "type")]
    pub fund_source_type: String,

    pub id: Option<serde_json::Value>,

    pub description: String,

    pub details: Details,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Details {
    pub scheme: String,

    pub last4: String,

    pub first6: String,

    pub masked: String,

    pub issuer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub transaction_id: String,

    pub receipt_no: String,

    #[serde(rename = "approval_code")]
    pub receipt_approval_code: String,

    pub approval_code: String,
}