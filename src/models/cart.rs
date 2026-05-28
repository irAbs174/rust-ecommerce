use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Cart {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CartItem {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price_at_time: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCartItemRequest {
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartResponse {
    pub id: Uuid,
    pub items: Vec<CartItemResponse>,
    pub total: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price_at_time: Decimal,
    pub subtotal: Decimal,
}
