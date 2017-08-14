

pub fn verse(n: u32) -> String {
    let wall: &str = "of beer on the wall";
    let last: &str = "Take it down and pass it around,";
    let mid: &str = "Take one down and pass it around,";
    let store: &str = "Go to the store and buy some more,";

    let mut ret_string = String::new();

    if n == 0 {
        ret_string.push_str("No more bottles ");
        ret_string.push_str(wall);
        ret_string.push_str(", no more bottles of beer.\n");
        ret_string.push_str(store);
        ret_string.push_str(" 99 bottles ");
        ret_string.push_str(wall);
        ret_string.push_str(".\n");
    } else if n == 1 {
        ret_string.push_str(
            format!(
                "{} bottle {}, {} bottle of beer.\n{} no more bottles {}.\n",
                n,
                wall,
                n,
                last,
                wall
            ).as_str(),
        );
    } else if n == 2 {
        ret_string.push_str(
            format!(
                "{} bottles {}, {} bottles of beer.\n{} 1 bottle {}.\n",
                n,
                wall,
                n,
                mid,
                wall
            ).as_str(),
        );
    } else {
        ret_string.push_str(
            format!(
                "{} bottles {}, {} bottles of beer.\n{} {} bottles {}.\n",
                n,
                wall,
                n,
                mid,
                n - 1,
                wall
            ).as_str(),
        );
    }

    ret_string
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ret_string = String::new();
    let mut i = start;
    while i >= end {
        ret_string.push_str(verse(i).as_str());
        if i > end {
            ret_string.push_str("\n");
        }
        if i == 0 {
            break;
        }

        i -= 1;
    }
    ret_string
}
