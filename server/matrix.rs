
pub struct Matrix {
    size: (u32, u32),
    data: Vec<f32>,
}

impl Matrix {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            data: vec![0.0; (size.0 * size.1) as usize],
        }
    }
    pub fn identity(size: (u32, u32)) -> Self {
        let mut data = vec![0.0; (size.0 * size.1) as usize];
        data.iter_mut().step_by(size.0 as usize + 1).for_each(|v| *v = 1.0);
        Self {
            size,
            data,
        }
    }
}