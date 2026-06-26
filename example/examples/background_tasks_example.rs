use rustapi::{get, routes, serve, BackgroundTasks, Json};
use std::fs::OpenOptions;
use std::io::Write;

#[get("/background")]
async fn background_handler(
    tasks: BackgroundTasks,
) -> Json<serde_json::Value> {
    tasks.add_task(async {
        let mut file = OpenOptions::new().create(true).append(true).open("tasks.log").unwrap();
        writeln!(file, "Task 1 starting...").unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        writeln!(file, "Task 1 completed!").unwrap();
        file.flush().unwrap();
    });

    tasks.add_task(async {
        let mut file = OpenOptions::new().create(true).append(true).open("tasks.log").unwrap();
        writeln!(file, "Task 2 starting...").unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        writeln!(file, "Task 2 completed!").unwrap();
        file.flush().unwrap();
    });

    rustapi::json!({
        "message": "Background tasks scheduled"
    })
    .into()
}

#[tokio::main]
async fn main() {
    let state = ();

    let api = routes![(), background_handler_route()];

    serve(api.build_router(state)).await;
}
