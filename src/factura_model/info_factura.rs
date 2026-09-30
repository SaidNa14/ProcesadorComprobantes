use crate::formateador_fecha::parsear_fecha;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct InfoFactura {
    #[serde(rename = "fechaEmision", deserialize_with = "parsear_fecha")]
    pub fecha_emision: NaiveDate,
    #[serde(rename = "dirEstablecimiento")]
    pub direccion_establecimiento: String,
    #[serde(rename = "contribuyenteEspecial")]
    pub contribuyente_especial: String,
    #[serde(rename = "obligadoContabilidad")]
    pub obligado_contabilidad: String,
    #[serde(rename = "tipoIdentificacionComprador")]
    pub tipo_identificacion_comprador: String,
    #[serde(rename = "guiaRemision", default)]
    pub guia_remision: Option<String>,
    #[serde(rename = "razonSocialComprador")]
    pub razon_social_comprador: String,
    #[serde(rename = "identificacionComprador")]
    pub identificacion_comprador: String,
    #[serde(rename = "direccionComprador")]
    pub direccion_comprador: String,
    #[serde(rename = "totalSinImpuestos", with = "rust_decimal::serde::str")]
    pub total_sin_impuestos: Decimal,
    #[serde(rename = "totalDescuento", with = "rust_decimal::serde::str")]
    pub total_descuento: Decimal,
    #[serde(rename = "totalConImpuestos")]
    pub total_con_impuestos: TotalConImpuestos,
    #[serde(with = "rust_decimal::serde::str")]
    pub propina: Decimal,
    #[serde(rename = "importeTotal", with = "rust_decimal::serde::str")]
    pub importe_total: Decimal,
    #[serde(default)]
    pub moneda: Option<String>,
    pub pagos: Pagos,
}

impl Display for InfoFactura {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dirección Establecimiento: {}, Total Descuento: {}",
            self.direccion_establecimiento, self.total_descuento,
        )
    }
}

// --- TotalConImpuestos ---

#[derive(Deserialize)]
pub struct TotalConImpuestos {
    #[serde(rename = "totalImpuesto")]
    pub total_impuesto: Vec<TotalImpuesto>,
}

#[derive(Deserialize)]
pub struct TotalImpuesto {
    pub codigo: String,
    #[serde(rename = "codigoPorcentaje")]
    pub codigo_porcentaje: String,
    #[serde(
        rename = "descuentoAdicional",
        default,
        with = "rust_decimal::serde::str_option"
    )]
    pub descuento_adicional: Option<Decimal>,
    #[serde(rename = "baseImponible", with = "rust_decimal::serde::str")]
    pub base_imponible: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub valor: Decimal,
}

// --- Pagos ---

#[derive(Deserialize)]
pub struct Pagos {
    pub pago: Vec<Pago>,
}

#[derive(Deserialize)]
pub struct Pago {
    #[serde(rename = "formaPago")]
    pub forma_pago: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub total: Decimal,
    pub plazo: String,
    #[serde(rename = "unidadTiempo")]
    pub unidad_tiempo: String,
}
