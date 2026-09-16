use chrono::NaiveDate;
use serde::Deserialize;
/* Ok, segun lo que entendi, en la primera parte de la funcion defino el lifetime con 'de
* pero tambien le digo que le voy a pasar un generico como en <T>
* Al final de lo que devuelve con el Result, probablemente me toque implementar el tipo error a ese
* generico o usar alguno que ya existe*/
pub fn parsear_fecha<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let texto = String::deserialize(deserializer)?;
    let formatted_text = NaiveDate::parse_from_str(&texto, "%d/%m/%Y")
        .map_err(|chrono_err| serde::de::Error::custom(chrono_err.to_string()));
    formatted_text
}
