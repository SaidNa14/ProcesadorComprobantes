use anyhow::{Error, Result};
use std::{fs, path::Path};

use crate::factura_model::Factura;

pub mod factura_model;
pub mod formateador_fecha;
fn main() -> Result<(), Error> {
    fn cargar_facturas<P: AsRef<Path>>(ruta_dir: P) -> Result<Vec<Factura>, Error> {
        let mut coleccion = Vec::new();
        for entrada in fs::read_dir(ruta_dir)? {
            let entrada = entrada?;
            let ruta = entrada.path();

            if ruta.is_file() && ruta.extension().is_some_and(|v| v == "xml") {
                let contenido = fs::read_to_string(&ruta)?;
                let deserializer = &mut quick_xml::de::Deserializer::from_str(&contenido);
                match serde_path_to_error::deserialize::<_, Factura>(deserializer) {
                    Ok(factura) => coleccion.push(factura),
                    Err(e) => {
                        let ruta_campo = e.path().to_string();
                        let error_interno = e.into_inner();
                        println!(
                            "Error en archivo {:?}\n  campo: {}\n  detalle: {}",
                            ruta.file_name(),
                            ruta_campo,
                            error_interno
                        );
                    }
                }
            }
        }
        Ok(coleccion)
    }
    let facturas: Vec<Factura> =
        cargar_facturas("/home/saidna14/Documents/ProcesadorComprobantes/src/facturas/")?;
    for factura in facturas {
        println!("{}", factura.info_tributaria)
    }
    Ok(())
}
