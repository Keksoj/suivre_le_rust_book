

pub fn salutation(name: &str) -> String {
    format!("Bonjour {}.", name)
}


#[cfg(test)]
mod tests {
    use super::*;

    // On veut tester que les noms passés à salutation() ressortent derrière
    #[test]
    fn salutation_contient_le_nom() {

        // On donne l'argument "Carol" à salutation() qui retourne la valeur
        // result
        let result = salutation("Carol");

        // On teste si ce résultat contient le mot "Carol"
        // contains() est une fonction qui renvoie une booléenne
        // Si contains() renvoie FALSE, assert! va afficher le message d'erreur
        // customisé. On peut même afficher des variables avec {}
        assert!(
            result.contains("Carol"),
            "salutation ne contenait pas le nom, ça contenait '{}'", result
        );
    }
}
