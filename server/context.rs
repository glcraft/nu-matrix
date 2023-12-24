use std::collections::HashMap;
use crate::matrix::Matrix;

pub struct ApplicationContext {
    sessions: HashMap<u64, PerSessionContext>,
}

impl ApplicationContext {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
    pub fn session(&self, id: u64) -> Option<&PerSessionContext> {
        self.sessions.get(&id)
    }
    pub fn session_mut(&mut self, id: u64) -> Option<&mut PerSessionContext> {
        self.sessions.get_mut(&id)
    }
    pub fn new_session(&mut self, pid: u64) -> &mut PerSessionContext {
        self.sessions.insert(pid, PerSessionContext::new());
        unsafe { self.session_mut(pid).unwrap_unchecked() }
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