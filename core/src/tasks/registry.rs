/*
    Appellation: registry <tasks>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Task;
use crate::Shared;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct TaskRegistry {
    tasks: Shared<HashMap<Task, usize>>
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new()))
        }
    }
    pub fn register(&mut self, task: Task) -> usize {
        let mut tasks = self.tasks.lock().unwrap();
        let count = if let Some(cnt) = tasks.get_mut(&task) {
            *cnt += 1;
            *cnt
        } else {
           1
        };
        tasks.insert(task, count);
        count
    }
    pub fn running(&self) -> HashMap<Task, usize> {
        self.tasks.lock().unwrap().clone()
    }
}