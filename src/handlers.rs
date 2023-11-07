use axum::{extract, Json};
use axum::http::StatusCode;
use crate::db_models::{CreateException, Exception};
use sqlx::PgPool;

pub async fn health() ->StatusCode{
    StatusCode::OK
}


pub async fn insert_exception(
    extract::State(pool):extract::State<PgPool>,
    Json(payload): Json<CreateException>
) -> Result<(StatusCode, Json<Exception>), StatusCode>  {


    let exception = Exception::new(payload.signature, payload.application, payload.project_class, payload.project_method);

    let res = sqlx::query(
        r#"
        INSERT INTO exception_tb (id, signature, application, created_at, updated_at,project_class,project_method)
        VALUES ($1, $2, $3, $4, $5,$6,$7)
        "#,
    ).bind(&exception.id)
        .bind(&exception.signature)
        .bind(&exception.application)
        .bind(&exception.created_at)
        .bind(&exception.updated_at)
        .bind(&exception.project_class)
        .bind(&exception.project_method)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(exception))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}

pub async fn get_all(extract::State(pool):extract::State<PgPool>,) -> Result<Json<Vec<Exception>>,StatusCode>{
    let res = sqlx::query_as::<_,Exception>("SELECT * FROM exception_tb")
        .fetch_all(&pool)
        .await;
    match res {
        Ok(exceptions) => Ok(Json(exceptions)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


pub async fn delete_exception(
    extract::State(pool):extract::State<PgPool>,
    extract::Path(id): extract::Path<uuid::Uuid>
) -> StatusCode {
    let res = sqlx::query(
        r#"
            DELETE FROM exception_tb
            WHERE id = $1"#
    )
        .bind(id)
        .execute(&pool)
        .await
        .map(|res|match res.rows_affected() {
            0=> StatusCode::NOT_FOUND,
            _=> StatusCode::OK
        });

    match res {
        Ok(status) => status,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }

}





