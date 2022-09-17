mod input;
mod job;
mod moyenne;

fn main() {
    let first_name = "Mamadou";
    let last_name = "Sarr";
    let age: i32 = 26;

    println!(
        "Hi my name is {firstname} {lastname} I am {age} years old. {job}",
        firstname = first_name,
        lastname = last_name,
        age = age,
        job = job::print_job("FullStack Web Developer")
    );
    moyenne::calc_moy();
}
