

pub fn build_proverb(input: Vec<&str>) -> String {
    let mut ret_string = String::new();
    if input.len() > 0 {
        for n in 0..(input.len() - 1) as usize {
            ret_string.push_str(
                format!(
                    "For want of a {} the {} was lost.\n",
                    input[n],
                    input[n + 1]
                ).as_str(),
            );
        }
        if input.len() > 2 {
            ret_string.push_str("And all for the want of a horseshoe nail.");
        } else {
            ret_string.push_str("And all for the want of a nail.");
        }
    }

    ret_string
}
