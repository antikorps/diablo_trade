use std::{env::current_exe, fs::File, io::Write};

use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;

mod comparador;
mod scraper;

/// Diablo Trader scraper y comparador de objetos
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Argumentos {
    /// Ejecutar el programa en modo scraper para actualizar la base de datos con el histórico de productos
    #[arg(short, long)]
    scraper: bool,
}
#[derive(Deserialize, Clone)]
pub struct Configuracion {
    pub usuario: String,
    pub max_base_datos: i64,
    pub rango_comparacion: f64,
}

fn main() {
    let argumentos = Argumentos::parse();

    let ruta_ejecutable = current_exe().expect("no se ha podido recuperar la ruta del ejecutable");
    let ruta_raiz = ruta_ejecutable
        .parent()
        .expect("no se ha podido recuperar el directorio del ejecutable");
    let ruta_base_datos = ruta_raiz.join("diablo_trade.sqlite3");
    let ruta_configuracion = ruta_raiz.join("configuracion.json");
    let archivo_configuracion = File::open(ruta_configuracion)
        .expect("no se ha podido abrir el archivo configuracion.json");

    let base_datos = rusqlite::Connection::open(ruta_base_datos)
        .expect("ha fallado la conexión con la base de datos sqlite3");
    let configuracion: Configuracion = serde_json::from_reader(archivo_configuracion)
        .expect("no se ha podido deserializar el json de configuracion");

    let mut cabeceras = reqwest::header::HeaderMap::new();
    cabeceras.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/119.0",
        ),
    );
    cabeceras.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let cliente = Client::builder()
        .default_headers(cabeceras)
        .build()
        .expect("no se ha podido crear el cliente http");

    if argumentos.scraper {
        println!("===> comienza el scraping del histórico de diablo trade...");
        scraper::scraper(cliente, base_datos, configuracion);
        return;
    }

    let informe = comparador::comparador(cliente, base_datos, configuracion);
    let ruta_informe = ruta_raiz.join("index.html");
    let mut archivo_informe = File::create(ruta_informe)
        .expect("no se ha podido crear el archivo con el informe de resultados");
    archivo_informe
        .write(informe.as_bytes())
        .expect("error escribiendo el informe con los resultados");
    println!(
        "===> archivo de resultados index.html creado correctamente en {}",
        ruta_raiz.display()
    )
}
