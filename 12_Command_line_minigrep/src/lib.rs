// Ici on met tout ce qui n'est pas essentiel dans main(). Ça sera plus manipulable.

use std::error::Error;
use std::fs; // pour gérer les fichiers


// Le struct Config sert à grouper les deux arguments d'une façon lisible pour
// les développeurs qui reliront le code. Ils sont faciles à mobiliser.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,    // le premier argument, c'est le terme recherché
    pub filename: String, // et le deuxième argument, le nom de fichier
    pub case_insensitive: bool,
}

impl Config {

    pub fn nouveau(matches: clap::ArgMatches<'_>) -> Result<Config, &'static str> {

        let query: String = matches.value_of("terme").unwrap().to_string();
        let filename: String = matches.value_of("file").unwrap().to_string();
        let case_insensitive = matches.is_present("case-insensitive");
        Ok(Config { query, filename, case_insensitive })
    }
}

// La fonction run() prend comme argument... les arguments de la commande, dans
// leur version soignée, une instance du struct Config.
// C'est elle qui va faire tout ce qui nous intéresse.
// Elle renvoie soit (), soit Bx<dyn Error>, un TRAIT OBJECT, voir chap. 17
// Ça nous permet de renvoyer l'erreur qu'on veut (donc dynamique)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // lisons le contenu du fichier avec std::fs::read_to_string()
    // Noter le ? à la fin de la ligne, il renvoie l'erreur de read_to_string()
    let contenu = fs::read_to_string(config.filename)?;

    // dans le struct Config, case_sensitive est une booléenne, ça sert de
    // condition pour if
    // On fait tourner les fonctions de recherche et on attribue les valeurs
    // retournées au vecteur 'results'
    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contenu)
    } else {
        search_case_sensitive(&config.query, &contenu)
    };

    for line in results {
        println!("{}", line);
    }

    // Renvoyer (), emballé dans Ok()
    Ok(())
}

// la fonction search() parcourt du contenu à la recherche du terme
// Plusieurs arguments, une valeur retour ? Il faut des lifetimes !
fn search_case_sensitive<'a>(query: &str, contenu: &'a str) -> Vec<&'a str> {

    // créons un vecteur qui contiendra les résultats
    let mut results = Vec::new();

    // la fonction lines() est un ITÉRATEUR
    for line in contenu.lines() {

        // Regarde-moi ce code qui se lit comme du shakespeare :
        if line.contains(query) {
            results.push(line);
        }
    }
    // Renvoyons le vecteur
    results
}

// La même fonction, non sensible à la casse grâce à la méthode to_lowercase()
// Rédigé selon le chapitre 13.3, en programmation fonctionnelle
// L'intérêt est de réduire la quantité de variables modifiables et mutables
fn search_case_insensitive<'a>(query: &str, contenu: &'a str) -> Vec<&'a str> {

    // On passe le terme recherché en minuscules
    // Important : query doit rester une &str, sinon elle ne passe pas dans la
    // méthode contains() vue plus bas.
    let query = &query.to_lowercase();

    // Ce code utilise des itérateurs, à comparer avec la fonction
    // search_case_sensitive() plus haut.

    // lines() est un itérateur
    contenu.lines()
        // filter() crée un itérateur à partir d'une fonction, une fermeture,
        // qui ne laisse passer que les lignes où contains() renvoie TRUE
        .filter(|line| line.to_lowercase().contains(query))
        .collect() // on repackage le tout
}

#[cfg(test)]
mod tests {

    use super::*;

    // Test Driven Developpment.
    // On crée d'abord la fonction case_sensitive, ensuite on rédige le code.
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    // La même, mais qui ne tient pas compte de la casse
    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}
