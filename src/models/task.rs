use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TaskListResponse {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub priority: Option<i32>,
    pub checked: Option<i32>,
    pub note: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    pub parent: Option<String>,
    pub group: Option<String>,
    pub start: Option<String>,
    pub deadline: Option<String>,
    #[serde(rename = "useTime")]
    pub use_time: Option<bool>,
    #[serde(rename = "timeLength")]
    pub time_length: Option<i64>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "showInBasket")]
    #[allow(dead_code)]
    pub show_in_basket: Option<bool>,
    #[serde(rename = "modificatedDate")]
    #[allow(dead_code)]
    pub modificated_date: Option<String>,
    #[serde(rename = "isNote")]
    #[allow(dead_code)]
    pub is_note: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItemListResponse {
    pub checklist_items: Vec<ChecklistItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    #[allow(dead_code)]
    pub id: String,
    pub title: String,
    pub done: Option<bool>,
    #[allow(dead_code)]
    pub parent_order: Option<f64>,
}

fn display_priority(p: &Option<i32>) -> String {
    match p {
        Some(0) => "high".to_string(),
        Some(1) => "normal".to_string(),
        Some(2) => "low".to_string(),
        _ => "-".to_string(),
    }
}

fn display_completed(c: &Option<i32>) -> String {
    match c {
        Some(1) => "true".to_string(),
        _ => "false".to_string(),
    }
}

fn parse_datetime(iso: &str) -> Option<NaiveDateTime> {
    let trimmed = iso.trim_end_matches('Z');
    NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S%.f"))
        .ok()
}

fn format_date(iso: &str) -> String {
    parse_datetime(iso)
        .map(|dt| dt.format("%a, %b %d, %Y").to_string())
        .unwrap_or_else(|| iso.to_string())
}

fn format_duration(iso: &str, use_time: Option<bool>, time_length: Option<i64>) -> String {
    match use_time {
        Some(true) => {
            let start_time = parse_datetime(iso)
                .map(|dt| dt.format("%H:%M").to_string())
                .unwrap_or_else(|| iso.to_string());
            match time_length {
                Some(minutes) if minutes > 0 => {
                    let end = parse_datetime(iso)
                        .map(|dt| {
                            (dt + chrono::Duration::minutes(minutes))
                                .format("%H:%M")
                                .to_string()
                        })
                        .unwrap_or_else(|| "...".to_string());
                    format!("{} - {}", start_time, end)
                }
                _ => format!("{} - ...", start_time),
            }
        }
        _ => "All Day".to_string(),
    }
}

impl Task {
    pub fn display_detail(&self, checklist: &[ChecklistItem]) -> String {
        let mut lines = vec![
            format!("**ID:** {}", self.id),
            format!("**Title:** {}", self.title),
        ];
        if let Some(ref v) = self.note {
            lines.push(format!("**Note:** {}", v));
        }
        if !checklist.is_empty() {
            lines.push("**Checklist:**".to_string());
            for item in checklist {
                let mark = if item.done == Some(true) { "x" } else { " " };
                lines.push(format!("  [{}] {}", mark, item.title));
            }
        }
        if let Some(ref v) = self.start {
            lines.push(format!("**Date:** {}", format_date(v)));
            lines.push(format!(
                "**Duration:** {}",
                format_duration(v, self.use_time, self.time_length)
            ));
        }
        if let Some(ref v) = self.deadline {
            lines.push(format!("**Deadline:** {}", format_date(v)));
        }
        lines.push(format!(
            "**Completed:** {}",
            display_completed(&self.checked)
        ));
        lines.push(format!(
            "**Priority:** {}",
            display_priority(&self.priority)
        ));
        if let Some(ref v) = self.project_id {
            lines.push(format!("**Project:** {}", v));
        }
        if let Some(ref v) = self.parent {
            lines.push(format!("**Parent:** {}", v));
        }
        if let Some(ref v) = self.group {
            lines.push(format!("**Group:** {}", v));
        }
        if let Some(ref v) = self.tags
            && !v.is_empty()
        {
            lines.push(format!("**Tags:** {}", v.join(", ")));
        }
        lines.join("\n")
    }

