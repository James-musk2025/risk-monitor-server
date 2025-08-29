use ntex::web;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey there!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let server = web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    });
    
    let addr = ("127.0.0.1", 8080);
    println!("Server running on http://{}:{}", addr.0, addr.1);
    
    server.bind(addr)?
        .run()
        .await
}