use anyhow::Result;
use clap::{Subcommand, ValueEnum};

use crate::client::ApiClient;
use crate::models::task::{Task, TaskCreate, TaskListResponse, TaskUpdate};

#[derive(Clone, ValueEnum)]
pub enum Priority {
    High,
    Normal,
    Low,
}

impl Priority {
    fn as_i32(&self) -> i32 {
        match self {
            Priority::High => 0,
            Priority::Normal => 1,
            Priority::Low => 2,
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum CheckedStatus {
    Empty,
    Checked,
    Cancelled,
}

impl CheckedStatus {
    fn as_i32(&self) -> i32 {
        match self {
            CheckedStatus::Empty => 0,
            CheckedStatus::Checked => 1,
            CheckedStatus::Cancelled => 2,
        }
    }
}

#[derive(Subcommand)]
pub enum TaskCmd {
    #[command(about = "List tasks with optional filters")]
    List {
        #[arg(long, help = "Filter by project ID (P-<uuid>)")]
        project_id: Option<String>,
        #[arg(long, help = "Filter by parent task ID (T-<uuid>)")]
        parent: Option<String>,
        #[arg(
            long,
            help = "Filter tasks starting on or after this date (ISO 8601, inclusive)"
        )]
        start_from: Option<String>,
        #[arg(
            long,
            help = "Filter tasks starting on or before this date (ISO 8601, inclusive)"
        )]
        start_to: Option<String>,
        #[arg(long, help = "Maximum number of tasks to return (max 1000)")]
        max_count: Option<u32>,
        #[arg(long, help = "Number of tasks to skip for pagination")]
        offset: Option<u32>,
        #[arg(long, help = "Include soft-deleted tasks")]
        include_removed: bool,
        #[arg(long, help = "Include archived tasks")]
        include_archived: bool,
    },
    #[command(about = "Get a single task by ID")]
    Get {
        #[arg(help = "Task ID (T-<uuid> format)")]
        id: String,
    },
    #[command(about = "Create a new task")]
    Create {
        #[arg(long, help = "Task title (required)")]
        title: String,
        #[arg(long, help = "Task description/notes")]
        note: Option<String>,
        #[arg(long, value_enum, help = "Task priority: high, normal, or low")]
        priority: Option<Priority>,
        #[arg(long, help = "Assign to project (P-<uuid>)")]
        project_id: Option<String>,
        #[arg(long, help = "Parent task ID for subtasks (T-<uuid>)")]
        parent: Option<String>,
        #[arg(long, help = "Task group ID (Q-<uuid>)")]
        group: Option<String>,
        #[arg(long, help = "Deadline date (ISO 8601)")]
        deadline: Option<String>,
        #[arg(long, help = "Start date (ISO 8601)")]
        start: Option<String>,
        #[arg(long, value_delimiter = ',', help = "Comma-separated tag IDs")]
        tags: Option<Vec<String>>,
    },
    #[command(about = "Update an existing task (only specified fields are changed)")]
    Update {
        #[arg(help = "Task ID to update (T-<uuid> format)")]
        id: String,
        #[arg(long, help = "New task title")]
        title: Option<String>,
        #[arg(long, help = "New task description/notes")]
        note: Option<String>,
        #[arg(long, value_enum, help = "New priority: high, normal, or low")]
        priority: Option<Priority>,
        #[arg(
            long,
            value_enum,
            help = "Completion status: empty, checked, or cancelled"
        )]
        checked: Option<CheckedStatus>,
        #[arg(long, help = "Move to project (P-<uuid>)")]
        project_id: Option<String>,
        #[arg(long, help = "New parent task ID (T-<uuid>)")]
        parent: Option<String>,
        #[arg(long, help = "New task group ID (Q-<uuid>)")]
        group: Option<String>,
        #[arg(long, help = "New deadline date (ISO 8601)")]
        deadline: Option<String>,
        #[arg(long, help = "New start date (ISO 8601)")]
        start: Option<String>,
        #[arg(
            long,
            value_delimiter = ',',
            help = "Replace tags with comma-separated tag IDs"
        )]
        tags: Option<Vec<String>>,
    },
    #[command(about = "Delete a task by ID (soft-delete)")]
    Delete {
        #[arg(help = "Task ID to delete (T-<uuid> format)")]
        id: String,
    },
}

pub fn run(client: &ApiClient, cmd: TaskCmd, json: bool) -> Result<()> {
    match cmd {
        TaskCmd::List {
            project_id,
            parent,
            start_from,
            start_to,
            max_count,
            offset,
            include_removed,
            include_archived,
        } => {
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(ref v) = project_id {
                query.push(("projectId", v.to_string()));
            }
            if let Some(ref v) = parent {
                query.push(("parent", v.to_string()));
            }
            if let Some(ref v) = start_from {
                query.push(("startDateFrom", v.to_string()));
            }
            if let Some(ref v) = start_to {
                query.push(("startDateTo", v.to_string()));
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
            if include_archived {
                query.push(("includeArchived", "true".to_string()));
            }

            if json {
                let resp: serde_json::Value = client.get("/v2/task", &query)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let resp: TaskListResponse = client.get("/v2/task", &query)?;
                if resp.tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    for t in &resp.tasks {
                        println!("{}\n", t.display_list_item());
                    }
                }
            }
        }
        TaskCmd::Get { id } => {
            if json {
                let resp: serde_json::Value = client.get(&format!("/v2/task/{}", id), &[])?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let task: Task = client.get(&format!("/v2/task/{}", id), &[])?;
                println!("{}", task.display_detail());
            }
        }
        TaskCmd::Create {
            title,
            note,
            priority,
            project_id,
            parent,
            group,
            deadline,
            start,
            tags,
        } => {
            let data = TaskCreate {
                title,
                note,
                priority: priority.map(|p| p.as_i32()),
                project_id,
                parent,
                group,
                deadline,
                start,
                tags,
                is_note: None,
            };
            if json {
                let resp: serde_json::Value = client.post("/v2/task", &data)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let task: Task = client.post("/v2/task", &data)?;
                println!("Created task {}", task.id);
            }
        }
        TaskCmd::Update {
            id,
            title,
            note,
            priority,
            checked,
            project_id,
            parent,
            group,
            deadline,
            start,
            tags,
        } => {
            let data = TaskUpdate {
                title,
                note,
                priority: priority.map(|p| p.as_i32()),
                checked: checked.map(|c| c.as_i32()),
                project_id,
                parent,
                group,
                deadline,
                start,
                tags,
                is_note: None,
            };
            if json {
                let resp: serde_json::Value = client.patch(&format!("/v2/task/{}", id), &data)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let task: Task = client.patch(&format!("/v2/task/{}", id), &data)?;
                println!("Updated task {}", task.id);
            }
        }
        TaskCmd::Delete { id } => {
            client.delete(&format!("/v2/task/{}", id))?;
            println!("Deleted task {}", id);
        }
    }
    Ok(())
}
