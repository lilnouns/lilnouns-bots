#[derive(Debug)]
pub struct Event {
    kind: String,
    title: Option<String>,
    description: Option<String>,
}

impl Event {
    // Constructor
    pub fn new(kind: String, title: Option<String>, description: Option<String>) -> Self {
        Self {
            kind,
            title,
            description,
        }
    }

    // Getters
    pub fn kind(&self) -> &String {
        &self.kind
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    // Setters
    pub fn set_kind(&mut self, kind: String) {
        self.kind = kind;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
}
