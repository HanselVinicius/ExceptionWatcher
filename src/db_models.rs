use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[warn(dead_code)]
#[derive(Serialize, FromRow,Debug,Deserialize)]
pub struct Exception {
    pub id:Uuid,
    pub signature: String,
    pub application:String,
    pub project_class:String,
    pub project_method:String,
    pub created_at:DateTime<Utc>,
    pub updated_at:DateTime<Utc>
}

#[derive(Debug, Deserialize)]
pub struct CreateException{
    pub signature: String,
    pub application:String,
    pub project_class:String,
    pub project_method:String,
}

impl Exception{
    pub fn new(signature_p:String, application_p:String, project_class:String, project_method:String,) ->Self{
        let now = chrono::Utc::now();
        Self{
            id:Uuid::new_v4(),
            signature:signature_p,
            application:application_p,
            project_class,
            project_method,
            created_at:now,
            updated_at:now
        }
    }
}