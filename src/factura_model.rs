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
    #[serde(rename = "nombreComercial")]
    pub nombre_comercial: String,
    pub ruc: String,
    #[serde(rename = "tipoEmision")]
    pub tipo_emision: String,
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
