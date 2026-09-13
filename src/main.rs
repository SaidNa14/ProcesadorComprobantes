use anyhow::{Error, Result};
use std::{fs, path::Path};

use crate::factura_model::Factura;

pub mod factura_model;
fn main() -> Result<(), Error> {
    fn cargar_facturas<P: AsRef<Path>>(ruta_dir: P) -> Result<Vec<Factura>, Error> {
        let mut coleccion = Vec::new();
        for entrada in fs::read_dir(ruta_dir)? {
            let entrada = entrada?;
            let ruta = entrada.path();

            if ruta.is_file() && ruta.extension().is_some_and(|v| v == "xml") {
                let contenido = fs::read_to_string(&ruta)?;
                if let Ok(factura) = quick_xml::de::from_str::<Factura>(&contenido) {
                    coleccion.push(factura);
                } else {
                    println!("Error al deserializar el archivo")
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
