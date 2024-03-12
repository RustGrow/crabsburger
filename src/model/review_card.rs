#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ReviewCard {
    pub paragraph: &'static str,
    pub img: &'static str,
    pub img_alt: &'static str,
    pub name: &'static str,
    pub job: &'static str,
}
