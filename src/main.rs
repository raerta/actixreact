extern crate actix_web;
use actix_web::{ middleware, web, App, HttpServer, Result };
use std::io;
use actix_files as fs;
use std::path::PathBuf;

async fn single_page_app() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("./frontend/build/index.html");
    Ok(fs::NamedFile::open(path)?)
}

#[actix_web::main] // Actix Web için async runtime'ı başlatır
async fn main() -> io::Result<()> {
    let port = 8080;
    let address = format!("127.0.0.1:{}", port);

    println!("Server running at http://{}", address);

    // Sunucuyu başlat ve ayarla
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(single_page_app))
            .route("/user", web::get().to(single_page_app))
            .service(fs::Files::new("/", "./frontend/build").index_file("index.html"))
    })
        .bind(&address)
        ? // Burada .bind için hata kontrolü yapılıyor
        .run().await // Sunucuyu başlat // Asenkron çalışmayı bekler
}
