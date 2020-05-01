#[derive(PartialEq, Clone, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size >= shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 20,
                style: String::from("sandal"),
            },
            Shoe {
                size: 30,
                style: String::from("boot"),
            },
        ];

        let myshoes = shoes_in_my_size(shoes.clone(), 25);
        assert_eq!(myshoes.as_slice(), &shoes[2..=2]);
    }
}
