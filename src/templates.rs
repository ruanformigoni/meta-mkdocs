use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a>
{
  pub projects: &'a [String],
  pub editor: &'a String,
  pub error: bool,
  pub error_msg: &'a String,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a>
{
  pub message: &'a str,
}
// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
