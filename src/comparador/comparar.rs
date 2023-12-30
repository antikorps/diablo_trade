use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Informe {
    pub info: Vec<Resultado>,
}
#[derive(Serialize)]
pub struct Resultado {
    pub original: Value,
    pub similares: Vec<Value>,
}

fn crear_sql_in_string(coleccion: &Vec<&str>) -> String {
    let mut parametro_in = String::new();

    for (indice, elemento) in coleccion.iter().enumerate() {
        let elemento_entrecomillado_comas = format!("\"{elemento}\", ");
        if indice == coleccion.len() - 1 {
            let sin_comas = elemento_entrecomillado_comas.trim_end_matches(", ");
            parametro_in.push_str(&sin_comas);
        } else {
            parametro_in.push_str(&elemento_entrecomillado_comas);
        }
    }
    return format!("({parametro_in})");
}

fn crear_sql_in_agrupamiento_armas(tipo: &str) -> String {
    let una_mano = vec!["axe", "dagger", "mace", "scythe", "sword", "wand"];

    if una_mano.contains(&tipo) {
        return crear_sql_in_string(&una_mano);
    }

    let dos_manos = vec![
        "polearm",
        "staff",
        "twohandedaxe",
        "twohandedmace",
        "twohandedscythe",
        "twohandedsword",
    ];
    if dos_manos.contains(&tipo) {
        return crear_sql_in_string(&dos_manos);
    }

    let distancia = vec!["bow", "crossbow"];
    if distancia.contains(&tipo) {
        return crear_sql_in_string(&distancia);
    }

    let tipo_individual = vec![tipo];
    return crear_sql_in_string(&tipo_individual);
}

