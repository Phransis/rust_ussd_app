use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn ussd_handler(req: HttpRequest) -> impl Responder {
    // Extract USSD parameters from the query string
    let query_params: Vec<&str> = req.query_string().split('&').collect();
    let mut ussd_code = "";
    let mut user_input = "";
    let mut session_id = "";

    for param in query_params {
        let kv: Vec<&str> = param.split('=').collect();
        match kv[0] {
            "ussd_code" => ussd_code = kv[1],
            "user_input" => user_input = kv[1],
            "session_id" => session_id = kv[1],
            _ => (),
        }
    }

    // Implement business logic based on USSD parameters
    let response = match user_input {
        "1" => "Welcome to USSD App! Choose an option:\n1. Balance Inquiry\n2. Account Info",
        "1*1" => "Your account balance is $1000",
        "1*2" => "Your account info: Account Number: XXXX-XXXX-XXXX, Account Type: Savings",
        _ => "Invalid USSD code. Please try again.",
    };

    // Construct USSD response
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(response)
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("This API is working, Yay!!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = "127.0.0.1:8080";
    println!("Running on port {}", port);
    HttpServer::new(|| {
        App::new()
            .route("/test", web::get().to(index))
            .route("/ussd", web::get().to(ussd_handler))
    })
    .bind(port)?
    .run()
    .await
}
