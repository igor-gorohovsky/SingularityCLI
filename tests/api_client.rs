use mockito::Server;
use singularity_cli::client::ApiClient;
use singularity_cli::models::project::{Project, ProjectCreate, ProjectListResponse};
use singularity_cli::models::task::{Task, TaskCreate};

#[test]
fn get_projects_list() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v2/project")
        .match_header("authorization", "Bearer test-token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"projects": [{"id": "P-1", "title": "Project One"}]}"#)
        .create();

    let client = ApiClient::with_base_url("test-token".into(), server.url());
    let resp: ProjectListResponse = client.get("/v2/project", &[]).unwrap();

    assert_eq!(resp.projects.len(), 1);
    assert_eq!(resp.projects[0].id, "P-1");
    assert_eq!(resp.projects[0].title, "Project One");
    mock.assert();
}

#[test]
fn get_projects_with_query_params() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v2/project")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("maxCount".into(), "10".into()),
            mockito::Matcher::UrlEncoded("includeRemoved".into(), "true".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"projects": []}"#)
        .create();

    let client = ApiClient::with_base_url("tok".into(), server.url());
    let query = vec![
        ("maxCount", "10".to_string()),
        ("includeRemoved", "true".to_string()),
    ];
    let resp: ProjectListResponse = client.get("/v2/project", &query).unwrap();

    assert!(resp.projects.is_empty());
    mock.assert();
}

#[test]
fn get_single_project() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v2/project/P-42")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "P-42", "title": "Found It"}"#)
        .create();

    let client = ApiClient::with_base_url("tok".into(), server.url());
    let project: Project = client.get("/v2/project/P-42", &[]).unwrap();

    assert_eq!(project.id, "P-42");
    assert_eq!(project.title, "Found It");
    mock.assert();
}

#[test]
fn post_creates_project() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/v2/project")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(
            serde_json::json!({"title": "New Project"}),
        ))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "P-new", "title": "New Project"}"#)
        .create();

    let client = ApiClient::with_base_url("tok".into(), server.url());
    let data = ProjectCreate {
        title: "New Project".to_string(),
        ..Default::default()
    };
    let project: Project = client.post("/v2/project", &data).unwrap();

    assert_eq!(project.id, "P-new");
    mock.assert();
}

#[test]
fn post_creates_task_with_fields() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/v2/task")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "title": "My Task",
            "priority": 0,
            "projectId": "P-1"
        })))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "T-new", "title": "My Task", "priority": 0, "projectId": "P-1"}"#)
        .create();

    let client = ApiClient::with_base_url("tok".into(), server.url());
    let data = TaskCreate {
        title: "My Task".to_string(),
        priority: Some(0),
        project_id: Some("P-1".to_string()),
        ..Default::default()
    };
    let task: Task = client.post("/v2/task", &data).unwrap();

    assert_eq!(task.id, "T-new");
    assert_eq!(task.priority, Some(0));
    mock.assert();
}

#[test]
fn patch_updates_project() {
    let mut server = Server::new();
    let mock = server
        .mock("PATCH", "/v2/project/P-42")
        .match_body(mockito::Matcher::Json(
            serde_json::json!({"title": "Updated"}),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "P-42", "title": "Updated"}"#)
        .create();

    let client = ApiClient::with_base_url("tok".into(), server.url());
    let data = singularity_cli::models::project::ProjectUpdate {
        title: Some("Updated".to_string()),
        ..Default::default()
    };
    let project: Project = client.patch("/v2/project/P-42", &data).unwrap();

    assert_eq!(project.title, "Updated");
    mock.assert();
}

#[test]
fn delete_succeeds() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/v2/project/P-42")
        .with_status(200)
        .create();

    let client = ApiClient::with_base_url("tok".into(), server.url());
    client.delete("/v2/project/P-42").unwrap();
    mock.assert();
}

#[test]
fn unauthorized_returns_error() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v2/project")
        .with_status(401)
        .with_body("Unauthorized")
        .create();

    let client = ApiClient::with_base_url("bad-token".into(), server.url());
    let result: Result<ProjectListResponse, _> = client.get("/v2/project", &[]);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("unauthorized"), "got: {}", err);
    mock.assert();
}

#[test]
fn server_error_returns_status_and_body() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/v2/project")
        .with_status(500)
        .with_body("Internal Server Error")
        .create();

    let client = ApiClient::with_base_url("tok".into(), server.url());
    let result: Result<ProjectListResponse, _> = client.get("/v2/project", &[]);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("500"), "got: {}", err);
    assert!(err.contains("Internal Server Error"), "got: {}", err);
    mock.assert();
}

#[test]
fn delete_error_returns_status() {
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/v2/project/P-99")
        .with_status(404)
        .with_body("Not Found")
        .create();

    let client = ApiClient::with_base_url("tok".into(), server.url());
    let result = client.delete("/v2/project/P-99");

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("404"), "got: {}", err);
    mock.assert();
}
