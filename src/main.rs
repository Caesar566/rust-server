use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/*
first example
*/
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok()
//        .body("Hello, world!")
// }

// #[post("/echo")]
// async fn echo() -> impl Responder {
//     HttpResponse::Ok()
//       .body("123")
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok()
//      .body("Hey there")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//            .service(hello)
//            .service(echo)
//            //.service(manual_hello)
//            .route("/hey", web::get().to(manual_hello))
//     })
//    .bind("127.0.0.1:8080")?
//    .run()
//    .await
// }


/*
By app example
*/

// async fn index() -> impl Responder {
//     "Hello  world!"
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()>{
//     HttpServer::new(|| {
//         App::new()
//           .service(
//             web::scope("/app")
//                 .route("/index.html", web::get().to(index)),
//           )
//     })
//     .bind(("127.0.0.1",8080))?
//     .run()
//     .await
// }