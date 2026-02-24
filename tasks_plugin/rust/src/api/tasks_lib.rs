use flutter_rust_bridge::frb;

#[frb(ignore)]
mod database {
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{Database, DatabaseConnection};
    use std::sync::OnceLock;

    static DATABASE_URL: OnceLock<String> = OnceLock::new();

    pub(super) async fn set_db_path(path: String) {
        let url = format!("sqlite://{}?mode=rwc", path);
        DATABASE_URL.set(url.clone()).ok();
        let conn = Database::connect(url)
            .await
            .expect("error while connecting to database");
        let _ = Migrator::up(&conn, None).await;
    }

    pub(super) async fn get_conn() -> DatabaseConnection {
        let url = DATABASE_URL
            .get()
            .expect("Database not initialized. Call init_app first.");
        Database::connect(url)
            .await
            .expect("error while connecting to database")
    }

    pub(super) mod entities {
        pub mod tasks {
            use crate::api::tasks_lib::domain::models::Task;
            use sea_orm::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "tasks")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
                pub title: String,
                pub description: String,
                pub completed: bool,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}

            impl From<Model> for Task {
                fn from(model: Model) -> Self {
                    Task {
                        id: model.id,
                        title: model.title,
                        description: model.description,
                        completed: model.completed,
                    }
                }
            }
        }
    }

    pub(super) mod models {
        pub mod tasks {
            use super::super::entities::tasks::Entity as TasksEntity;
            use sea_orm::DerivePartialModel;
            use serde::Serialize;

            #[derive(Debug, Serialize, PartialEq, Eq, DerivePartialModel)]
            #[sea_orm(entity = "TasksEntity", from_query_result)]
            pub struct Tasks {
                #[sea_orm(primary_key)]
                pub id: i32,
                pub title: String,
                pub description: String,
                pub completed: bool,
            }
        }
    }
}

#[frb(ignore)]
pub(crate) mod repository {

    pub mod notes {
        use crate::api::tasks_lib::database::entities::tasks;
        use crate::api::tasks_lib::database::{
            entities::tasks::{ActiveModel as TaskAm, Entity as TasksEntity, Model as TaskModel},
            get_conn, set_db_path,
        };
        use crate::api::tasks_lib::domain::models::Task;
        use sea_orm::{
            ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
        };

        pub async fn init(db_path: String) {
            set_db_path(db_path).await;
        }

        pub async fn get_all_tasks() -> Result<Vec<TaskModel>, anyhow::Error> {
            let conn = get_conn().await;
            let tasks = TasksEntity::find().all(&conn).await?;
            Ok(tasks)
        }

        pub async fn create_task(task: Task) -> Result<(), anyhow::Error> {
            let conn = get_conn().await;
            let active: TaskAm = task.into();
            active.insert(&conn).await?;
            Ok(())
        }

        pub async fn update_task(task: Task) -> Result<(), anyhow::Error> {
            let conn = get_conn().await;
            let active: TaskAm = task.into();
            active.update(&conn).await?;
            Ok(())
        }

        pub async fn delete_task(task_id: i32) -> Result<(), anyhow::Error> {
            let conn = get_conn().await;
            let Some(task_found) = TasksEntity::find()
                .filter(tasks::Column::Id.eq(task_id))
                .one(&conn)
                .await?
            else {
                return Ok(());
            };
            task_found.delete(&conn).await?;
            Ok(())
        }

        impl From<Task> for TaskAm {
            fn from(task: Task) -> Self {
                TaskAm {
                    id: Set(task.id),
                    title: Set(task.title),
                    description: Set(task.description),
                    completed: Set(task.completed),
                }
            }
        }
    }
}

pub mod domain {
    pub mod models {
        use flutter_rust_bridge::frb;
        use serde::{Deserialize, Serialize};

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
    }
}
