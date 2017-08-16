pub fn square(s: u32) -> u64 {
    if s > 64 || s == 0 {
        panic!("Square must be between 1 and 64");
    }

    2u64.pow(s-1)
}

pub fn total() -> u64 {
    let mut sum = 0;
    for i in 1u32..65u32 {
        sum += square(i);
    }

    sum
}
