use super::DeliveryProduct;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BatchDeliveryProduct {
    pub country: String,
    pub delivery_product: String,
}

impl BatchDeliveryProduct {
    pub fn new(country: impl Into<String>, delivery_product: DeliveryProduct) -> Self {
        Self {
            country: country.into(),
            delivery_product: delivery_product.as_str().to_string(),
        }
    }
}
