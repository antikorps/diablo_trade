use crate::Configuracion;
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use rusqlite::Connection;
use serde_json::Value;
use std::{str::FromStr, thread::sleep, time::Duration};
use url::Url;
/*

CREATE TABLE "registros" (
    "id"	INTEGER NOT NULL,
    "_id"	TEXT NOT NULL,
    "fecha"	INTEGER NOT NULL,
    "registro"	TEXT NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
)

*/

fn utc_a_unix(fecha_hora: &str) -> Result<i64, String> {
    let tiempo_unix;
    match DateTime::<Utc>::from_str(fecha_hora) {
        Err(error) => {
            return Err(error.to_string());
        }
        Ok(ok) => {
            tiempo_unix = ok;
        }
    }
    return Ok(tiempo_unix.timestamp());
}

fn recuperar_fecha_mas_actual(base_datos: &Connection) -> i64 {
    let consulta_sql_fecha_actual = "
    
    SELECT fecha FROM registros
        ORDER BY fecha DESC LIMIT 1

    ";

    return base_datos.query_row(consulta_sql_fecha_actual, [], |fila| {
        let fecha: i64 = fila.get(0)?;
        return Ok(fecha);
    }).expect("error fatal: no se ha podido recuperar la fecha más actual de los registros de la base de datos");
}

fn comprobar_existencia_tabla_registros(base_datos: &Connection) -> bool {
    let consulta_existencia_registros = r#"
    
    SELECT 
    COUNT(*)
    FROM 
        sqlite_schema
    WHERE 
        type ='table' AND 
        name == "registros"
    
    "#;

    let coincidencias = base_datos
        .query_row(consulta_existencia_registros, [], |fila| {
            let coincidencias: i64 = fila.get(0)?;
            return Ok(coincidencias);
        })
        .expect("error fatal: no se ha podido comprobar la existencia de la tabla registros");

    if coincidencias == 0 {
        println!("ATENCIÓN: la base de datos no tiene tabla registros, esto implica un scrapeado total del histórico que puede llevar horas");
        println!("se recomienda descargar el archivo diablo_trade.sqlite y colocarlo en el mismo directorio que el ejecutable");

        let consulta_create = r#"
        CREATE TABLE "registros" (
            "id"	INTEGER NOT NULL,
            "_id"	TEXT NOT NULL,
            "fecha"	INTEGER NOT NULL,
            "registro"	TEXT NOT NULL,
            PRIMARY KEY("id" AUTOINCREMENT)
        )
        "#;

        base_datos
            .execute(consulta_create, [])
            .expect("ha fallado la creación de la tabla registros");
        return false;
    }

    return true;
}