    pub fn display_list_item(&self, checklist: &[ChecklistItem]) -> String {
        let mut lines = vec![
            format!("- ID: {}", self.id),
            format!("  Task: {}", self.title),
        ];
        if let Some(ref v) = self.note {
            lines.push(format!("  Note: {}", v));
        }
        if !checklist.is_empty() {
            lines.push("  Checklist:".to_string());
            for item in checklist {
                let mark = if item.done == Some(true) { "x" } else { " " };
                lines.push(format!("    [{}] {}", mark, item.title));
            }
        }
        if let Some(ref v) = self.start {
            lines.push(format!("  Date: {}", format_date(v)));
            lines.push(format!(
                "  Duration: {}",
                format_duration(v, self.use_time, self.time_length)
            ));
        }
        if let Some(ref v) = self.deadline {
            lines.push(format!("  Deadline: {}", format_date(v)));
        }
        lines.push(format!("  Completed: {}", display_completed(&self.checked)));
        lines.push(format!("  Priority: {}", display_priority(&self.priority)));
        lines.join("\n")
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
            "isNote": false,
            "useTime": true,
            "timeLength": 90
        }"#;
        let t: Task = serde_json::from_str(json).unwrap();
        assert_eq!(t.id, "T-abc");
        assert_eq!(t.priority, Some(0));
        assert_eq!(t.checked, Some(1));
        assert_eq!(t.project_id.as_deref(), Some("P-123"));
        assert_eq!(t.use_time, Some(true));
        assert_eq!(t.time_length, Some(90));
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
    fn display_completed_values() {
        assert_eq!(display_completed(&Some(0)), "false");
        assert_eq!(display_completed(&Some(1)), "true");
        assert_eq!(display_completed(&Some(2)), "false");
        assert_eq!(display_completed(&None), "false");
    }

    #[test]
    fn format_date_iso8601() {
        assert_eq!(format_date("2026-02-27T09:00:00Z"), "Fri, Feb 27, 2026");
    }

    #[test]
    fn format_date_with_fractional_seconds() {
        assert_eq!(format_date("2026-02-27T09:00:00.000Z"), "Fri, Feb 27, 2026");
    }

    #[test]
    fn format_date_invalid_fallback() {
        assert_eq!(format_date("not-a-date"), "not-a-date");
    }

    #[test]
    fn format_duration_all_day() {
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(false), Some(0)),
            "All Day"
        );
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", None, None),
            "All Day"
        );
    }

    #[test]
    fn format_duration_with_time_range() {
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(true), Some(90)),
            "09:00 - 10:30"
        );
    }

    #[test]
    fn format_duration_open_ended() {
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(true), Some(0)),
            "09:00 - ..."
        );
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(true), None),
            "09:00 - ..."
        );
    }

    #[test]
    fn deserialize_checklist_item_list() {
        let json = r#"{"checklistItems": [
            {"id": "cl-1", "title": "Buy milk", "done": true, "parentOrder": 0.0},
            {"id": "cl-2", "title": "Call dentist", "done": false, "parentOrder": 1.0}
        ]}"#;
        let resp: ChecklistItemListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.checklist_items.len(), 2);
        assert_eq!(resp.checklist_items[0].title, "Buy milk");
        assert_eq!(resp.checklist_items[0].done, Some(true));
        assert_eq!(resp.checklist_items[1].title, "Call dentist");
        assert_eq!(resp.checklist_items[1].done, Some(false));
    }

    #[test]
    fn deserialize_checklist_item_minimal() {
        let json = r#"{"checklistItems": [{"id": "cl-1", "title": "Item"}]}"#;
        let resp: ChecklistItemListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.checklist_items.len(), 1);
        assert!(resp.checklist_items[0].done.is_none());
        assert!(resp.checklist_items[0].parent_order.is_none());
    }
}
