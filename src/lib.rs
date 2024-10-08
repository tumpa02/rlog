use actix_files::Files;
use actix_web::{dev::Server, get, middleware, web, App, HttpResponse, HttpServer};
use std::fs;
use std::net::TcpListener;
use tera::Tera;

pub mod utils;
pub mod handlers;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[get("/robots.txt")]
async fn robots_txt() -> HttpResponse {
    match fs::read_to_string("./static/robots.txt") {
        Ok(contents) => HttpResponse::Ok().content_type("text/plain").body(contents),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("sitemap.txt")]
async fn sitemap_txt() -> HttpResponse {
    match fs::read_to_string("./static/sitemap.txt") {
        Ok(contents) => HttpResponse::Ok().content_type("text/plain").body(contents),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn start_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static/").use_last_modified(true))
            .service(handlers::index)
            .service(handlers::about)
            .service(handlers::blog)
            .service(handlers::blog_post)
            .service(handlers::contact)
            .service(robots_txt)
            .service(sitemap_txt)
            .default_service(web::route().to(handlers::not_found))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
