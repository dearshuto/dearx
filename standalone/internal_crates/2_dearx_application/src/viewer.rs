pub struct Viewer;

impl Viewer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Viewer {
    fn default() -> Self {
        Self::new()
    }
}
