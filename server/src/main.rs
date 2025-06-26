use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_cors::Cors;
use actix_files::Files;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub patterns: Vec<PatternData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternData {
    pub id: u32,
    pub name: String,
    pub steps: Vec<Option<StepData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepData {
    pub frequency: f32,
    pub duration: f32,
    pub amplitude: f32,
    pub waveform: String,
}

struct AppState {
    sessions: Mutex<HashMap<String, Session>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        sessions: Mutex::new(HashMap::new()),
    });

    println!("🎛️ ntrck server starting on http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(Files::new("/", "../static").index_file("index.html"))
            .route("/api/sessions", web::get().to(get_sessions))
            .route("/api/sessions", web::post().to(create_session))
            .route("/api/sessions/{id}", web::get().to(get_session))
            .route("/api/sessions/{id}", web::put().to(update_session))
            .route("/api/sessions/{id}", web::delete().to(delete_session))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_sessions(data: web::Data<AppState>) -> Result<HttpResponse> {
    let sessions = data.sessions.lock().unwrap();
    let session_list: Vec<Session> = sessions.values().cloned().collect();
    
    Ok(HttpResponse::Ok().json(session_list))
}

async fn create_session(data: web::Data<AppState>) -> Result<HttpResponse> {
    let session_id = Uuid::new_v4().to_string();
    let session = Session {
        id: session_id.clone(),
        name: format!("Session {}", chrono::Utc::now().timestamp()),
        created_at: Utc::now(),
        patterns: vec![],
    };

    {
        let mut sessions = data.sessions.lock().unwrap();
        sessions.insert(session_id.clone(), session.clone());
    }

    Ok(HttpResponse::Created().json(session))
}

async fn get_session(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let session_id = path.into_inner();
    let sessions = data.sessions.lock().unwrap();
    
    if let Some(session) = sessions.get(&session_id) {
        Ok(HttpResponse::Ok().json(session))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Session not found"
        })))
    }
}

async fn update_session(
    path: web::Path<String>,
    session_data: web::Json<Session>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let session_id = path.into_inner();
    let mut sessions = data.sessions.lock().unwrap();
    
    if sessions.contains_key(&session_id) {
        sessions.insert(session_id, session_data.into_inner());
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Session updated successfully"
        })))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Session not found"
        })))
    }
}

async fn delete_session(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let session_id = path.into_inner();
    let mut sessions = data.sessions.lock().unwrap();
    
    if sessions.remove(&session_id).is_some() {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Session deleted successfully"
        })))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Session not found"
        })))
    }
}
