use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
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

fn format_in_tz(naive: NaiveDateTime, tz: Option<Tz>, fmt: &str) -> String {
    match tz {
        Some(tz) => Utc
            .from_utc_datetime(&naive)
            .with_timezone(&tz)
            .format(fmt)
            .to_string(),
        None => naive.format(fmt).to_string(),
    }
}

pub(crate) fn format_date(iso: &str, tz: Option<Tz>) -> String {
    parse_datetime(iso)
        .map(|dt| format_in_tz(dt, tz, "%a, %b %d, %Y"))
        .unwrap_or_else(|| iso.to_string())
}

fn format_duration(
    iso: &str,
    use_time: Option<bool>,
    time_length: Option<i64>,
    tz: Option<Tz>,
) -> String {
    if use_time != Some(true) {
        return "All Day".to_string();
    }
    let parsed = match parse_datetime(iso) {
        Some(dt) => dt,
        None => return iso.to_string(),
    };
    let start_time = format_in_tz(parsed, tz, "%H:%M");
    match time_length {
        Some(minutes) if minutes > 0 => {
            let end_dt = match tz {
                Some(tz) => {
                    let local = Utc.from_utc_datetime(&parsed).with_timezone(&tz);
                    (local + chrono::Duration::minutes(minutes))
                        .format("%H:%M")
                        .to_string()
                }
                None => (parsed + chrono::Duration::minutes(minutes))
                    .format("%H:%M")
                    .to_string(),
            };
            format!("{} - {}", start_time, end_dt)
        }
        _ => format!("{} - ...", start_time),
    }
}

pub fn convert_date_filter(value: &str, is_end: bool, tz: Option<Tz>) -> anyhow::Result<String> {
    if value.contains('T') {
        return Ok(value.to_string());
    }
    let naive_date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("invalid date '{}': {}", value, e))?;
    let naive_dt = if is_end {
        naive_date.and_hms_opt(23, 59, 59).unwrap()
    } else {
        naive_date.and_hms_opt(0, 0, 0).unwrap()
    };
    match tz {
        Some(tz) => {
            let local_dt = tz.from_local_datetime(&naive_dt).single().ok_or_else(|| {
                anyhow::anyhow!("ambiguous or invalid local time for date '{}'", value)
            })?;
            Ok(local_dt
                .with_timezone(&Utc)
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string())
        }
        None => Ok(format!(
            "{}T{}Z",
            value,
            if is_end { "23:59:59" } else { "00:00:00" }
        )),
    }
}

impl Task {
    pub fn display_detail(&self, checklist: &[ChecklistItem], tz: Option<Tz>) -> String {
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
            lines.push(format!("**Date:** {}", format_date(v, tz)));
            lines.push(format!(
                "**Duration:** {}",
                format_duration(v, self.use_time, self.time_length, tz)
            ));
        }
        if let Some(ref v) = self.deadline {
            lines.push(format!("**Deadline:** {}", format_date(v, tz)));
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

    pub fn display_list_item(&self, checklist: &[ChecklistItem], tz: Option<Tz>) -> String {
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
            lines.push(format!("  Date: {}", format_date(v, tz)));
            lines.push(format!(
                "  Duration: {}",
                format_duration(v, self.use_time, self.time_length, tz)
            ));
        }
        if let Some(ref v) = self.deadline {
            lines.push(format!("  Deadline: {}", format_date(v, tz)));
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
        assert_eq!(
            format_date("2026-02-27T09:00:00Z", None),
            "Fri, Feb 27, 2026"
        );
    }

    #[test]
    fn format_date_with_fractional_seconds() {
        assert_eq!(
            format_date("2026-02-27T09:00:00.000Z", None),
            "Fri, Feb 27, 2026"
        );
    }

    #[test]
    fn format_date_invalid_fallback() {
        assert_eq!(format_date("not-a-date", None), "not-a-date");
    }

    #[test]
    fn format_date_with_timezone() {
        let tz: Tz = "Europe/Kyiv".parse().unwrap();
        // 23:00 UTC = 01:00 next day in Kyiv (UTC+2 in winter)
        assert_eq!(
            format_date("2026-02-27T23:00:00Z", Some(tz)),
            "Sat, Feb 28, 2026"
        );
    }

    #[test]
    fn format_date_without_timezone_unchanged() {
        assert_eq!(
            format_date("2026-02-27T23:00:00Z", None),
            "Fri, Feb 27, 2026"
        );
    }

    #[test]
    fn format_duration_all_day() {
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(false), Some(0), None),
            "All Day"
        );
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", None, None, None),
            "All Day"
        );
    }

    #[test]
    fn format_duration_with_time_range() {
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(true), Some(90), None),
            "09:00 - 10:30"
        );
    }

    #[test]
    fn format_duration_with_timezone() {
        let tz: Tz = "Europe/Kyiv".parse().unwrap();
        // 09:00 UTC = 11:00 Kyiv (UTC+2)
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(true), Some(90), Some(tz)),
            "11:00 - 12:30"
        );
    }

    #[test]
    fn format_duration_open_ended() {
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(true), Some(0), None),
            "09:00 - ..."
        );
        assert_eq!(
            format_duration("2026-02-27T09:00:00Z", Some(true), None, None),
            "09:00 - ..."
        );
    }

    #[test]
    fn convert_date_filter_start_with_timezone() {
        let tz: Tz = "Europe/Kyiv".parse().unwrap();
        // Kyiv is UTC+2 in winter, so 2026-02-28T00:00:00+02:00 = 2026-02-27T22:00:00Z
        let result = convert_date_filter("2026-02-28", false, Some(tz)).unwrap();
        assert_eq!(result, "2026-02-27T22:00:00Z");
    }

    #[test]
    fn convert_date_filter_end_with_timezone() {
        let tz: Tz = "Europe/Kyiv".parse().unwrap();
        let result = convert_date_filter("2026-02-28", true, Some(tz)).unwrap();
        assert_eq!(result, "2026-02-28T21:59:59Z");
    }

    #[test]
    fn convert_date_filter_without_timezone() {
        let result = convert_date_filter("2026-02-28", false, None).unwrap();
        assert_eq!(result, "2026-02-28T00:00:00Z");
        let result = convert_date_filter("2026-02-28", true, None).unwrap();
        assert_eq!(result, "2026-02-28T23:59:59Z");
    }

    #[test]
    fn convert_date_filter_passthrough_full_iso() {
        let result = convert_date_filter("2026-02-28T00:00:00Z", false, None).unwrap();
        assert_eq!(result, "2026-02-28T00:00:00Z");
    }

    #[test]
    fn convert_date_filter_invalid_date() {
        assert!(convert_date_filter("not-a-date", false, None).is_err());
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
