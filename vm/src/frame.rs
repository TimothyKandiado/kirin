#[derive(Debug, Copy, Clone)]
pub struct Frame {
    pub return_address: Option<usize>,
    pub register_base: usize,
}
