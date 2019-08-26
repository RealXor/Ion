#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    pub mat_val: [[f32; 4]; 3],
}

impl Matrix {
    pub fn empty() -> Self {
        Self {
            mat_val: Default::default(),
        }
    }
}