use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::models::tag::{Tag, TagCreate, TagListResponse, TagUpdate};

#[derive(Subcommand)]
pub enum TagCmd {
    #[command(about = "List all tags")]
    List {
        #[arg(long, help = "Filter by parent tag ID")]
        parent: Option<String>,
        #[arg(long, help = "Maximum number of tags to return (max 1000)")]
        max_count: Option<u32>,
        #[arg(long, help = "Number of tags to skip for pagination")]
        offset: Option<u32>,
        #[arg(long, help = "Include soft-deleted tags")]
        include_removed: bool,
    },
    #[command(about = "Get a single tag by ID")]
    Get {
        #[arg(help = "Tag ID")]
        id: String,
    },
    #[command(about = "Create a new tag")]
    Create {
        #[arg(long, help = "Tag title (required)")]
        title: String,
        #[arg(long, help = "Parent tag ID for nesting")]
        parent: Option<String>,
        #[arg(long, help = "Display order")]
        order: Option<f64>,
    },
    #[command(about = "Update an existing tag (only specified fields are changed)")]
    Update {
        #[arg(help = "Tag ID to update")]
        id: String,
        #[arg(long, help = "New tag title")]
        title: Option<String>,
        #[arg(long, help = "New parent tag ID")]
        parent: Option<String>,
        #[arg(long, help = "New display order")]
        order: Option<f64>,
    },
    #[command(about = "Delete a tag by ID (soft-delete)")]
    Delete {
        #[arg(help = "Tag ID to delete")]
        id: String,
    },
}

pub fn run(client: &ApiClient, cmd: TagCmd, json: bool) -> Result<()> {
    match cmd {
        TagCmd::List {
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
                let resp: serde_json::Value = client.get("/v2/tag", &query)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let resp: TagListResponse = client.get("/v2/tag", &query)?;
                if resp.tags.is_empty() {
                    println!("No tags found.");
                } else {
                    for t in &resp.tags {
                        println!("{}\n", t.display_list_item());
                    }
                }
            }
        }
        TagCmd::Get { id } => {
            if json {
                let resp: serde_json::Value = client.get(&format!("/v2/tag/{}", id), &[])?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let tag: Tag = client.get(&format!("/v2/tag/{}", id), &[])?;
                println!("{}", tag.display_detail());
            }
        }
        TagCmd::Create {
            title,
            parent,
            order,
        } => {
            let data = TagCreate {
                title,
                parent,
                order,
            };
            if json {
                let resp: serde_json::Value = client.post("/v2/tag", &data)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let tag: Tag = client.post("/v2/tag", &data)?;
                println!("Created tag {}", tag.id);
            }
        }
        TagCmd::Update {
            id,
            title,
            parent,
            order,
        } => {
            let data = TagUpdate {
                title,
                parent,
                order,
            };
            if json {
                let resp: serde_json::Value = client.patch(&format!("/v2/tag/{}", id), &data)?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            } else {
                let tag: Tag = client.patch(&format!("/v2/tag/{}", id), &data)?;
                println!("Updated tag {}", tag.id);
            }
        }
        TagCmd::Delete { id } => {
            client.delete(&format!("/v2/tag/{}", id))?;
            println!("Deleted tag {}", id);
        }
    }
    Ok(())
}
