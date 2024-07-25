use actix_web::{post, get, web, HttpResponse, Responder};
use crate::service::services::convert_date_format;
use crate::model::models::SalesOrders;


//root service
#[get("/")]
async fn root() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to the Azure Function!")
}

//Post rounte to change Date Format in Sales Orders file from yyyy-mm-dd to yyyymmdd
#[post("/api/date_formated")]
pub async fn date_formated(sales_order: web::Json<SalesOrders>) -> impl Responder {
    let mut sales_order = sales_order.into_inner();

    match convert_date_format(&mut sales_order) {
        Ok(_) => HttpResponse::Ok().json(sales_order),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}