use serde::Serialize;

#[derive(Serialize)]
#[serde(rename = "soapenv:Envelope")]
pub struct Envelope {
    #[serde(rename = "@xmlns:soapenv")]
    pub xmlns_env: String,
    #[serde(rename = "@xmlns:ec")]
    pub xmlns_ec: String,
    #[serde(rename = "soapenv:Body")]
    pub soapenv_body: SoapEnvBody,
}

#[derive(Serialize)]
pub struct SoapEnvBody {
    #[serde(rename = "ec:autorizacionComprobante")]
    pub autorizacion_comprobante: AutorizacionComprobante,
}
#[derive(Serialize)]
pub struct AutorizacionComprobante {
    #[serde(rename = "claveAccesoComprobante")]
    pub clave_acceso_comprobante: String,
}
