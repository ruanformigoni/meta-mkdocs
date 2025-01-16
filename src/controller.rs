// Imports
use chrono::offset::Local;
use askama::Template;
use axum::{
  extract::Form,
  response::Html,
};

// fn: create() {{{
pub async fn create(Form(payload): Form<crate::request::CreateProjectRequest>) -> Html<String>
{
  // Get target project directory
  let project_dir = crate::DIR_PROJECTS.lock().unwrap().clone().join(&payload.name);
  // Check if it exists
  if project_dir.exists()
  {
    let template = crate::templates::IndexTemplate
    {
      projects: &crate::projects::list(),
      editor: &crate::URL_EDITOR.lock().unwrap().to_string_lossy().to_string(),
      error: true,
      error_msg: &String::from("Project exists"),
    };
    return Html(template.render().unwrap());
  }
  // Create project
  let result = std::process::Command::new("mkdocs")
    .args(["new", &project_dir.to_string_lossy()])
    .output();
  // Redraw html
  if result.is_ok()
  {
    crate::request::html().await
  }
  else
  {
    let template = crate::templates::ErrorTemplate
    {
      message: "Failed to create project",
    };
    Html(template.render().unwrap())
  }
} // fn: create() }}}

// fn: delete() {{{
pub async fn delete(Form(payload): Form<crate::request::CreateProjectRequest>) -> Html<String>
{
  // Get target project directory
  let dir_root = crate::DIR_PROJECTS.lock().unwrap().clone();
  let name_project = payload.name;
  let dir_project = dir_root.clone().join(&name_project);
  // Check if it exists
  if ! dir_project.exists() || dir_root == dir_project
  {
    let template = crate::templates::IndexTemplate
    {
      projects: &crate::projects::list(),
      editor: &crate::URL_EDITOR.lock().unwrap().to_string_lossy().to_string(),
      error: true,
      error_msg: &String::from("Project does not exist"),
    };
    return Html(template.render().unwrap());
  }
  // Erase project
  let mut target = String::from(".deleted.");
  target.push_str(&Local::now().format("%Y-%m-%d@%H:%M").to_string());
  target.push_str(&name_project);
  let result = std::fs::rename(dir_project, dir_root.join(target));
  // Redraw html
  if result.is_ok()
  {
    crate::request::html().await
  }
  else
  {
    let template = crate::templates::ErrorTemplate { message: "Failed to delete project", };
    Html(template.render().unwrap())
  }
} // fn: delete() }}}

// fn: html() {{{
pub async fn html() -> Html<String>
{
  crate::request::html().await
} // fn: html() }}}

// fn: list() {{{
pub async fn list() -> axum::Json<serde_json::Value>
{
  crate::request::list().await
} // fn: list() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
