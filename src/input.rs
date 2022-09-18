use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub fn get_line() -> String {
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("have read , {} bytes", b1);

    return line;
}

pub fn trim_newline(mut line: String) -> String {
    if line.ends_with("\n") {
        line.pop();
    }
    if line.ends_with("\r") {
        line.pop();
    }
    return line;
}

pub struct E;

pub fn parse_input<T: FromStr + Display + Debug>(line: String) -> Result<T, E> {
    match trim_newline(line).parse::<T>() {
        Ok(i) => {
            let n = i;
            return Ok(n);
        }
        Err(_) => Err(E),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line() {
        let line = get_line();
        assert_eq!(line, "sarr\n");
        assert_ne!(line, "sarr");
    }

    #[test]
    fn test_parse_input() {
        let a = parse_input::<f64>(String::from("2")).unwrap_or(0.0);
        let b = parse_input::<f64>(String::from("2")).unwrap_or(0.0);
        let c = a / b;
        assert_eq!(c, 1.0);
        assert_eq!(
            parse_input::<f64>(String::from("12.5")).unwrap_or(0.00),
            12.5
        );
    }
}
