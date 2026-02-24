use super::tasks_lib::domain::models::Task;
use anyhow;
use flutter_rust_bridge::frb;

#[frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[frb]
pub async fn sync_db(db_path: String) -> anyhow::Result<()> {
    crate::api::tasks_lib::repository::notes::init(db_path).await;
    Ok(())
}

pub async fn get_all_tasks() -> anyhow::Result<Vec<Task>> {
    let tasks = crate::api::tasks_lib::repository::notes::get_all_tasks().await?;
    Ok(tasks.into_iter().map(Task::from).collect())
}

pub async fn delete_task(id: i32) -> anyhow::Result<()> {
    crate::api::tasks_lib::repository::notes::delete_task(id).await?;
    Ok(())
}

#[frb]
pub async fn create_task(
    id: i32,
    title: String,
    description: String,
    completed: bool,
) -> anyhow::Result<()> {
    let task = Task {
        id,
        title,
        description,
        completed,
    };
    crate::api::tasks_lib::repository::notes::create_task(task).await?;
    Ok(())
}

#[frb]
pub async fn update_task(
    id: i32,
    title: String,
    description: String,
    completed: bool,
) -> anyhow::Result<()> {
    let task = Task {
        id,
        title,
        description,
        completed,
    };
    crate::api::tasks_lib::repository::notes::update_task(task).await?;
    Ok(())
}
