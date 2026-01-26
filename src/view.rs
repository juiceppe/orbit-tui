#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewType {
    ProfileSelect,
    Projects,
    Targets {
        project: String,
    },
    Services {
        project: String,
        target: String,
    },
    Schema {
        project: String,
        target: String,
        service: String,
    },
}

pub struct NavigationStack {
    stack: Vec<ViewType>,
    pub current: ViewType,
}

impl NavigationStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            current: ViewType::ProfileSelect,
        }
    }

    pub fn push(&mut self, view: ViewType) {
        self.stack.push(self.current.clone());
        self.current = view;
    }

    pub fn pop(&mut self) -> bool {
        if let Some(previous) = self.stack.pop() {
            self.current = previous;
            true
        } else {
            false // We are at root already
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.stack.is_empty()
    }
}

impl Default for NavigationStack {
    fn default() -> Self {
        Self::new()
    }
}
