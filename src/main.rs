mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use tera::Tera;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::new("src/templates/**/*").expect("Ошибка парсинга шаблонов");
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Инициализируем БД
    let db_pool = db::init_db().expect("Не удалось создать БД");
    let db_data = web::Data::new(db_pool);

    println!("Сервер запущен: http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/", web::get().to(handlers::index))
            .route("/shorten", web::post().to(handlers::shorten))
            .route("/{code}", web::get().to(handlers::redirect))
            .default_service(web::route().to(handlers::not_found))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}