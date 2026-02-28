use anyhow::Result;
use chrono_tz::Tz;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::models::project::{Project, ProjectCreate, ProjectListResponse, ProjectUpdate};

#[derive(Subcommand)]
pub enum ProjectCmd {
    #[command(about = "List all projects")]
    List {
        #[arg(long, help = "Maximum number of projects to return (max 1000)")]
        max_count: Option<u32>,
        #[arg(long, help = "Number of projects to skip for pagination")]
        offset: Option<u32>,
        #[arg(long, help = "Include soft-deleted projects")]
        include_removed: bool,
        #[arg(long, help = "Include archived projects")]
        include_archived: bool,
    },
    #[command(about = "Get a single project by ID")]
    Get {
        #[arg(help = "Project ID (P-<uuid> format)")]
        id: String,
    },
    #[command(about = "Create a new project")]
    Create {
        #[arg(long, help = "Project title (required)")]
        title: String,
        #[arg(long, help = "Project description/notes")]
        note: Option<String>,
        #[arg(long, help = "Parent project ID (P-<uuid>) for nesting")]
        parent: Option<String>,
        #[arg(long, help = "Color hex code (e.g. #FF0000)")]
        color: Option<String>,
        #[arg(long, help = "Emoji icon for the project")]
        emoji: Option<String>,
        #[arg(long, help = "Start date (ISO 8601 format)")]
        start: Option<String>,
        #[arg(long, help = "End date (ISO 8601 format)")]
        end: Option<String>,
        #[arg(long, help = "Create as a notebook instead of a project")]
        notebook: bool,
    },
    #[command(about = "Update an existing project (only specified fields are changed)")]
    Update {
        #[arg(help = "Project ID to update (P-<uuid> format)")]
        id: String,
        #[arg(long, help = "New project title")]
        title: Option<String>,
        #[arg(long, help = "New project description/notes")]
        note: Option<String>,
        #[arg(long, help = "New parent project ID (P-<uuid>)")]
        parent: Option<String>,
        #[arg(long, help = "New color hex code")]
        color: Option<String>,
        #[arg(long, help = "New emoji icon")]
        emoji: Option<String>,
        #[arg(long, help = "New start date (ISO 8601)")]
        start: Option<String>,
        #[arg(long, help = "New end date (ISO 8601)")]
        end: Option<String>,
        #[arg(long, help = "Set notebook flag (true/false)")]
        notebook: Option<bool>,
    },
    #[command(about = "Delete a project by ID (soft-delete)")]
    Delete {
        #[arg(help = "Project ID to delete (P-<uuid> format)")]
        id: String,
    },
}

pub fn run(client: &ApiClient, cmd: ProjectCmd, json: bool, tz: Option<Tz>) -> Result<()> {
    match cmd {
        ProjectCmd::List {
            max_count,
            offset,
            include_removed,
            include_archived,
        } => {
            let mut query: Vec<(&str, String)> = Vec::new();
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
                let resp: serde_json::Value = client.get("/v2/project", &query)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let resp: ProjectListResponse = client.get("/v2/project", &query)?;
                if resp.projects.is_empty() {
                    println!("No projects found.");
                } else {
                    for p in &resp.projects {
                        println!("{}\n", p.display_list_item());
                    }
                }
            }
        }
        ProjectCmd::Get { id } => {
            if json {
                let resp: serde_json::Value = client.get(&format!("/v2/project/{}", id), &[])?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let project: Project = client.get(&format!("/v2/project/{}", id), &[])?;
                println!("{}", project.display_detail(tz));
            }
        }
        ProjectCmd::Create {
            title,
            note,
            parent,
            color,
            emoji,
            start,
            end,
            notebook,
        } => {
            let data = ProjectCreate {
                title,
                note,
                parent,
                color,
                emoji,
                start,
                end,
                is_notebook: if notebook { Some(true) } else { None },
            };
            if json {
                let resp: serde_json::Value = client.post("/v2/project", &data)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let project: Project = client.post("/v2/project", &data)?;
                println!("Created project {}", project.id);
            }
        }
        ProjectCmd::Update {
            id,
            title,
            note,
            parent,
            color,
            emoji,
            start,
            end,
            notebook,
        } => {
            let data = ProjectUpdate {
                title,
                note,
                parent,
                color,
                emoji,
                start,
                end,
                is_notebook: notebook,
            };
            if json {
                let resp: serde_json::Value =
                    client.patch(&format!("/v2/project/{}", id), &data)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let project: Project = client.patch(&format!("/v2/project/{}", id), &data)?;
                println!("Updated project {}", project.id);
            }
        }
        ProjectCmd::Delete { id } => {
            client.delete(&format!("/v2/project/{}", id))?;
            println!("Deleted project {}", id);
        }
    }
    Ok(())
}
