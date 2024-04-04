pub mod checkout {
    pub mod request {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CheckoutRequest {
            pub total_amount: TotalAmount,

            pub request_reference_number: String,

            pub items: Vec<Item>,
        }

        impl CheckoutRequest {
            pub fn new(
                total_amount: f64,
                currency: String,
                request_reference_number: String,
                items: Vec<Item>,
            ) -> Self {
                CheckoutRequest {
                    total_amount: TotalAmount {
                        value: total_amount,
                        currency,
                    },
                    request_reference_number,
                    items,
                }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Item {
            pub amount: Amount,

            pub quantity: String,

            pub total_amount: Amount,

            pub name: String,

            pub code: String,

            pub description: String,
        }

        impl Item {
            pub fn new(
                amount: f64,
                quantity: String,
                total_amount: f64,
                name: String,
                code: String,
                description: String,
            ) -> Self {
                Item {
                    amount: Amount { value: amount },
                    quantity,
                    total_amount: Amount {
                        value: total_amount,
                    },
                    name,
                    code,
                    description,
                }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Amount {
            pub value: f64,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TotalAmount {
            pub value: f64,

            pub currency: String,
        }
    }

    pub mod response {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CheckoutResponse {
            pub checkout_id: String,

            pub redirect_url: String,
        }
    }
}
