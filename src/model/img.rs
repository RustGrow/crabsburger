#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Image {
    pub class: &'static str,
    pub src: &'static str,
    pub alt: &'static str,
}
