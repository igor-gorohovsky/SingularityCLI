use serde::{Deserialize, Serialize};

fn display_opt(o: &Option<String>) -> String {
    o.as_deref().unwrap_or("-").to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskGroupListResponse {
    pub task_groups: Vec<TaskGroup>,
}

#[derive(Debug, Deserialize)]
pub struct TaskGroup {
    pub id: String,
    pub title: String,
    pub parent: Option<String>,
    #[serde(rename = "parentOrder")]
    pub parent_order: Option<f64>,
    pub fake: Option<bool>,
    #[serde(rename = "modificatedDate")]
    #[allow(dead_code)]
    pub modificated_date: Option<String>,
}

impl TaskGroup {
    pub fn display_detail(&self) -> String {
        let mut lines = vec![
            format!("**ID:** {}", self.id),
            format!("**Title:** {}", self.title),
        ];
        if let Some(ref v) = self.parent {
            lines.push(format!("**Parent:** {}", v));
        }
        if let Some(v) = self.parent_order {
            lines.push(format!("**Order:** {}", v));
        }
        if let Some(v) = self.fake {
            lines.push(format!("**Fake:** {}", v));
        }
        lines.join("\n")
    }

    pub fn display_list_item(&self) -> String {
        format!("- ID: {}\n  Group: {}\n  Parent: {}", self.id, self.title, display_opt(&self.parent))
    }
}

#[derive(Debug, Serialize)]
pub struct TaskGroupCreate {
    pub title: String,
    pub parent: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentOrder")]
    pub parent_order: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fake: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct TaskGroupUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentOrder")]
    pub parent_order: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fake: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_task_group_list() {
        let json = r#"{"taskGroups": [{"id": "Q-1", "title": "Group 1", "parent": "P-1", "parentOrder": 2.0}]}"#;
        let resp: TaskGroupListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.task_groups.len(), 1);
        assert_eq!(resp.task_groups[0].id, "Q-1");
        assert_eq!(resp.task_groups[0].parent.as_deref(), Some("P-1"));
        assert_eq!(resp.task_groups[0].parent_order, Some(2.0));
    }

    #[test]
    fn deserialize_task_group_minimal() {
        let json = r#"{"id": "Q-2", "title": "Bare"}"#;
        let g: TaskGroup = serde_json::from_str(json).unwrap();
        assert_eq!(g.id, "Q-2");
        assert!(g.parent.is_none());
        assert!(g.fake.is_none());
    }

    #[test]
    fn serialize_create_camel_case() {
        let data = TaskGroupCreate {
            title: "G".to_string(),
            parent: "P-1".to_string(),
            parent_order: Some(3.0),
            fake: None,
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json["parentOrder"], 3.0);
        assert!(json.get("parent_order").is_none());
        assert!(json.get("fake").is_none());
    }

    #[test]
    fn serialize_update_empty() {
        let data = TaskGroupUpdate::default();
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json, serde_json::json!({}));
    }
}
