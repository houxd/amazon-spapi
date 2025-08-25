use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod configuration;

pub mod aplus_content_2020_11_01;
pub mod app_integrations_2024_04_01;
pub mod application_2023_11_30;
pub mod awd_2024_05_09;
pub mod catalog_items_2020_12_01;
pub mod catalog_items_2022_04_01;
pub mod catalog_items_v0;
pub mod customer_feedback_2024_06_01;
pub mod data_kiosk_2023_11_15;
pub mod product_type_definitions_2020_09_01;
pub mod easy_ship_2022_03_23;
pub mod fba_inbound_eligibility_v1;
pub mod fba_inventory_v1;
pub mod feeds_2021_06_30;
pub mod finances_2024_06_19;
pub mod finances_v0;
pub mod fulfillment_inbound_2024_03_20;
pub mod fulfillment_inbound_v0;
pub mod fulfillment_outbound_2020_07_01;
pub mod invoices_api_model_2024_06_19;
pub mod listings_items_2020_09_01;
pub mod listings_items_2021_08_01;
pub mod listings_restrictions_2021_08_01;
pub mod merchant_fulfillment_v0;
pub mod messaging_v1;
pub mod notifications_v1;
pub mod orders_v0;
pub mod orders_v0_shipment;
pub mod product_fees_v0;
pub mod product_pricing_2022_05_01;
pub mod product_pricing_v0;
pub mod replenishment_2022_11_07;
pub mod reports_2021_06_30;
pub mod sales_v1;
pub mod seller_wallet_2024_03_01;
pub mod sellers_v1;
pub mod service_v1;
pub mod shipment_invoicing_v0;
pub mod shipping_v1;
pub mod shipping_v2;
pub mod solicitations_v1;
pub mod supply_sources_2020_07_01;
pub mod tokens_2021_03_01;
pub mod transfers_2024_06_01;
pub mod uploads_2020_11_01;
pub mod vehicles_2024_11_01;
pub mod vendor_direct_fulfillment_inventory_v1;
pub mod vendor_direct_fulfillment_orders_2021_12_28;
pub mod vendor_direct_fulfillment_orders_v1;
pub mod vendor_direct_fulfillment_payments_v1;
pub mod vendor_direct_fulfillment_sandbox_data_2021_10_28;
pub mod vendor_direct_fulfillment_shipping_2021_12_28;
pub mod vendor_direct_fulfillment_shipping_v1;
pub mod vendor_direct_fulfillment_transactions_2021_12_28;
pub mod vendor_direct_fulfillment_transactions_v1;
pub mod vendor_invoices_v1;
pub mod vendor_orders_v1;
pub mod vendor_shipments_v1;
pub mod vendor_transaction_status_v1;
