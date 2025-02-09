#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode{
    #[default]
    Osu,
    Mania,
    Taiko,
    Catch
}