use super::img::Image;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PromoCard {
    pub promo_type: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub img: Image,
}
