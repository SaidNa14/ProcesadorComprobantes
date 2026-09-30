pub mod detalles;
pub mod info_adicional;
pub mod info_factura;
pub mod info_tributaria;

pub use detalles::{DetAdicional, Detalle, DetallesAdicionales, Impuesto, Impuestos, Detalles};
pub use info_adicional::{CampoAdicional, InfoAdicional};
pub use info_factura::{InfoFactura, Pago, Pagos, TotalConImpuestos, TotalImpuesto};
pub use info_tributaria::InfoTributaria;

use serde::Deserialize;

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