pub fn scraper(cliente: Client, base_datos: Connection, configuracion: Configuracion) {
    let existencia_tabla_registros = comprobar_existencia_tabla_registros(&base_datos);
    let mut fecha_mas_actual = 0;
    if existencia_tabla_registros {
        fecha_mas_actual = recuperar_fecha_mas_actual(&base_datos);
    };

    let mut cursor = 1;
    let mut actualizaciones_correctas = 0;
    let mut incorporaciones_correctas = 0;

    'bucle_peticiones: loop {
        let query = format!(
            r#"{{"0":{{"json":{{"mode":["season softcore"],"itemType":[],"class":[],"sockets":[],"category":[],"price":{{"min":0,"max":9999999999}},"powerLevel":[0,1000],"levelRequired":[0,100],"effectsGroup":[],"sort":{{"updatedAt":-1,"createdAt":-1}},"sold":true,"exactPrice":false,"cursor":{cursor},"limit":100}}}}}}"#
        );
        let url = Url::parse("https://diablo.trade/api/trpc/offer.search");
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
            let fecha_hora = &registro["updatedAt"].as_str();
            if fecha_hora.is_none() {
                eprintln!("registro {id} descartado por no tener fecha de actualización");
                continue;
            }
            let fecha_actualizacion_unix = utc_a_unix(fecha_hora.unwrap());
            if fecha_actualizacion_unix.is_err() {
                eprintln!("registro {id} descartado por no poder convertir a unix la fecha de actualizacion");
                continue;
            }
            let registro_unix = fecha_actualizacion_unix.unwrap();

            if registro_unix < fecha_mas_actual {
                break 'bucle_peticiones;
            }

            let mut id_existe = false;
            let consulta_existencia = base_datos.query_row(
                "SELECT COUNT(*) FROM registros WHERE id = ?",
                [id],
                |fila| {
                    Ok({
                        let recuperar_fila: i64 = fila.get(0)?;
                        if recuperar_fila == 1 {
                            id_existe = true
                        }
                    })
                },
            );
            if consulta_existencia.is_err() {
                eprintln!(
                    "el registro {id} ha fallado al intentar consultar la existencia: {}",
                    consulta_existencia.err().unwrap()
                );
                continue;
            }

            let registro_texto = registro.to_string();
            if id_existe {
                match base_datos.execute(
                    "UPDATE registros
                SET registro = ?, fecha = ? WHERE _id = ?;",
                    (id, registro_unix, &registro_texto),
                ) {
                    Err(error) => {
                        eprintln!("no se ha podido actualizar el registro correspondiendo al id {id} por {error}")
                    }
                    Ok(_) => {
                        actualizaciones_correctas += 1;
                    }
                }
            } else {
                match base_datos.execute(
                    "INSERT INTO registros (_id, fecha, registro) VALUES (?, ?, ?)",
                    (id, registro_unix, &registro_texto),
                ) {
                    Err(error) => {
                        eprintln!("no se ha podido insertar el registro correspondiendo al id {id} por {error}")
                    }
                    Ok(_) => {
                        incorporaciones_correctas += 1;
                    }
                }
            }
        }

        let siguiente = &respuesta_json[0]["result"]["data"]["json"]["nextCount"];
        if !siguiente.is_i64() {
            break 'bucle_peticiones;
        }
        cursor = siguiente.as_i64().unwrap();

        sleep(Duration::from_secs(3));
    }

    println!(
        "Tras el scrapeado, se han realizado las siguientes operaciones en la base de datos:
--> {incorporaciones_correctas} nuevas incorporaciones
--> {actualizaciones_correctas} actualizaciones
"
    );

    let consulta_sql_numero_registros = "
        SELECT COUNT(*) FROM registros
    ";

    let resultado_total = base_datos.query_row(consulta_sql_numero_registros, [], |f| {
        let total: i64 = f.get(0)?;
        return Ok(total);
    });
    if resultado_total.is_err() {
        eprintln!(
            "no se ha podido obtener el total de registros de la base de datos {}",
            resultado_total.err().unwrap()
        );
        return;
    }
    let total = resultado_total.unwrap();
    if total < configuracion.max_base_datos {
        return;
    }

    let numero_registros_borrar = total - configuracion.max_base_datos;
    let consulta_sql_borrado = format!(
        "
    
    DELETE FROM registros WHERE _id IN (
        SELECT _id FROM registros ORDER BY fecha ASC LIMIT {numero_registros_borrar}
    ); 

    "
    );

    match base_datos.execute(&consulta_sql_borrado, []) {
        Err(error) => {
            eprintln!("no se ha podido realizar el borrado de registros {error}");
            return;
        }
        Ok(_) => {
            println!("Actualización de base datos realizada correctamente. Se han borrado {numero_registros_borrar} registros viejos.")
        }
    }

    match base_datos.execute("VACUUM;", []) {
        Err(error) => {
            eprintln!("aunque se han eliminado {numero_registros_borrar} registros ha fallado el VACUUM de la base de datos, por lo que el tamaño podría no haberse reducido correctamente {error}")
        }
        Ok(_) => (),
    }
}
