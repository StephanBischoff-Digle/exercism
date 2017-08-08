pub fn is_leap_year(year: u16) -> bool {
    let a = year % 4 == 0;
    let b = year % 100 == 0;
    let c = year % 400 == 0;
    a && !b || a && c
}
