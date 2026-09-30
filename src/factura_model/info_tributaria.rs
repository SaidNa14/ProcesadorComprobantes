use serde::Deserialize;
use std::fmt::Display;

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

impl Display for InfoTributaria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "nombre_comercial: {}, ruc: {}, tipo_emision: {}",
            self.nombre_comercial, self.ruc, self.tipo_emision
        )
    }
}
