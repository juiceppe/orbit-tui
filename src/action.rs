#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    Tick,
    Render,
    NavigateUp,
    NavigateDown,
    Select,
    Back,
    ToggleSupergraph,
}
