use crate::input;

pub fn calc_moy() {
    let mut nom = String::new();
    println!("Entrez votre nom");
    nom = input::get_line(nom);
    println!("Votre nom est {}", nom);
}
