

pub fn reply(sentence: &str) -> &'static str {
    if sentence.trim().is_empty() {
        return "Fine. Be that way!";
    }


    let filtered: Vec<char> = sentence.trim().chars().filter(|c| c.is_alphabetic()).collect();
    if !filtered.is_empty() && filtered.iter().all(|c| c.is_uppercase()) {
        return "Whoa, chill out!";
    }

    if sentence.trim().ends_with("?") {
        return "Sure.";
    }


    "Whatever."
}
