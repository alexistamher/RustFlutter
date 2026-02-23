use anyhow;
use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use serde_json;

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
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    #[frb(sync)]
    pub fn new(id: i32, title: String, description: String, completed: bool) -> Self {
        Self {
            id,
            title,
            description,
            completed,
        }
    }

    #[frb(name = "copyWith")]
    pub fn copy_with(&self, completed: bool) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            completed,
        }
    }

    #[frb(name = "toJson")]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub async fn get_all_tasks() -> anyhow::Result<Vec<Task>> {
    let response = reqwest::get("http://127.0.0.1:8080/tasks")
        .await?
        .json::<Vec<Task>>()
        .await?;
    Ok(response)
}

pub async fn delete_task(id: i32) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    client
        .delete(format!("http://127.0.0.1:8080/tasks/{id}"))
        .send()
        .await?;
    Ok(())
}

#[frb]
pub async fn create_task(
    id: i32,
    title: String,
    description: String,
    completed: bool,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    client
        .post(format!("http://127.0.0.1:8080/tasks"))
        .header("Content-Type", "application/json")
        .body(
            Task {
                id,
                title,
                description,
                completed,
            }
            .to_json(),
        )
        .send()
        .await?;
    Ok(())
}

#[frb]
pub async fn update_task(
    id: i32,
    title: String,
    description: String,
    completed: bool,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    client
        .put(format!("http://127.0.0.1:8080/tasks/{id}"))
        .body(
            Task {
                id,
                title,
                description,
                completed,
            }
            .to_json(),
        )
        .send()
        .await?;
    Ok(())
}
