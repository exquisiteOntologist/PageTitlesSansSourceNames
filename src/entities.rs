pub struct Title<'a> {
    /// This ID can be used to track the titles against external entities.
    pub id: &'a i32,
    /// The title
    pub title: &'a str,
}

impl<'a> Title<'a> {
    /// Create a new title from and ID and title.
    /// Note that the pure_title field will be blank.
    pub fn new(id: &'a i32, title: &'a str) -> Self {
        Self { id, title }
    }
}
