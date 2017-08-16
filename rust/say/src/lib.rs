use std::collections::HashMap;


pub fn encode(number: u64) -> String {
    DecomposedNumber::new(number).say()
}

#[derive(Debug)]
pub struct DecomposedNumber {
    digits: Vec<u8>,
}

impl DecomposedNumber {
    pub fn new(n: u64) -> DecomposedNumber {

        let mut v: Vec<u8> = Vec::new();

        if n == 0 {
            v.push(0);
        } else {
            let mut k = n;
            while k != 0 {
                let p = k % 10;
                v.push(p as u8);
                k -= p;
                if k >= 10 {
                    k = k / 10;
                }
            }
        }

        DecomposedNumber { digits: v }
    }

    pub fn say(&self) -> String {
        let mut ones = HashMap::new();
        ones.insert(0, "zero");
        ones.insert(1, "one");
        ones.insert(2, "two");
        ones.insert(3, "three");
        ones.insert(4, "four");
        ones.insert(5, "five");
        ones.insert(6, "six");
        ones.insert(7, "seven");
        ones.insert(8, "eight");
        ones.insert(9, "nine");

        let mut teens = HashMap::new();
        teens.insert(10, "ten");
        teens.insert(11, "eleven");
        teens.insert(12, "twelve");
        teens.insert(13, "thirdteen");
        teens.insert(14, "fourteen");
        teens.insert(15, "fifteen");
        teens.insert(16, "sixteen");
        teens.insert(17, "seventeen");
        teens.insert(18, "eighteen");
        teens.insert(19, "nineteen");

        let mut tens = HashMap::new();
        tens.insert(2, "twenty");
        tens.insert(3, "thirty");
        tens.insert(4, "forty");
        tens.insert(5, "fifty");
        tens.insert(6, "sixty");
        tens.insert(7, "seventy");
        tens.insert(8, "eighty");
        tens.insert(9, "ninety");

        let mut quants = HashMap::new();
        quants.insert(2, "hundred");
        quants.insert(3, "thousand");
        quants.insert(6, "million");
        quants.insert(9, "billion");
        quants.insert(12, "trillion");
        quants.insert(15, "quadrillion");
        quants.insert(18, "quintillion");

        let mut ret_vec: Vec<String> = Vec::new();
        for (exp, n) in self.digits.iter().enumerate() {
            if exp % 3 == 0 {
                if exp > 1 &&
                    (*n != 0u8 || self.digits.len() > exp + 1 && self.digits[exp + 1] == 1)
                {
                    ret_vec.push(format!("{} ", quants.get(&exp).unwrap()));
                }

                if self.digits.len() > exp + 1 && self.digits[exp + 1] == 1 {
                    ret_vec.push(format!("{} ", teens.get(&(10 + n)).unwrap()));
                } else {
                    if *n != 0u8 || self.digits.len() == 1 {
                        ret_vec.push(format!("{} ", ones.get(n).unwrap()));
                    }
                }

            } else if exp % 3 == 1 {
                let spacer = match self.digits[exp - 1] {
                    x if x == 0 => " ",
                    _ => "-",
                };
                if *n > 1u8 {
                    ret_vec.push(format!("{}{}", tens.get(n).unwrap(), spacer));
                }
            } else if exp % 3 == 2 {
                if *n != 0u8 {
                    ret_vec.push(format!(
                        "{} {} ",
                        ones.get(n).unwrap(),
                        quants.get(&2).unwrap()
                    ));
                }
            }
        }

        ret_vec.reverse();
        let ret_str = ret_vec.iter().fold(String::new(), |acc, s| acc + s);
        ret_str.trim().to_owned()
    }
}
