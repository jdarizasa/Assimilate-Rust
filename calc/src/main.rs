// Web Microservice for calculating multiple operations
use actix_web::{App, HttpResponse, HttpServer, get, web};

#[get("/add/{a}/{b}")]
async fn add(a: web::Path<(f64, f64)>) -> HttpResponse {
    let (x, y) = a.into_inner();
    HttpResponse::Ok().json(calc::add(x, y))
}

#[get("/subtract/{a}/{b}")]
async fn subtract(a: web::Path<(f64, f64)>) -> HttpResponse {
    let (x, y) = a.into_inner();
    HttpResponse::Ok().json(calc::subtract(x, y))
}

#[get("/multiply/{a}/{b}")]
async fn multiply(a: web::Path<(f64, f64)>) -> HttpResponse {
    let (x, y) = a.into_inner();
    HttpResponse::Ok().json(calc::multiply(x, y))
}

#[get("/divide/{a}/{b}")]
async fn divide(a: web::Path<(f64, f64)>) -> HttpResponse {
    let (x, y) = a.into_inner();
    match calc::divide(x, y) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error) => HttpResponse::BadRequest().json(error),
    }
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
