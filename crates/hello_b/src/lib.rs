const YO: &str = "Hello t. B";

pub fn hello() -> String {
    println!("{}", YO);
    YO.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_b_works() {
        let result = hello();
        assert_eq!(result, YO);
    }
}
