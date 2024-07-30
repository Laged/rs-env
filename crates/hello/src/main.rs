fn main() {
  hello_a::hello();
  hello_b::hello();
}

#[cfg(test)]
mod tests { 
  #[test]
  fn hello_works() {
    let a = hello_a::hello();
    let b = hello_b::hello();
    let parts_a: Vec<&str> = a.split(". ").collect();
    let parts_b: Vec<&str> = b.split(". ").collect();
    assert_eq!(2, parts_a.len());
    assert_eq!(parts_a.len(), parts_b.len());
    assert_eq!(parts_a[0], parts_b[0]);
    assert_ne!(parts_a[1], parts_b[1]);
  }
}
