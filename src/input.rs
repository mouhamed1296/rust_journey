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
    while line.ends_with("\n") || line.ends_with("\r") {
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

pub fn get_input_i64() -> i64 {
    return parse_input::<i64>(get_line()).unwrap_or(0);
}

pub fn get_input_i32() -> i32 {
    return parse_input::<i32>(get_line()).unwrap_or(0);
}

pub fn get_input_u32() -> u32 {
    return parse_input::<u32>(get_line()).unwrap_or(0);
}

pub fn get_input_u64() -> u64 {
    return parse_input::<u64>(get_line()).unwrap_or(0);
}

pub fn get_input_f32() -> f32 {
    return parse_input::<f32>(get_line()).unwrap_or(0.0);
}

pub fn get_input_f64() -> f64 {
    return parse_input::<f64>(get_line()).unwrap_or(0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_newline() {
        assert_eq!(trim_newline(String::from("line1\n")), "line1");
        assert_eq!(trim_newline(String::from("line1\r")), "line1");
        assert_eq!(trim_newline(String::from("line1\r\n")), "line1");
        assert_eq!(trim_newline(String::from("line1\n\r")), "line1");
        assert_eq!(trim_newline(String::from("line1\n\r\r\n\n\n\n\r")), "line1");
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
