use crate::models::{ItemCreate, ItemDb, ItemResponse, ListItemsQuery};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use rustapi::{get, post, APIRouter, HTTPException, Response, ValidatedJson};

#[get("/hello")]
pub async fn hello() -> &'static str {
    "Hello from RustAPI!"
}

#[get("")]
pub async fn list_items(
    State(state): State<AppState>,
    rustapi::Query(query): rustapi::Query<ListItemsQuery>,
) -> rustapi::Json<Vec<ItemResponse>> {
    let items = state.items.lock().unwrap();
    let limit = query.limit.unwrap_or(10) as usize;

    let mut results: Vec<ItemResponse> = items
        .iter()
        .filter(|item| {
            if let Some(ref q) = query.q {
                item.name.contains(q)
            } else {
                true
            }
        })
        .map(|item| ItemResponse {
            id: item.id,
            name: item.name.clone(),
            price: item.price,
        })
        .collect();

    results.truncate(limit);
    rustapi::Json(results)
}

#[get("/{id}")]
pub async fn get_item(
    State(state): State<AppState>,
    rustapi::Path(id): rustapi::Path<i32>,
) -> Result<rustapi::Json<ItemResponse>, HTTPException> {
    let items = state.items.lock().unwrap();
    let item = items.iter().find(|i| i.id == id).ok_or_else(|| {
        HTTPException::new(StatusCode::NOT_FOUND, format!("Item {} not found", id))
    })?;

    Ok(rustapi::Json(ItemResponse {
        id: item.id,
        name: item.name.clone(),
        price: item.price,
    }))
}

#[post("")]
pub async fn create_item(
    State(state): State<AppState>,
    ValidatedJson(body): ValidatedJson<ItemCreate>,
) -> Response<rustapi::Json<ItemResponse>> {
    let mut items = state.items.lock().unwrap();
    let next_id = items.len() as i32 + 1;

    let db_item = ItemDb {
        id: next_id,
        name: body.name.clone(),
        price: body.price,
        internal_secret: std::env::var("INTERNAL_SECRET")
            .unwrap_or_else(|_| "development_secret".to_string()),
    };

    items.push(db_item.clone());

    Response::new(
        StatusCode::CREATED,
        rustapi::Json(ItemResponse {
            id: db_item.id,
            name: db_item.name,
            price: db_item.price,
        }),
    )
}

#[get("/error")]
pub async fn trigger_error() -> Result<&'static str, HTTPException> {
    Err(HTTPException::new(
        StatusCode::BAD_REQUEST,
        "This is a deliberate error",
    ))
}


#[get("/request-info")]
pub async fn request_info(req: rustapi::Request) -> String {
    format!("Request method: {}, Path: {}", req.method(), req.uri().path())
}

pub fn items_router() -> APIRouter<AppState> {
    APIRouter::new()
        .prefix("/items")
        .tag("items")
        .route(list_items_route())
        .route(get_item_route())
        .route(create_item_route())
        .route(request_info_route().with_state::<AppState>())
}
