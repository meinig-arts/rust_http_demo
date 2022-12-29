use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web::{ HttpResponse, get};
use actix_files as fs;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
  HttpServer::new( || {
    App::new()
    .route("/", web::get().to(greet))
    .route("/{name}", web::get().to(greet))
    .service(htmlgreet)
    .service(fs::Files::new("/static/files", ".").show_files_listing())
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}

async fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", name)  // no semicolon here
}

#[get("/sus/sus")]
async fn htmlgreet(_req: HttpRequest) -> impl Responder {
  HttpResponse::Ok()
  .content_type("text/html; charset=utf-8")
  .body("<h1>Hallo SUS!</h1>")
}