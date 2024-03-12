#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FoodCard {
    pub img_path: &'static str,
    pub alt: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub price: f64,
}
