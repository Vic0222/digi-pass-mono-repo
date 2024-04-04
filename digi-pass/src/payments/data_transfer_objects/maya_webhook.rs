use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MayaWebhookRequest {
    pub id: String,

    pub items: Vec<Item>,

    pub request_reference_number: String,

    pub receipt_number: String,

    pub created_at: String,

    pub updated_at: String,

    pub payment_scheme: String,

    pub express_checkout: bool,

    pub refunded_amount: String,

    pub can_pay_pal: bool,

    pub expired_at: String,

    pub status: String,

    pub payment_status: String,

    pub payment_details: PaymentDetails,

    pub buyer: Buyer,

    pub merchant: Merchant,

    pub total_amount: TotalAmount,

    pub redirect_url: RedirectUrl,

    pub transaction_reference_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buyer {
    pub contact: Contact,

    pub first_name: String,

    pub last_name: String,

    pub billing_address: IngAddress,

    pub shipping_address: IngAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngAddress {
    pub line1: String,

    pub line2: String,

    pub city: String,

    pub state: String,

    pub zip_code: String,

    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub phone: String,

    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,

    pub code: String,

    pub description: String,

    pub quantity: String,

    pub amount: TotalAmountClass,

    pub total_amount: TotalAmountClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalAmountClass {
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Merchant {
    pub currency: String,

    pub email: String,

    pub locale: String,

    pub homepage_url: String,

    pub is_email_to_merchant_enabled: bool,

    pub is_email_to_buyer_enabled: bool,

    pub is_payment_facilitator: bool,

    pub is_page_customized: bool,

    pub supported_schemes: Vec<String>,

    pub can_pay_pal: bool,

    pub pay_pal_email: Option<serde_json::Value>,

    pub pay_pal_web_experience_id: Option<serde_json::Value>,

    pub express_checkout: bool,

    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDetails {
    pub responses: Responses,

    pub payment_at: String,

    #[serde(rename = "3ds")]
    pub the_3_ds: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Responses {
    pub efs: Efs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Efs {
    #[serde(rename = "paymentTransactionReferenceNo")]
    pub payment_transaction_reference_no: String,

    pub status: String,

    pub receipt: Receipt,

    pub payer: Payer,

    pub amount: EfsAmount,

    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfsAmount {
    pub total: Total,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Total {
    pub currency: String,

    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payer {
    pub funding_instrument: FundingInstrument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingInstrument {
    pub card: Card,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub card_number: String,

    pub expiry_month: i64,

    pub expiry_year: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectUrl {
    pub success: String,

    pub failure: String,

    pub cancel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: String,

    pub currency: String,

    pub details: Details,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    pub discount: String,

    pub service_charge: String,

    pub shipping_fee: String,

    pub tax: String,

    pub subtotal: String,
}
