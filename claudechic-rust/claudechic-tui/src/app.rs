use claudechic_core::models::Agent;

pub struct App {
    pub agents: Vec<Agent>,
    pub current_agent_idx: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            current_agent_idx: 0,
            should_quit: false,
        }
    }
}
