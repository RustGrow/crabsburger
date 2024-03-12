use super::svg::Svg;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct HomeCardIcon {
    pub svg: Svg,
    pub title: &'static str,
}
