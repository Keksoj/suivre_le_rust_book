// On crée le TRAIT Sommaire pour résumer le struct de texte, quel qu'il soit
pub trait Summary {

        // dans ce trait, on définit une MÉTHODE pour résumer le struct
    fn summarize(&self) -> String {

        // Ce code s'exécute par défaut SI AUCUNE MÉTHODE N'EST DÉFINIE dans
        // le bloc d'implémentation
        String::from("(lire plus...)")
    }
}

// Définition du struct NewsArticle
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// BLOC D'IMPLÉMENTATION
// On implémente le trait Summary sur le struct NewsArticle
impl Summary for NewsArticle {
    // Ici le code a été masqué pour l'exemple
    // Si ce bloc d'implémentation est vide, c'est le comportement par défaut
    // qui s'exécutera, celui qui écrit (en lire plus...)

    fn summarize(&self) -> String {
        format!(
            "{}, par {}, depuis {}",
            self.headline, self.author, self.location
        )
    }
}

// Définition du struct Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// On implémente Summary pour le struct Tweet
impl Summary for Tweet {

    fn summarize(&self) -> String {
        // en fait on concatène le nom d'utilisateur et le contenu avec format!
        // format! renvoie une String
        format!("{}: {}", self.username, self.content)
    }

}

// On définit la fonction notify(), qui appelle la méthode summarize() sur le
// paramètre 'item', ce paramètre est d'un type qui implémente le trait Summary
pub fn notify(item: impl Summary) {

    // Ici, on peut appeler n'importe quelle méthode qui vient du trait Summary
    println!("Breaking news! {}", item.summarize());
}

// TRAIT BOUNDS
// L'exemple ci-dessus fonctionne bien pour les petits bouts de code
// Ça s'écrit également comme ça :    (avec un nom différent pour la fonction)
// On utilise un paramètre de type générique
// Ce type doit obligatoirement être implémenté par Summary
// Cette syntaxe convient mieux pour le code complexe
pub fn notifier<T: Summary>(item:T) {
    println!("Des news ravageuses! {}", item.summarize());
}
