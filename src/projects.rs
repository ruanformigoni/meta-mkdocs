pub fn list() -> Vec<String>
{
  let project_dir = crate::DIR_PROJECTS.lock().unwrap().clone();
  std::fs::read_dir(project_dir)
    .unwrap()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_dir())
    .filter_map(|e| e.file_name().into_string().ok())
    .filter(|e| ! e.starts_with(".deleted"))
    .collect()
}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
