use crate::Configuracion;
use reqwest::blocking::Client;
use rusqlite::Connection;

mod comparar;
mod informe_html;
mod recuperar;

pub fn comparador(cliente: Client, base_datos: Connection, configuracion: Configuracion) -> String {
    println!("===> comienza el proceso de recuperación de objetos del usuario");
    let registros_recuperados = recuperar::recuperar(&cliente, &configuracion);
    println!(
        "===> comienza el proceso de comparación de los {} objetos recuperados del usuario",
        &registros_recuperados.len()
    );
    let mut resultados = Vec::new();
    for registro in registros_recuperados {
        let resultado = comparar::buscar_objetos_parecidos(
            &base_datos,
            &registro,
            configuracion.rango_comparacion,
        );
        if resultado.is_err() {
            continue;
        }
        resultados.push(resultado.unwrap());
    }

    let informe = comparar::Informe { info: resultados };
    let informe_json = serde_json::to_string(&informe)
        .expect("error en la serialización del informe de resultados tras la comparacion");

    return informe_html::generar_html(informe_json);
}
