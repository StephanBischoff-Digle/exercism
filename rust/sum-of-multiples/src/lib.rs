pub fn sum_of_multiples(n: u32, factors: &Vec<u32>) -> u32 {
    use std::collections::HashSet;

    if n == 0 {
        return 0u32;
    }

    let mut set = HashSet::new();
    for f in factors {
        for i in 1u32..n {
            if i % f == 0 {
                set.insert(i);
            }
        }
    }
    set.iter().sum()
}