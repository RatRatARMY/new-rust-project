#[derive(Debug, Clone)]
pub enum Events {
    SolveEquation,
    AChanged(String),
    BChanged(String),
    CChanged(String),
    DChanged(String),
    ChangePage(crate::page::Pages),
}
