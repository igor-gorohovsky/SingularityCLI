use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::models::task::format_date;

#[derive(Debug, Deserialize)]
pub struct ProjectListResponse {
    pub projects: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub note: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub emoji: Option<String>,
    pub color: Option<String>,
    pub parent: Option<String>,
    #[serde(rename = "parentOrder")]
    #[allow(dead_code)]
    pub parent_order: Option<f64>,
    #[serde(rename = "isNotebook")]
    pub is_notebook: Option<bool>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "modificatedDate")]
    #[allow(dead_code)]
    pub modificated_date: Option<String>,
}

impl Project {
    pub fn display_detail(&self, tz: Option<Tz>) -> String {
        let mut lines = vec![
            format!("**ID:** {}", self.id),
            format!("**Title:** {}", self.title),
        ];
        if let Some(ref v) = self.note {
            lines.push(format!("**Note:** {}", v));
        }
        if let Some(ref v) = self.parent {
            lines.push(format!("**Parent:** {}", v));
        }
        if let Some(ref v) = self.emoji {
            lines.push(format!("**Emoji:** {}", v));
        }
        if let Some(ref v) = self.color {
            lines.push(format!("**Color:** {}", v));
        }
        if let Some(ref v) = self.start {
            lines.push(format!("**Start:** {}", format_date(v, tz)));
        }
        if let Some(ref v) = self.end {
            lines.push(format!("**End:** {}", format_date(v, tz)));
        }
        if let Some(ref v) = self.tags
            && !v.is_empty()
        {
            lines.push(format!("**Tags:** {}", v.join(", ")));
        }
        if let Some(v) = self.is_notebook {
            lines.push(format!("**Notebook:** {}", v));
        }
        lines.join("\n")
    }

    pub fn display_list_item(&self) -> String {
        format!("- ID: {}\n  Project: {}", self.id, self.title)
    }
}

#[derive(Debug, Serialize, Default)]
pub struct ProjectCreate {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isNotebook")]
    pub is_notebook: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct ProjectUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isNotebook")]
    pub is_notebook: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_project_list_response() {
        let json = r#"{
            "projects": [
                {
                    "id": "P-123",
                    "title": "My Project",
                    "note": "Some notes",
                    "parentOrder": 1.0,
                    "isNotebook": false,
                    "tags": ["tag1", "tag2"],
                    "modificatedDate": "2025-01-01T00:00:00Z"
                }
            ]
        }"#;
        let resp: ProjectListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.projects.len(), 1);
        let p = &resp.projects[0];
        assert_eq!(p.id, "P-123");
        assert_eq!(p.title, "My Project");
        assert_eq!(p.note.as_deref(), Some("Some notes"));
        assert_eq!(p.parent_order, Some(1.0));
        assert_eq!(p.is_notebook, Some(false));
        assert_eq!(p.tags.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn deserialize_project_minimal() {
        let json = r#"{"id": "P-456", "title": "Bare"}"#;
        let p: Project = serde_json::from_str(json).unwrap();
        assert_eq!(p.id, "P-456");
        assert_eq!(p.title, "Bare");
        assert!(p.note.is_none());
        assert!(p.parent.is_none());
        assert!(p.tags.is_none());
    }

    #[test]
    fn serialize_create_skips_none() {
        let data = ProjectCreate {
            title: "Test".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json, serde_json::json!({"title": "Test"}));
    }

    #[test]
    fn serialize_create_includes_set_fields() {
        let data = ProjectCreate {
            title: "Test".to_string(),
            note: Some("A note".to_string()),
            is_notebook: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json["title"], "Test");
        assert_eq!(json["note"], "A note");
        assert_eq!(json["isNotebook"], true);
        assert!(json.get("parent").is_none());
    }

    #[test]
    fn serialize_update_empty() {
        let data = ProjectUpdate::default();
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn serialize_update_partial() {
        let data = ProjectUpdate {
            title: Some("New Title".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json, serde_json::json!({"title": "New Title"}));
    }
}
