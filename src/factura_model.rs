use crate::formateador_fecha::parsear_fecha;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
#[serde(rename = "factura")]
pub struct Factura {
    #[serde(rename = "infoTributaria")]
    pub info_tributaria: InfoTributaria,
}

#[derive(Deserialize)]
pub struct InfoTributaria {
    pub ambiente: u8,
    #[serde(rename = "tipoEmision")]
    pub tipo_emision: u8,
    #[serde(rename = "razonSocial")]
    pub razon_social: String,
    #[serde(rename = "nombreComercial")]
    pub nombre_comercial: String,
    pub ruc: String,
    #[serde(rename = "claveAcceso")]
    pub clave_acceso: String,
    #[serde(rename = "codDoc")]
    pub codigo_documento: String,
    #[serde(rename = "estab")]
    pub establecimiento: String,
    #[serde(rename = "ptoEmi")]
    pub punto_emision: String,
    pub secuencial: String,
}

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
    #[serde(rename = "totalSinImpuestos")]
    pub total_sin_impuestos: Decimal,
    #[serde(rename = "totalDescuento")]
    pub total_descuento: Decimal,
    #[serde(rename = "totalConImpuestos")]
    pub total_con_impuestos: Decimal,
    pub propina: Decimal,
    #[serde(rename = "importeTotal")]
    pub importe_total: Decimal,
    pub moneda: String,
}
#[derive(Deserialize)]
pub struct TotalImpuesto{
    pub codigo: u8,
    #[serde(rename = "codigoPorcentaje")]
    pub codigo_porcentaje: String,
    #[serde(rename = "descuentoAdicional", default)]
    pub descuento_adicional: Option<Decimal>,
    #[serde(rename = "baseImponible")]
    pub base_imponible: Decimal,
    pub valor: Decimal,
}



impl Display for InfoTributaria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "nombre_comercial: {}, ruc: {}, tipo_emision: {}",
            self.nombre_comercial, self.ruc, self.tipo_emision
        )
    }
}

pub struct Facturas {
    facturas: Vec<Factura>,
}

impl IntoIterator for Facturas {
    type Item = Factura;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.facturas.into_iter()
    }
}
