use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::models::task_group::{
    TaskGroup, TaskGroupCreate, TaskGroupListResponse, TaskGroupUpdate,
};

#[derive(Subcommand)]
pub enum TaskGroupCmd {
    #[command(about = "List task groups, optionally filtered by parent project")]
    List {
        #[arg(long, help = "Filter by parent project ID (P-<uuid>)")]
        parent: Option<String>,
        #[arg(long, help = "Maximum number of groups to return (max 1000)")]
        max_count: Option<u32>,
        #[arg(long, help = "Number of groups to skip for pagination")]
        offset: Option<u32>,
        #[arg(long, help = "Include soft-deleted groups")]
        include_removed: bool,
    },
    #[command(about = "Get a single task group by ID")]
    Get {
        #[arg(help = "Task group ID (Q-<uuid> format)")]
        id: String,
    },
    #[command(about = "Create a new task group inside a project")]
    Create {
        #[arg(long, help = "Group title (required)")]
        title: String,
        #[arg(long, help = "Parent project ID (P-<uuid>, required)")]
        parent: String,
        #[arg(long, help = "Sort order within parent project")]
        order: Option<f64>,
    },
    #[command(about = "Update an existing task group (only specified fields are changed)")]
    Update {
        #[arg(help = "Task group ID to update (Q-<uuid> format)")]
        id: String,
        #[arg(long, help = "New group title")]
        title: Option<String>,
        #[arg(long, help = "Move to a different project (P-<uuid>)")]
        parent: Option<String>,
        #[arg(long, help = "New sort order within parent")]
        order: Option<f64>,
    },
    #[command(about = "Delete a task group by ID (soft-delete)")]
    Delete {
        #[arg(help = "Task group ID to delete (Q-<uuid> format)")]
        id: String,
    },
}

pub fn run(client: &ApiClient, cmd: TaskGroupCmd, json: bool) -> Result<()> {
    match cmd {
        TaskGroupCmd::List {
            parent,
            max_count,
            offset,
            include_removed,
        } => {
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(ref v) = parent {
                query.push(("parent", v.to_string()));
            }
            if let Some(v) = max_count {
                query.push(("maxCount", v.to_string()));
            }
            if let Some(v) = offset {
                query.push(("offset", v.to_string()));
            }
            if include_removed {
                query.push(("includeRemoved", "true".to_string()));
            }

            if json {
                let resp: serde_json::Value = client.get("/v2/task-group", &query)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let resp: TaskGroupListResponse = client.get("/v2/task-group", &query)?;
                if resp.task_groups.is_empty() {
                    println!("No task groups found.");
                } else {
                    for g in &resp.task_groups {
                        println!("{}\n", g.display_list_item());
                    }
                }
            }
        }
        TaskGroupCmd::Get { id } => {
            if json {
                let resp: serde_json::Value = client.get(&format!("/v2/task-group/{}", id), &[])?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let group: TaskGroup = client.get(&format!("/v2/task-group/{}", id), &[])?;
                println!("{}", group.display_detail());
            }
        }
        TaskGroupCmd::Create {
            title,
            parent,
            order,
        } => {
            let data = TaskGroupCreate {
                title,
                parent,
                parent_order: order,
                fake: None,
            };
            if json {
                let resp: serde_json::Value = client.post("/v2/task-group", &data)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let group: TaskGroup = client.post("/v2/task-group", &data)?;
                println!("Created task group {}", group.id);
            }
        }
        TaskGroupCmd::Update {
            id,
            title,
            parent,
            order,
        } => {
            let data = TaskGroupUpdate {
                title,
                parent,
                parent_order: order,
                fake: None,
            };
            if json {
                let resp: serde_json::Value =
                    client.patch(&format!("/v2/task-group/{}", id), &data)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let group: TaskGroup = client.patch(&format!("/v2/task-group/{}", id), &data)?;
                println!("Updated task group {}", group.id);
            }
        }
        TaskGroupCmd::Delete { id } => {
            client.delete(&format!("/v2/task-group/{}", id))?;
            println!("Deleted task group {}", id);
        }
    }
    Ok(())
}
