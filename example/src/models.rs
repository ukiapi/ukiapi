use ukiapi::model;
use ukiapi::JsonSchema;

#[model]
pub struct ItemDb {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub internal_secret: String,
}

#[model]
pub struct ItemCreate {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
}

#[model]
pub struct ItemResponse {
    pub id: i32,
    pub name: String,
    pub price: f64,
}

#[model]
pub struct ListItemsQuery {
    pub q: Option<String>,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i32>,
}

#[model]
pub struct LoginRequest {
    pub username: String,
}

#[model]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Clone, ukiapi::Serialize, ukiapi::Deserialize, JsonSchema)]
pub struct UserClaims {
    pub sub: String,
    pub exp: u64,
}
