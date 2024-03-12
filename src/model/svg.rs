#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SvgPath {
    pub d: &'static str,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Svg {
    pub class: &'static str,
    pub xmlns: &'static str,
    pub view_box: &'static str,
    pub path: SvgPath,
}
