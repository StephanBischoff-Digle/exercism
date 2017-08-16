pub fn factors(number: u64) -> Vec<u64> {
    let mut ret_vec = Vec::new();
    let mut n = number;

    let mut k = 2;
    while n != 1 {
        if n % k == 0 {
            n = n / k;
            ret_vec.push(k);
        } else {
            k += 1;
        }
    }

    ret_vec
}