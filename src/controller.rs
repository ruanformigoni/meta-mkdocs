// Imports
use std::
{
  sync::{LazyLock,Mutex},
  process::Child,
};
use serde::Deserialize;
use chrono::offset::Local;
use askama::Template;
use axum::{
  extract::Form,
  response::Html,
};

#[derive(Deserialize)]
pub struct CreateProjectRequest
{
  pub name: String,
}

static CHILD: LazyLock<Mutex<Option<(String,Child)>>> = LazyLock::new(|| Mutex::new(None));

// fn: query_serve() {{{
pub fn query_serve() -> String
{
  if let Some((name,_)) = CHILD.lock().unwrap().as_ref()
  {
    return name.clone();
  }
  return String::new();
} // fn: serve() }}}

// fn: serve() {{{
pub async fn serve(Form(payload): Form<CreateProjectRequest>) -> axum::response::Redirect
{
  // Get target project directory
  let project_dir = crate::DIR_PROJECTS.lock().unwrap().clone().join(&payload.name);
  // Kill existing process
  if let Some((_,child)) = CHILD.lock().unwrap().as_mut() && let Err(e) = child.kill()
  {
    eprintln!("{}", e);
  } // if
  // Serve project on the project's directory
  if let Err(e) = std::env::set_current_dir(project_dir.to_string_lossy().to_string())
  {
    eprintln!("{}", e);
  } // if
  let url_serve = format!("0.0.0.0:{}", crate::PORT_SERVE.lock().unwrap().to_string_lossy());
  let result = std::process::Command::new("mkdocs")
    .args(["serve", "-a", &url_serve])
    .spawn();
  // Redraw html
  if result.is_ok()
  {
    *CHILD.lock().unwrap() = Some((payload.name,result.ok().take().unwrap()));
  }
  axum::response::Redirect::to("/")
} // fn: serve() }}}

// fn: create() {{{
pub async fn create(Form(payload): Form<CreateProjectRequest>) -> axum::response::Redirect
{
  // Get target project directory
  let project_dir = crate::DIR_PROJECTS.lock().unwrap().clone().join(&payload.name);
  // Create project
  if let Err(e) = std::process::Command::new("mkdocs")
    .args(["new", &project_dir.to_string_lossy()])
    .output()
  {
    eprintln!("Failure to create project: {}", e);
  }
  // Back to root
  axum::response::Redirect::to("/")
} // fn: create() }}}

// fn: delete() {{{
pub async fn delete(Form(payload): Form<CreateProjectRequest>) -> axum::response::Redirect
{
  // Get target project directory
  let dir_root = crate::DIR_PROJECTS.lock().unwrap().clone();
  let name_project = payload.name;
  let dir_project = dir_root.clone().join(&name_project);
  // Erase project
  let mut target = String::from(".deleted.");
  target.push_str(&Local::now().format("%Y-%m-%d@%H:%M").to_string());
  target.push_str(&name_project);
  // Move folder to target name
  if let Err(e) = std::fs::rename(dir_project, dir_root.join(target))
  {
    eprintln!("Failure to delete project: {}", e);
  }
  // Back to root
  axum::response::Redirect::to("/")
} // fn: delete() }}}

// fn: html() {{{
pub async fn html() -> Html<String>
{
  let projects: Vec<String> = crate::projects::list();
  let template = crate::templates::IndexTemplate
  {
    domain: &crate::DOMAIN.lock().unwrap().clone().to_string(),
    port_serve: &crate::PORT_SERVE.lock().unwrap().to_string_lossy().to_string(),
    port_editor: &crate::PORT_EDITOR.lock().unwrap().to_string_lossy().to_string(),
    serving: &crate::controller::query_serve(),
    projects: &projects,
    error: false,
    error_msg: &String::new(),
  };
  Html(template.render().unwrap())
} // fn: html() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
