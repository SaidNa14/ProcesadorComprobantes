use serde::Deserialize;

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
