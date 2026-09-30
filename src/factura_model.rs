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
    #[serde(rename = "infoFactura")]
    pub info_factura: InfoFactura,
    pub detalles: Detalles,
    #[serde(rename = "infoAdicional")]
    pub info_adicional: InfoAdicional,
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

#[derive(Deserialize)]
pub struct DetAdicionales {
    #[serde(rename = "@nombre")]
    pub nombre: String,
    #[serde(rename = "@valor")]
    pub valor: String,
}

#[derive(Deserialize)]
pub struct DetallesAdicionales {
    #[serde(rename = "detAdicional", default)]
    pub det_adicional: Vec<DetAdicionales>,
}

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

#[derive(Deserialize)]
pub struct InfoAdicional {
    #[serde(rename = "campoAdicional")]
    pub campo_adicional: Vec<CampoAdicional>,
}

#[derive(Deserialize)]
pub struct CampoAdicional {
    #[serde(rename = "@nombre")]
    pub nombre: String,
    #[serde(rename = "$text")]
    pub text: String,
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
