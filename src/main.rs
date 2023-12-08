use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/info")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
    //    .content_type("text/html; charset=utf-8")
       .body("<h1>Hello, world!</h1>")
}

#[post("/echo")]
async fn post_info() -> impl Responder {
    HttpResponse::Ok()
    //   .content_type("text/html; charset=utf-8")
      .body("<h1>Hello, world!</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
           .service(index)
           .service(post_info)
    })
   .bind("0.0.0.0:9090")?
   .run()
   .await
}