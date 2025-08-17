fn calculate_price_of_apples(num_apples: i32) -> i32 {
    let price = if num_apples > 40 {
        1
    } else {
        2
    };

    num_apples * price
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
