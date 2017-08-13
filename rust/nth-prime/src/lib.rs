/// Calculates the nth prime number.
/// 
/// # Arguments
/// 
/// * `n` - The number of the prime to be returned.
/// 
/// # Source
/// 
/// [Daniel Fischer @
/// stackoverflow](https://stackoverflow.com/questions/9625663/calculating-and-printing-the-nth-prime-number/9704912#9704912)
/// 
pub fn nth(n: u32) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("non valid n!");
    }
    if n < 2 {
        return Ok(2);
    }
    if n == 2 {
        return Ok(3);
    }

    let m = n as f64;
    let pre_limit = (m * (m.ln() + m.ln().ln())).ceil() as u64 + 3;
    let pre_root = (pre_limit as f64).sqrt().ceil() as u64 + 1;
    let limit = (pre_limit - 1) / 2;
    let root = pre_root / 2 - 1;

    let mut sieve = vec![false; limit as usize];
    let mut count = 1;
    for i in 0..root {
        if !sieve[i as usize] {
            count += 1;

            let mut j = 2 * i * (i + 3) + 3;
            let p = 2 * i + 3;
            while j < limit {
                sieve[j as usize] = true;
                j += p;
            }
        }
    }

    let mut p = root;
    while count < n {
        if !sieve[p as usize] {
            count += 1;
        }
        p += 1;
    }

    Ok(2 * p + 1)
}
