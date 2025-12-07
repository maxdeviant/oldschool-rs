#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum ClueTier {
    Beginner,
    Easy,
    Medium,
    Hard,
    Elite,
    Master,
}
