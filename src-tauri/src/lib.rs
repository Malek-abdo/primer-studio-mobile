use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub duration: f32,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub asset_id: String,
    pub name: String,
    pub kind: String,
    pub start: f32,
    pub duration: f32,
    pub track: u8,
    pub color: String,
    pub volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub updated_at: String,
    pub duration: f32,
    pub assets: Vec<Asset>,
    pub clips: Vec<Clip>,
}

fn db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("primer-realm-local.json"))
}

fn read_projects(app: &AppHandle) -> Result<Vec<Project>, String> {
    let path = db_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

fn write_projects(app: &AppHandle, projects: &[Project]) -> Result<(), String> {
    let path = db_path(app)?;
    let data = serde_json::to_string_pretty(projects).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_projects(app: AppHandle) -> Result<Vec<Project>, String> {
    read_projects(&app)
}

#[tauri::command]
fn create_project(app: AppHandle, name: String) -> Result<Project, String> {
    let mut projects = read_projects(&app)?;
    let now = chrono::Utc::now().to_rfc3339();
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        updated_at: now,
        duration: 0.0,
        assets: Vec::new(),
        clips: Vec::new(),
    };
    projects.insert(0, project.clone());
    write_projects(&app, &projects)?;
    Ok(project)
}

#[tauri::command]
fn save_project(app: AppHandle, project: Project) -> Result<Project, String> {
    let mut projects = read_projects(&app)?;
    let mut saved = project;
    saved.updated_at = chrono::Utc::now().to_rfc3339();
    if let Some(item) = projects.iter_mut().find(|p| p.id == saved.id) {
        *item = saved.clone();
    } else {
        projects.insert(0, saved.clone());
    }
    write_projects(&app, &projects)?;
    Ok(saved)
}

#[tauri::command]
fn delete_project(app: AppHandle, id: String) -> Result<(), String> {
    let mut projects = read_projects(&app)?;
    projects.retain(|p| p.id != id);
    write_projects(&app, &projects)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_projects,
            create_project,
            save_project,
            delete_project
        ])
        .run(tauri::generate_context!())
        .expect("error while building Primer Studio");
}
