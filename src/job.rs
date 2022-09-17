pub fn print_job(job_title: &str) -> String {
    let mut job = String::from("I am a ");
    job.push_str(job_title);

    return job;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job() {
        assert_eq!(print_job("Web Developer"), "I am a Web Developer")
    }
}
