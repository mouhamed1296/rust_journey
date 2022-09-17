pub fn get_line(mut line: String) -> String {
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("have read , {} bytes", b1);

    return line;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line() {
        let mut line = String::new();
        println!("Entrez votre nom: ");
        line = get_line(line);
        assert_eq!(line, "sarr\n");
        assert_ne!(line, "sarr");
    }
}
