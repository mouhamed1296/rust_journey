use crate::input::get_input_f64;
use crate::input::get_line;

pub fn calc_moy() {
    println!("Entrez votre nom: ");
    let nom = get_line();
    println!("Entrez votre prenom: ");
    let prenom = get_line();
    println!("Entrez la premiere note");
    let note1: f64 = get_input_f64();
    println!("Entrez la deuxième note");
    let note2: f64 = get_input_f64();
    let moy: f64 = (note1 + note2) / 2.0;

    println!(
        "Bonjour {prenom} {nom} votre moyenne est {moy}",
        prenom = prenom,
        nom = nom,
        moy = moy
    );
}
