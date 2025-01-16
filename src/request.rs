// Imports
use serde::Deserialize;
use askama::Template;
use axum::{
  response::Html,
  Json,
};

#[derive(Deserialize)]
pub struct CreateProjectRequest
{
  pub name: String,
}

// fn: list() {{{
pub async fn list() -> Json<serde_json::Value>
{
  // Get projects from project directory
  let projects: Vec<String> = crate::projects::list();
  // Send response
  Json(serde_json::json!({ "projects": projects }))
} // fn: list() }}}

// fn: html() {{{
pub async fn html() -> Html<String>
{
  let projects: Vec<String> = crate::projects::list();
  let template = crate::templates::IndexTemplate
  {
    projects: &projects,
    error: false,
    error_msg: &String::new(),
  };
  Html(template.render().unwrap())
} // fn: html() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
