use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Cliente {
    pub id: i64,
    pub nombre: String,
    pub direccion: String,
    pub cuit_dni: String,
    pub telefono: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ClienteInput {
    pub nombre: String,
    pub direccion: String,
    pub cuit_dni: String,
    pub telefono: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Empresa {
    pub id: i64,
    pub nombre: String,
    pub direccion: String,
    pub cuit: String,
    pub telefono: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct EmpresaInput {
    pub nombre: String,
    pub direccion: String,
    pub cuit: String,
    pub telefono: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Producto {
    pub id: i64,
    pub nombre: String,
    pub precio_centavos: i64,
    pub stock: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductoInput {
    pub nombre: String,
    pub precio_centavos: i64,
    pub stock: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Remito {
    pub id: i64,
    pub cliente_id: i64,
    pub fecha: String,
    pub total_centavos: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RemitoItem {
    pub id: i64,
    pub remito_id: i64,
    pub producto_id: Option<i64>,
    pub nombre_producto: String,
    pub precio_unitario_centavos: i64,
    pub cantidad: i64,
    pub subtotal_centavos: i64,
}

#[derive(Debug, Deserialize)]
pub struct RemitoItemInput {
    pub producto_id: i64,
    pub cantidad: i64,
}

#[derive(Debug, Deserialize)]
pub struct RemitoInput {
    pub cliente_id: i64,
    pub items: Vec<RemitoItemInput>,
}

#[derive(Debug, Serialize)]
pub struct RemitoCompleto {
    pub remito: Remito,
    pub items: Vec<RemitoItem>,
}
