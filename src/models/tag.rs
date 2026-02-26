use serde::{Deserialize, Serialize};

fn display_opt(o: &Option<String>) -> String {
    o.as_deref().unwrap_or("-").to_string()
}

#[derive(Debug, Deserialize)]
pub struct TagListResponse {
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: String,
    pub title: String,
    pub parent: Option<String>,
    pub order: Option<f64>,
    #[serde(rename = "modificatedDate")]
    #[allow(dead_code)]
    pub modificated_date: Option<String>,
}

impl Tag {
    pub fn display_detail(&self) -> String {
        let mut lines = vec![
            format!("**ID:** {}", self.id),
            format!("**Title:** {}", self.title),
        ];
        if let Some(ref v) = self.parent {
            lines.push(format!("**Parent:** {}", v));
        }
        if let Some(v) = self.order {
            lines.push(format!("**Order:** {}", v));
        }
        lines.join("\n")
    }

    pub fn display_list_item(&self) -> String {
        format!("- ID: {}\n  Tag: {}\n  Parent: {}", self.id, self.title, display_opt(&self.parent))
    }
}

#[derive(Debug, Serialize)]
pub struct TagCreate {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,
}

#[derive(Debug, Serialize, Default)]
pub struct TagUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tag_list() {
        let json = r#"{"tags": [{"id": "tag-1", "title": "urgent", "order": 1.0}]}"#;
        let resp: TagListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.tags.len(), 1);
        assert_eq!(resp.tags[0].id, "tag-1");
        assert_eq!(resp.tags[0].order, Some(1.0));
    }

    #[test]
    fn deserialize_tag_minimal() {
        let json = r#"{"id": "tag-2", "title": "low"}"#;
        let t: Tag = serde_json::from_str(json).unwrap();
        assert_eq!(t.id, "tag-2");
        assert!(t.parent.is_none());
        assert!(t.order.is_none());
    }

    #[test]
    fn serialize_create_skips_none() {
        let data = TagCreate {
            title: "bug".to_string(),
            parent: None,
            order: None,
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json, serde_json::json!({"title": "bug"}));
    }

    #[test]
    fn serialize_update_partial() {
        let data = TagUpdate {
            title: Some("renamed".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_value(&data).unwrap();
        assert_eq!(json, serde_json::json!({"title": "renamed"}));
    }
}
