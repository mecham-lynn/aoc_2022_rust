use argh::FromArgs;

#[derive(FromArgs)]
/// Set of args for Day 3
pub struct AocArgs{
    /// enables part_one functionality
    #[argh(switch)]
    pub part_one: bool,

    /// enables part_two functionality
    #[argh(switch)]
    pub part_two: bool,

    /// flag on whether to use demo text or the actual text
    #[argh(switch)]
    pub demo_text: bool,
}