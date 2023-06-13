#[derive(Debug)]
pub struct CardAdapter {
    pub text_items: String,
    pub items_format: String,
    pub front_format: String,
    pub back_format: String,
}

impl Default for CardAdapter {
    fn default() -> Self {
        CardAdapter {
            text_items: String::new(),
            items_format: String::new(),
            front_format: String::new(),
            back_format: String::new(),
        }
    }
}
