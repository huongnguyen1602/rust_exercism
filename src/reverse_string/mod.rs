#[allow(dead_code)]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

#[allow(dead_code)]
pub fn test() -> bool {
    assert_eq!(reverse("input"),"tupni");
    assert_eq!(reverse("子猫"), "猫子");
    println!("it is oke");
    true
}