pub mod checkout {
    pub mod request {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CheckoutRequest {
            pub data: Data,
        }

    
        impl CheckoutRequest {
            pub fn new( line_items: Vec<LineItem>, payment_method_types: Vec<String>) -> CheckoutRequest {
                CheckoutRequest {
                    data: Data {
                        attributes: Attributes {
                            send_email_receipt: false,
                            show_description: false,
                            show_line_items: true,
                            line_items,
                            payment_method_types,
                            description: "Pass".to_string(),
                        },
                    },
                }
            }
        }


    
        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct Data {
            pub attributes: Attributes,
        }
    
        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct Attributes {
            pub send_email_receipt: bool,
    
            pub show_description: bool,
    
            pub show_line_items: bool,
    
            pub line_items: Vec<LineItem>,
    
            pub payment_method_types: Vec<String>,
    
            pub description: String,
        }
    
        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct LineItem {
            pub currency: String,
    
            pub amount: i32,
    
            pub name: String,
    
            pub quantity: i32,
    
            pub description: String,
        }


        impl LineItem {
            pub fn new(currency: &str, amount: i32, name: &str, quantity: i32, description: &str) -> LineItem {
                LineItem {
                    currency: currency.to_string(),
                    amount,
                    name: name.to_string(),
                    quantity,
                    description: description.to_string(),
                }
            }
        }

    }
    
    pub mod response {
        use serde::{Deserialize, Serialize};

    
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CheckoutResult {
            pub data: CheckoutData,
        }
    
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CheckoutData {
            pub id: String,
    
            #[serde(rename = "type")]
            pub data_type: String,
    
            pub attributes: Attributes,
        }
    
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Attributes {
            pub checkout_url: String,
        }
    }
    
}