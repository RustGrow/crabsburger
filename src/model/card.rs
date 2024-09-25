#[derive(PartialEq, Debug, Clone)]
pub struct Category {
    pub title: String,
    pub text: String,
    pub img: Image,
    pub bg_color: &'static str,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Food {
    pub img_path: &'static str,
    pub alt: String,
    pub title: String,
    pub description: String,
    pub price: String,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct HomeIcon {
    pub svg: Svg,
    pub title: &'static str,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Image {
    pub class: &'static str,
    pub src: &'static str,
    pub alt: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Promo {
    pub promo_type: String,
    pub title: String,
    pub description: String,
    pub img: Image,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Review {
    pub paragraph: &'static str,
    pub img: &'static str,
    pub img_alt: &'static str,
    pub name: &'static str,
    pub job: &'static str,
}

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
