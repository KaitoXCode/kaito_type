#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

// Constructor
// TODO: add mutable cache for access control
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Property Accossors
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
