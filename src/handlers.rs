use axum::{extract, http, Json};
use axum::http::StatusCode;
use crate::db_models::{CreateException, Exception};
use sqlx::PgPool;

pub async fn health() ->http::StatusCode{
    http::StatusCode::OK
}


pub async fn insert_exception(
    extract::State(pool):extract::State<PgPool>,
    Json(payload): axum::Json<CreateException>
) -> Result<(StatusCode, Json<Exception>), StatusCode>  {


    let exception = Exception::new(payload.signature,payload.application);

    let res = sqlx::query(
        r#"
        INSERT INTO exception_tb (id, signature, application, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    ).bind(&exception.id)
        .bind(&exception.signature)
        .bind(&exception.application)
        .bind(&exception.created_at)
        .bind(&exception.updated_at)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(exception))),
        Err(_)=> Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}





