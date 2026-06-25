use rustapi::model;

#[model]
pub struct Item {
    pub name: String,
    pub price: f64,
}
