use actix_cors::Cors;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use log::{debug, info, warn};
use prisma_client_rust::NewClientError;
use serde::{Deserialize, Serialize};

mod constants;
mod middleware;
mod prisma;
mod task;
mod user;
mod utils;
use prisma::PrismaClient;
use serde_json::json;
use task::api::*;
use task::model::CreateTask;


#[get("/api/check")]
async fn hello() -> impl Responder {
    debug!("Hello world request made");
    HttpResponse::Ok().json(json!({
        "message":"API service is running successfully",
        "status":"UP"
    }))
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrorMessage {
    message: String,
    error: Option<String>,
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();
    info!("Server starting on port 5000");

    warn!("Connecting to prisma client");
    let prisma_service_builder: Result<PrismaClient, NewClientError> =
        PrismaClient::_builder().build().await;
    let prisma_service = prisma_service_builder.expect("Could not connect to prisma client");
    let client = web::Data::new(prisma_service);
    info!("Prisma client connected successfully");

    // let prisma_client = client.clone();
    // //start the cron service on another thread
    // actix_rt::spawn(async move {

    //     utils::scheduler::start_test_cron(&prisma_client).await;
    // });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            // .allowed_headers(vec![AUTHORIZATION, ACCEPT])
            // .allowed_header(CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(client.clone())
            .wrap(cors)
            .wrap(crate::middleware::auth_middleware::Authentication)
            .service(hello)
            .service(get_raw_array)
            .service(delete_by_id)
            .service(add_task)
            .configure(user::user::routes_config)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