pub fn buscar_objetos_parecidos(
    base_datos: &Connection,
    registro: &Value,
    rango: f64,
) -> Result<Resultado, ()> {
    let mut afijos_id = Vec::new();
    let mut afijos_valores = Vec::new();

    if !registro["itemType"].is_string() {
        eprintln!("Error de integridad de datos, itemType no es una string",);
        return Err(());
    }

    let tipo = registro["itemType"].as_str().unwrap();
    let sql_in_agrupamiento_tipo = crear_sql_in_agrupamiento_armas(tipo);

    if !registro["affixes"].is_array() {
        eprintln!(
            "Error de integridad de datos, affixes no es un array {}",
            registro.to_string()
        );
        return Err(());
    };

    for afijo in registro["affixes"].as_array().unwrap() {
        if !afijo["id"].is_string() {
            eprintln!(
                "Error de integridad de datos, affixe[id] no es string {}",
                registro.to_string()
            );
            return Err(());
        }
        let id = afijo["id"].as_str().unwrap();
        let valor = afijo["value"].as_f64().unwrap();

        afijos_id.push(id);
        afijos_valores.push(valor);
    }
    if afijos_id.len() != 4 {
        eprintln!("Se descarta el registro por no tener 4 afijos");
        return Err(());
    }
    let parametro_in = crear_sql_in_string(&afijos_id);

    let id_0 = afijos_id[0];
    let id_1 = afijos_id[1];
    let id_2 = afijos_id[2];
    let id_3 = afijos_id[3];

    let max_0 = afijos_valores[0] + (rango / 100.0) * afijos_valores[0];
    let min_0 = afijos_valores[0] - (rango / 100.0) * afijos_valores[0];

    let max_1 = afijos_valores[1] + (rango / 100.0) * afijos_valores[1];
    let min_1 = afijos_valores[1] - (rango / 100.0) * afijos_valores[1];

    let max_2 = afijos_valores[2] + (rango / 100.0) * afijos_valores[2];
    let min_2 = afijos_valores[2] - (rango / 100.0) * afijos_valores[2];

    let min_3 = afijos_valores[3] - (rango / 100.0) * afijos_valores[3];
    let max_3 = afijos_valores[3] + (rango / 100.0) * afijos_valores[3];

    let consulta_coincidencias_afijos = format!(
        r#"
    
        SELECT registro
        FROM registros
        WHERE json_extract(registro, '$.itemType') IN {sql_in_agrupamiento_tipo}
        AND
        (
            CASE WHEN json_extract(registro, '$.affixes[0].id') IN {parametro_in} THEN 1 ELSE 0 END +
            CASE WHEN json_extract(registro, '$.affixes[1].id') IN {parametro_in} THEN 1 ELSE 0 END +
            CASE WHEN json_extract(registro, '$.affixes[2].id') IN {parametro_in} THEN 1 ELSE 0 END +
            CASE WHEN json_extract(registro, '$.affixes[3].id') IN {parametro_in} THEN 1 ELSE 0 END
        ) >= 3
        AND (
            (
                CASE WHEN (
                    (json_extract(registro, '$.affixes[0].id') = "{id_0}" AND json_extract(registro, '$.affixes[0].value') BETWEEN {min_0} AND {max_0}) OR
                    (json_extract(registro, '$.affixes[0].id') = "{id_1}" AND json_extract(registro, '$.affixes[0].value') BETWEEN {min_1} AND {max_1}) OR
                    (json_extract(registro, '$.affixes[0].id') = "{id_2}" AND json_extract(registro, '$.affixes[0].value') BETWEEN {min_2} AND {max_2}) OR
                    (json_extract(registro, '$.affixes[0].id') = "{id_3}" AND json_extract(registro, '$.affixes[0].value') BETWEEN {min_3} AND {max_3})
                ) THEN 1 ELSE 0 END
            ) +
            (
                CASE WHEN (
                    (json_extract(registro, '$.affixes[1].id') = "{id_0}" AND json_extract(registro, '$.affixes[1].value') BETWEEN {min_0} AND {max_0}) OR
                    (json_extract(registro, '$.affixes[1].id') = "{id_1}" AND json_extract(registro, '$.affixes[1].value') BETWEEN {min_1} AND {max_1}) OR
                    (json_extract(registro, '$.affixes[1].id') = "{id_2}" AND json_extract(registro, '$.affixes[1].value') BETWEEN {min_2} AND {max_2}) OR
                    (json_extract(registro, '$.affixes[1].id') = "{id_3}" AND json_extract(registro, '$.affixes[1].value') BETWEEN {min_3} AND {max_3})
                ) THEN 1 ELSE 0 END
            ) +
            (
                CASE WHEN (
                    (json_extract(registro, '$.affixes[2].id') = "{id_0}" AND json_extract(registro, '$.affixes[2].value') BETWEEN {min_0} AND {max_0}) OR
                    (json_extract(registro, '$.affixes[2].id') = "{id_1}" AND json_extract(registro, '$.affixes[2].value') BETWEEN {min_1} AND {max_1}) OR
                    (json_extract(registro, '$.affixes[2].id') = "{id_2}" AND json_extract(registro, '$.affixes[2].value') BETWEEN {min_2} AND {max_2}) OR
                    (json_extract(registro, '$.affixes[2].id') = "{id_3}" AND json_extract(registro, '$.affixes[2].value') BETWEEN {min_3} AND {max_3})
                ) THEN 1 ELSE 0 END
            ) +
            (
                CASE WHEN (
                    (json_extract(registro, '$.affixes[3].id') = "{id_0}" AND json_extract(registro, '$.affixes[3].value') BETWEEN {min_0} AND {max_0}) OR
                    (json_extract(registro, '$.affixes[3].id') = "{id_1}" AND json_extract(registro, '$.affixes[3].value') BETWEEN {min_1} AND {max_1}) OR
                    (json_extract(registro, '$.affixes[3].id') = "{id_2}" AND json_extract(registro, '$.affixes[3].value') BETWEEN {min_2} AND {max_2}) OR
                    (json_extract(registro, '$.affixes[3].id') = "{id_3}" AND json_extract(registro, '$.affixes[3].value') BETWEEN {min_3} AND {max_3})
                ) THEN 1 ELSE 0 END
            )
        ) >= 3
        
        ORDER BY json_extract(registro, '$.price') DESC;

    "#
    );

    let mut sentencia = base_datos
        .prepare(&consulta_coincidencias_afijos)
        .expect("Error preparando la sentencia");

    let filas = sentencia
        .query_map([], |fila| {
            let registro: String = fila.get(0)?;
            return Ok(registro);
        })
        .unwrap();

    let mut similares = Vec::new();
    filas.for_each(|f| {
        if f.is_err() {
            eprintln!(
                "error recuperando recuperando una fila {}",
                f.err().unwrap()
            );
            return;
        }
        let v = f.unwrap();
        let serializar: Result<Value, serde_json::Error> = serde_json::from_str(&v);
        match serializar {
            Err(error) => {
                eprintln!("error serializando registro {v}: {error}");
                return;
            }
            Ok(ok) => similares.push(ok),
        }
    });

    return Ok(Resultado {
        original: registro.clone(),
        similares,
    });
}
