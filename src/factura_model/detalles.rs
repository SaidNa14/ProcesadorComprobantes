use rust_decimal::Decimal;
use serde::Deserialize;

// --- Detalles ---

#[derive(Deserialize)]
pub struct Detalles {
    pub detalle: Vec<Detalle>,
}

#[derive(Deserialize)]
pub struct Detalle {
    #[serde(rename = "codigoPrincipal")]
    pub codigo_principal: String,
    #[serde(rename = "codigoAuxiliar")]
    pub codigo_auxiliar: String,
    pub descripcion: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub cantidad: Decimal,
    #[serde(rename = "precioUnitario", with = "rust_decimal::serde::str")]
    pub precio_unitario: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub descuento: Decimal,
    #[serde(rename = "precioTotalSinImpuesto", with = "rust_decimal::serde::str")]
    pub precio_total_sin_impuestos: Decimal,
    #[serde(rename = "detallesAdicionales", default)]
    pub detalles_adicionales: Option<DetallesAdicionales>,
    pub impuestos: Impuestos,
}

// --- Detalles Adicionales ---

#[derive(Deserialize)]
pub struct DetallesAdicionales {
    #[serde(rename = "detAdicional", default)]
    pub det_adicional: Vec<DetAdicional>,
}

#[derive(Deserialize)]
pub struct DetAdicional {
    #[serde(rename = "@nombre")]
    pub nombre: String,
    #[serde(rename = "@valor")]
    pub valor: String,
}

// --- Impuestos ---

#[derive(Deserialize)]
pub struct Impuestos {
    pub impuesto: Vec<Impuesto>,
}

#[derive(Deserialize)]
pub struct Impuesto {
    pub codigo: String,
    #[serde(rename = "codigoPorcentaje")]
    pub codigo_porcentaje: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub tarifa: Decimal,
    #[serde(rename = "baseImponible", with = "rust_decimal::serde::str")]
    pub base_imponible: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub valor: Decimal,
}
