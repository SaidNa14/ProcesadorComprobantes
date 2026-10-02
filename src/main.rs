use crate::solicitud::{AutorizacionComprobante, Envelope};
use crate::{factura_model::Factura, solicitud::SoapEnvBody};
use anyhow::{Error, Result};
use reqwest::Client;
use reqwest::Response;
use std::{fs, path::Path};

pub mod factura_model;
pub mod formateador_fecha;
pub mod solicitud;

/*fn main() -> Result<(), Error> {
    fn cargar_facturas<P: AsRef<Path>>(ruta_dir: P) -> Result<Vec<Factura>, Error> {
        let mut coleccion = Vec::new();
        for entrada in fs::read_dir(ruta_dir)? {
            let entrada = entrada?;
            let ruta = entrada.path();

            if ruta.is_file() && ruta.extension().is_some_and(|v| v == "xml") {
                let contenido = fs::read_to_string(&ruta)?;
                let mut deserializer = quick_xml::de::Deserializer::from_str(&contenido);
                match serde_path_to_error::deserialize::<_, Factura>(&mut deserializer) {
                    Ok(factura) => coleccion.push(factura),
                    Err(e) => {
                        let ruta_campo = e.path().to_string();
                        let error_interno = e.into_inner();

                        println!(
                            "Error en {}\n campo: {}\n detalle: {}",
                            ruta.display(),
                            ruta_campo,
                            error_interno
                        )
                    }
                }
            }
        }
        Ok(coleccion)
    }
    let facturas: Vec<Factura> =
        cargar_facturas("/home/saidna14/Documents/ProcesadorComprobantes/src/facturas/")?;
    for factura in facturas {
        println!("{}", factura.info_factura)
    }
    Ok(())
} */
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let envelope = Envelope {
        xmlns_env: "http://schemas.xmlsoap.org/soap/envelope/".to_string(),
        xmlns_ec: "http://ec.gob.sri.ws.autorizacion".to_string(),
        soapenv_body: SoapEnvBody {
            autorizacion_comprobante: AutorizacionComprobante {
                clave_acceso_comprobante: "".to_string(),
            },
        },
    };
    let xml = quick_xml::se::to_string(&envelope)?;
    println!("{}", xml);

    let cliente = Client::new();

    let consulta = cliente
        .post("https://cel.sri.gob.ec/comprobantes-electronicos-ws/AutorizacionComprobantesOffline")
        .body(xml)
        .send()
        .await?;

    println!("Resultado {}", consulta.text().await?);
    Ok(())
}
