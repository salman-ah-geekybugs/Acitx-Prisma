use crate::prisma;
use crate::CreateTask;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::warn;
use prisma::PrismaClient;


#[get("/test")]
pub async fn get_raw_array(client: web::Data<PrismaClient>) -> impl Responder {
    let list = client.task().find_many(vec![]).exec().await.unwrap();
    HttpResponse::Ok().json(list)
}

#[delete("/test/{id}")]
pub async fn delete_by_id(client: web::Data<PrismaClient>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let res = client
        .task()
        .delete(prisma::task::id::equals(id))
        .exec()
        .await;
    match res {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            warn!("{:?}", e);

            HttpResponse::BadRequest().json(crate::ErrorMessage {
                message: "Query could not be executed".into(),
                error: Some(e.to_string()),
            })
        }
    }
}

#[post("/test")]
pub async fn add_task(
    client: web::Data<PrismaClient>,
    body: web::Json<CreateTask>,
) -> impl Responder {
    let created_task = client
        .task()
        .create(
            body.text.clone(),
            body.reminder,
            vec![prisma::task::SetParam::SetTimestamp(Some(
                body.timestamp.clone(),
            ))],
        )
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().json(created_task)
}
