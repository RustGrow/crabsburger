use crate::model::category_card::CategoryCard;
use crate::model::img::Image;

pub static CATEGORY_CARDS: &[CategoryCard] = &[
    CategoryCard {
    title: "BURGER",
    text: "Eat healthy and tasty",
    img: Image{
        class: "",
        src: "images/burger-1.png",
        alt: "A photo of a big and juicy burger with a patty, cheese, lettuce and tomato on a sesame bun.",
    },
    bg_color: "bg-secondaryColor",
    // bg_color: "bg-redColor",

    },
    CategoryCard {
    title: "SNACK",
    text: "Snack smart and happy",
    img: Image{
        class: "",
        src: "images/snack-1.png",
        alt: "A photo of various snacks, such as nuts, dried fruits, cheese sticks, chips, cookies and so on.",
    },
    bg_color: "bg-redColor",
    },
    CategoryCard {
    title: "BEVERAGE",
    text: "Drink enough and well",
    img: Image{
        class: "",
        src: "images/beverage-2.png",
        alt: "A photo of different drinks, such as water, milk, juice, tea, coffee, beer, wine and so on.",
    },
    bg_color: "bg-greenColor",
    },
];
