pub fn raindrops(n: u32) -> String {
    let mut ret_str = String::new();
    if n % 3 == 0 {
        ret_str += "Pling";
    }
    if n % 5 == 0 {
        ret_str += "Plang";
    }
    if n % 7 == 0 {
        ret_str += "Plong";
    }

    if ret_str.is_empty() {
        ret_str += &n.to_string();
    }

    ret_str
}
