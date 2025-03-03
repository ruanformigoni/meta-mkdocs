#![feature(let_chains)]
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
mod projects;
mod controller;

static DIR_PROJECTS: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));
static DOMAIN: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
static PORT_MAIN: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));
static PORT_EDITOR: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));
static PORT_SERVE: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));

// fn: main() {{{
#[tokio::main]
async fn main() -> anyhow::Result<()>
{
  let args: Vec<String> = std::env::args().collect();
  // Check arguments
  if args.len() != 6
  {
    eprintln!("Usage: /path/to/projects/directory domain port_main port_editor port_serve");
    std::process::exit(1);
  } // if
  // Create path
  let path = PathBuf::from(args.get(1).unwrap());
  if ! path.is_dir()
  {
    eprintln!("Path '{}' does not exist or is not a directory", path.to_string_lossy().into_owned());
    std::process::exit(1);
  } // if
  // Assign path
  *DIR_PROJECTS.lock().unwrap() = path;
  // Get server and port for self and editor
  *DOMAIN.lock().unwrap() = args.get(2).unwrap().into();
  *PORT_MAIN.lock().unwrap() = args.get(3).unwrap().into();
  *PORT_EDITOR.lock().unwrap() = args.get(4).unwrap().into();
  *PORT_SERVE.lock().unwrap() = args.get(5).unwrap().into();
  println!("domain: {}", *DOMAIN.lock().unwrap());
  println!("port_main: {}", PORT_MAIN.lock().unwrap().to_string_lossy());
  println!("port_editor: {}", PORT_EDITOR.lock().unwrap().to_string_lossy());
  println!("port_serve: {}", PORT_SERVE.lock().unwrap().to_string_lossy());
  // Create function router
  let app = Router::new()
    .route("/", get(controller::html))
    .route("/serve", post(controller::serve))
    .route("/create", post(controller::create))
    .route("/delete", post(controller::delete));
  // Bind to address
  let port_main: String = PORT_MAIN.lock().unwrap().to_string_lossy().to_string();
  axum::Server::bind(&format!("0.0.0.0:{}", &port_main).parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
  Ok(())
} // fn: main() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
