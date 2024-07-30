const YO: &str = "Hello t. A";

pub fn hello() -> String {
    println!("{}", YO);
    YO.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_a_works() {
        let result = hello();
        assert_eq!(result, YO);
    }
}
