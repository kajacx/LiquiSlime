pub struct Texture {
    pub name: String,
    pub filename: String,
    pub primary_color_filename: Option<String>,
    pub secondary_color_filename: Option<String>,
}

impl Texture {
    pub fn new(name: String, filename: String) -> Self {
        Self {
            name,
            filename,
            primary_color_filename: None,
            secondary_color_filename: None,
        }
    }

    pub fn new_with_color(
        name: String,
        filename: String,
        primary_color_filename: Option<String>,
        secondary_color_filename: Option<String>,
    ) -> Self {
        Self {
            name,
            filename,
            primary_color_filename,
            secondary_color_filename,
        }
    }
}
