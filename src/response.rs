use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct NoteResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct NoteData {
    pub note: NoteResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleNoteResponse {
    pub status: &'static str,
    pub data: NoteData,
}

#[derive(Serialize, Debug)]
pub struct NoteListResponse {
    pub status: &'static str,
    pub results: usize,
    pub notes: Vec<NoteResponse>,
}
