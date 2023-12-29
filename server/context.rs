use std::collections::HashMap;
use crate::matrix::Matrix;
use std::sync::{Arc, Mutex};

pub type PerSessionContextInstance = Arc<Mutex<PerSessionContext>>;

// Error triggered when session is not found when stopped
pub struct SessionNotFound;

pub struct ApplicationContext {
    sessions: HashMap<u64, PerSessionContextInstance>,
    stop: bool,
}

impl ApplicationContext {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            stop: false,
        }
    }
    pub fn session(&mut self, id: u64) -> PerSessionContextInstance {
        self.sessions.entry(id).or_insert_with(|| Arc::new(Mutex::new(PerSessionContext::new()))).clone()
    }
    pub fn new_session(&mut self, pid: u64) -> PerSessionContextInstance {
        self.sessions.insert(pid, Arc::new(Mutex::new(PerSessionContext::new())));
        self.session(pid)
    }
    pub fn stop_session(&mut self, id: u64) -> Result<(), SessionNotFound>{
        if self.sessions.remove(&id).is_none() {
            return Err(SessionNotFound);
        }
        self.stop = true;
        Ok(())
    }
    pub fn is_finishable(&self) -> bool {
        self.stop && self.sessions.is_empty()
    }
}

pub struct PerSessionContext {
    matrices: HashMap<u64, Matrix>,
    current_id: u64,
}

impl PerSessionContext {
    pub fn new() -> Self {
        Self {
            matrices: HashMap::new(),
            current_id: 1,
        }
    }
    pub fn matrix(&self, id: u64) -> Option<&Matrix> {
        self.matrices.get(&id)
    }
    pub fn matrix_mut(&mut self, id: u64) -> Option<&mut Matrix> {
        self.matrices.get_mut(&id)
    }
    pub fn add_matrix(&mut self, matrix: Matrix) -> u64 {
        self.matrices.insert(self.current_id, matrix);
        self.current_id += 1;
        self.current_id - 1
    }
}