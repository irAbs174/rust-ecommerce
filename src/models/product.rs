use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub stock: i32,
    pub category_id: Uuid,
    pub sku: String,
    pub image_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub stock: i32,
    pub category_id: Uuid,
    pub sku: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub category_id: Option<Uuid>,
    pub image_url: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub stock: i32,
    pub category_id: Uuid,
    pub sku: String,
    pub image_url: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListProductsQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub category_id: Option<Uuid>,
    pub search: Option<String>,
}
