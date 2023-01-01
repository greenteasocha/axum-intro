use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    id: i32,
    text: String,
    completed: bool,
}

impl Task {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,   // id: id, と書かなくて良いらしい
            text, // text: text, と書かなくて良いらしい
            completed: false,
        }
    }
}

type TaskHashMap = HashMap<i32, Task>;

pub trait TaskRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTaskPayload) -> Task;
    fn find(&self, id: i32) -> Option<Task>; // findされないかも
    fn all(&self) -> Vec<Task>; // array
    fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task>; // any
    fn delete(&self, id: i32) -> anyhow::Result<()>; // anyhowは何？
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateTaskPayload {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateTaskPayload {
    text: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TaskRepositoryForMemory {
    store: Arc<RwLock<TaskHashMap>>,
}

impl TaskRepositoryForMemory {
    pub fn new() -> Self {
        TaskRepositoryForMemory {
            store: Arc::default(),
        }
    }
}

impl TaskRepository for TaskRepositoryForMemory {
    fn create(&self, payload: CreateTaskPayload) -> Task {
        todo!();
    }
    fn find(&self, id: i32) -> Option<Task> {
        todo!();
    }
    fn all(&self) -> Vec<Task> {
        todo!();
    }
    fn update(&self, id: i32, payload: UpdateTaskPayload) -> anyhow::Result<Task> {
        todo!();
    }
    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
}
