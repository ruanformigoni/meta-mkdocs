// Imports
use std::
{
  sync::{LazyLock, Mutex},
  path::PathBuf,
};
use axum::{
  routing::{get, post},
  Router,
};

// Modules
mod templates;
mod request;
mod projects;
mod controller;

static DIR_PROJECTS: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));
static URL_SELF: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));
static URL_EDITOR: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));

// fn: main() {{{
#[tokio::main]
async fn main() -> anyhow::Result<()>
{
  let args: Vec<String> = std::env::args().collect();
  // Check arguments
  if args.len() != 4
  {
    eprintln!("Usage: /path/to/projects/directory server:port server:port");
    std::process::exit(1);
  } // if
  // Create path
  let path = PathBuf::from(args.get(1).unwrap());
  if ! path.is_dir()
  {
    eprintln!("Path '{}' does not exist or is not a directory", path.to_string_lossy().into_owned());
    std::process::exit(1);
  } // if
  // Get server and port for self and editor
  *URL_SELF.lock().unwrap() = args.get(2).unwrap().into();
  *URL_EDITOR.lock().unwrap() = format!("http://{}/files", args.get(3).unwrap()).into();
  // Assign path
  *DIR_PROJECTS.lock().unwrap() = path;
  // Create function router
  let app = Router::new()
    .route("/", get(controller::html))
    .route("/create", post(controller::create))
    .route("/delete", post(controller::delete))
    .route("/list", get(controller::list));
  // Bind to address
  axum::Server::bind(&URL_SELF.lock().unwrap().to_string_lossy().to_owned().parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
  Ok(())
} // fn: main() }}}


// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
