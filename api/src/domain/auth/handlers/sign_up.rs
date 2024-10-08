use ::entity::entities::user;
use bcrypt::{hash, DEFAULT_COST};
use common::dto::auth::SignUpRequestDto;
use common::infrastructure::error::ApiError;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::{json, Value};

pub async fn handler(dto: SignUpRequestDto, db: DatabaseConnection) -> Result<Value, ApiError> {
    let exist = user::Entity::find()
        .filter(user::Column::Email.eq(&dto.email))
        .one(&db)
        .await?;

    if exist.is_some() {
        return Err(ApiError::ConflictError);
    }

    let hashed_password = hash(dto.password, DEFAULT_COST)?;

    let user = user::ActiveModel {
        email: ActiveValue::Set(dto.email),
        password: ActiveValue::Set(hashed_password),
        ..Default::default()
    };

    let res = user.insert(&db).await?;

    Ok(json!(res))
}
