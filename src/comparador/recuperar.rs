use crate::Configuracion;
use reqwest::blocking::Client;
use serde_json::Value;
use std::{thread::sleep, time::Duration};
use url::Url;

pub fn recuperar(cliente: &Client, configuracion: &Configuracion) -> Vec<Value> {
    let mut cursor = 1;

    let mut coleccion_objetos = Vec::new();

    'bucle_peticiones: loop {
        let query = format!(
            r#"{{"0":{{"json":{{"id":"{}","mode":["season softcore"],"itemType":[],"class":[],"sockets":[],"category":[],"price":{{"min":0,"max":9999999999}},"powerLevel":[0,1000],"levelRequired":[0,100],"effectsGroup":[],"sort":{{"updatedAt":-1,"createdAt":-1}},"sold":false,"exactPrice":false,"cursor":{},"limit":100}}}}}}"#,
            configuracion.usuario, cursor
        );

        let url = Url::parse("https://diablo.trade/api/trpc/offer.getAllFromUser");
        if url.is_err() {
            eprintln!("imposible parsear la url de diablo trade");
            break 'bucle_peticiones;
        }
        let respuesta = cliente
            .get(url.unwrap())
            .query(&[("batch", "1"), ("input", &query)])
            .send();
        if respuesta.is_err() {
            eprintln!("error en la respuesta a la api en el cursor {cursor}");
            break 'bucle_peticiones;
        }

        let res = respuesta.unwrap();

        if res.status() != 200 {
            eprintln!(
                "se detiene el scraper en el cursor {cursor} por status code incorrecto {}",
                res.status()
            );
            break 'bucle_peticiones;
        }

        let texto = res.text();
        if texto.is_err() {
            eprintln!("se detiene el scraper en el cursor {cursor} por error al leer el contenido de la respuesta {}", texto.err().unwrap());
            break 'bucle_peticiones;
        }

        let deserializacion_json: Result<Value, serde_json::Error> =
            serde_json::from_str(&texto.unwrap());
        if deserializacion_json.is_err() {
            eprintln!("se detiene el scraper en el cursor {cursor} por un error deserialización la respuesta {}", deserializacion_json.err().unwrap());
            break 'bucle_peticiones;
        }
        let respuesta_json = deserializacion_json.unwrap();

        let registros = &respuesta_json[0]["result"]["data"]["json"]["data"];
        if !registros.is_array() {
            eprintln!("se detiene el scraper en el cursor {cursor} porque el data no es un array");
            break 'bucle_peticiones;
        }

        for registro in registros.as_array().unwrap() {
            let registro_id = &registro["_id"].as_str();
            if registro_id.is_none() {
                continue;
            }
            let id = registro_id.unwrap();
            let afijos = &registro["affixes"].as_array();
            if afijos.is_none() {
                eprintln!("registro {id} descartado por no tener afijos");
                continue;
            }
            if afijos.unwrap().len() != 4 {
                eprintln!("registro {id} descartado por no tener 4 afijos");
                continue;
            }

            coleccion_objetos.push(registro.to_owned());
        }

        let siguiente = &respuesta_json[0]["result"]["data"]["json"]["nextCount"];
        if !siguiente.is_i64() {
            break 'bucle_peticiones;
        }
        cursor = siguiente.as_i64().unwrap();

        sleep(Duration::from_secs(3));
    }

    return coleccion_objetos;
}
