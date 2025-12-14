use actix_web::{web, HttpResponse, http::StatusCode};
use tera::Context;
use rand::Rng;
use crate::{db, models::ShortenRequest, TEMPLATES};
use crate::db::DbPool;

/// Генерирует случайный код
fn generate_code() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Главная страница с формой
pub async fn index() -> HttpResponse {
    let context = Context::new();
    match TEMPLATES.render("index.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Ошибка шаблона: {}", e)),
    }
}

/// Создание короткой ссылки
pub async fn shorten(
    db: web::Data<DbPool>,
    form: web::Form<ShortenRequest>,
) -> HttpResponse {
    let conn = db.lock().unwrap();
    
    // Генерируем уникальный код
    let mut code = generate_code();
    while db::code_exists(&conn, &code).unwrap_or(false) {
        code = generate_code();
    }
    
    // Сохраняем в БД
    match db::insert_url(&conn, &code, &form.url) {
        Ok(_) => {
            let mut context = Context::new();
            context.insert("short_url", &format!("http://127.0.0.1:8080/{}", code));
            context.insert("original_url", &form.url);
            context.insert("code", &code);
            
            match TEMPLATES.render("success.html", &context) {
                Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
                Err(e) => HttpResponse::InternalServerError().body(format!("Ошибка: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Ошибка БД: {}", e)),
    }
}

/// Редирект по короткой ссылке
pub async fn redirect(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let code = path.into_inner();
    let conn = db.lock().unwrap();
    
    match db::get_url_by_code(&conn, &code) {
        Ok(Some(url)) => HttpResponse::PermanentRedirect()
            .append_header(("Location", url))
            .finish(),
        Ok(None) => render_404(),
        Err(_) => render_404(),
    }
}

/// Страница 404
pub async fn not_found() -> HttpResponse {
    render_404()
}

fn render_404() -> HttpResponse {
    let context = Context::new();
    match TEMPLATES.render("404.html", &context) {
        Ok(body) => HttpResponse::build(StatusCode::NOT_FOUND)
            .content_type("text/html")
            .body(body),
        Err(_) => HttpResponse::NotFound().body("404 - Страница не найдена"),
    }
}