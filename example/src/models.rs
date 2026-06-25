use rustapi::model;

// Database Model (Internal)
#[model]
pub struct ItemDb {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub internal_secret: String,
}

// API Request Model (What the user sends)
#[model]
pub struct ItemCreate {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
}

// API Response Model (What the user gets)
#[model]
pub struct ItemResponse {
    pub id: i32,
    pub name: String,
    pub price: f64,
}

// Query Parameters
#[model]
pub struct ListItemsQuery {
    pub q: Option<String>,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i32>,
}
