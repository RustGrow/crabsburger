use crate::model::img::Image;
use crate::model::promo_card::PromoCard;

pub static PROMO_CARDS: &[PromoCard] = &[
    PromoCard {
    promo_type: "Burger promo",
    title: "GET A FREE DRINK WITH ANY BURGER",
    description: "Enjoy our juicy and delicious burgers with a refreshing beverage of your choice.",
    img: Image {
        class: "",
        src: "images/promo-1.png",
        alt: "A photo of a burger and a drink on a wooden table.",
    },
},
    PromoCard {
    promo_type: "Beverage promo",
    title: "BUY ONE, GET ONE FREE ON ALL BEVERAGES",
    description: "Quench your thirst with our selection of drinks, from water and juice to coffee and beer.",
    img: Image {
        class: "",
        src: "images/promo-2.png",
        alt: "A photo of two glasses of different beverages on a colorful background.",
    },
}
];
