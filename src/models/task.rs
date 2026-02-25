use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize)]
pub struct TaskListResponse {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct Task {
    pub id: String,
    pub title: String,
    #[tabled(display_with = "display_priority")]
    pub priority: Option<i32>,
    #[tabled(display_with = "display_checked")]
    pub checked: Option<i32>,
    #[tabled(skip)]
    pub note: Option<String>,
    #[tabled(skip)]
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    #[tabled(skip)]
    pub parent: Option<String>,
    #[tabled(skip)]
    pub group: Option<String>,
    #[tabled(skip)]
    pub start: Option<String>,
    #[tabled(skip)]
    pub deadline: Option<String>,
    #[tabled(skip)]
    pub tags: Option<Vec<String>>,
    #[tabled(skip)]
    #[serde(rename = "showInBasket")]
    #[allow(dead_code)]
    pub show_in_basket: Option<bool>,
    #[tabled(skip)]
    #[serde(rename = "modificatedDate")]
    #[allow(dead_code)]
    pub modificated_date: Option<String>,
    #[tabled(skip)]
    #[serde(rename = "isNote")]
    #[allow(dead_code)]
    pub is_note: Option<bool>,
}

fn display_priority(p: &Option<i32>) -> String {
    match p {
        Some(0) => "high".to_string(),
        Some(1) => "normal".to_string(),
        Some(2) => "low".to_string(),
        _ => "-".to_string(),
    }
}

fn display_checked(c: &Option<i32>) -> String {
    match c {
        Some(0) => "empty".to_string(),
        Some(1) => "checked".to_string(),
        Some(2) => "cancelled".to_string(),
        _ => "-".to_string(),
    }
}

impl Task {
    pub fn display_detail(&self) {
        println!("ID:       {}", self.id);
        println!("Title:    {}", self.title);
        println!("Priority: {}", display_priority(&self.priority));
        println!("Checked:  {}", display_checked(&self.checked));
        if let Some(ref v) = self.note {
            println!("Note:     {}", v);
        }
        if let Some(ref v) = self.project_id {
            println!("Project:  {}", v);
        }
        if let Some(ref v) = self.parent {
            println!("Parent:   {}", v);
        }
        if let Some(ref v) = self.group {
            println!("Group:    {}", v);
        }
        if let Some(ref v) = self.start {
            println!("Start:    {}", v);
        }
        if let Some(ref v) = self.deadline {
            println!("Deadline: {}", v);
        }
        if let Some(ref v) = self.tags {
            if !v.is_empty() {
                println!("Tags:     {}", v.join(", "));
            }
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct TaskCreate {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "projectId")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isNote")]
    pub is_note: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct TaskUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "projectId")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isNote")]
    pub is_note: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_task_full() {
        let json = r#"{
            "id": "T-abc",
            "title": "Do stuff",
            "priority": 0,
            "checked": 1,
            "projectId": "P-123",
            "tags": ["t1"],
            "showInBasket": false,
            "modificatedDate": "2025-01-01T00:00:00Z",
            "isNote": false
        }"#;
        let t: Task = serde_json::from_str(json).unwrap();
        assert_eq!(t.id, "T-abc");
        assert_eq!(t.priority, Some(0));
        assert_eq!(t.checked, Some(1));
        assert_eq!(t.project_id.as_deref(), Some("P-123"));
    }

    #[test]
    fn deserialize_task_minimal() {
        let json = r#"{"id": "T-min", "title": "Minimal"}"#;
        let t: Task = serde_json::from_str(json).unwrap();
        assert_eq!(t.id, "T-min");
        assert!(t.priority.is_none());
        assert!(t.project_id.is_none());
    }

    #[test]
    fn serialize_create_skips_none() {
        let data = TaskCreate {
            title: "Test task".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json, serde_json::json!({"title": "Test task"}));
    }

    #[test]
    fn serialize_create_camel_case_rename() {
        let data = TaskCreate {
            title: "T".to_string(),
            project_id: Some("P-1".to_string()),
            is_note: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json["projectId"], "P-1");
        assert_eq!(json["isNote"], true);
        assert!(json.get("project_id").is_none());
    }

    #[test]
    fn serialize_update_partial() {
        let data = TaskUpdate {
            checked: Some(1),
            ..Default::default()
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json, serde_json::json!({"checked": 1}));
    }

    #[test]
    fn display_priority_values() {
        assert_eq!(display_priority(&Some(0)), "high");
        assert_eq!(display_priority(&Some(1)), "normal");
        assert_eq!(display_priority(&Some(2)), "low");
        assert_eq!(display_priority(&None), "-");
        assert_eq!(display_priority(&Some(99)), "-");
    }

    #[test]
    fn display_checked_values() {
        assert_eq!(display_checked(&Some(0)), "empty");
        assert_eq!(display_checked(&Some(1)), "checked");
        assert_eq!(display_checked(&Some(2)), "cancelled");
        assert_eq!(display_checked(&None), "-");
    }
}
