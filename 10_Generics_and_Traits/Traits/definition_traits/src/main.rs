// 2019-02-16

// Cours sur les traits. On va suivre tranquillement parce que je comprends pas.
// Suivons le cours et réalisons des structs de texte : un article,

// Voir lib.rs pour la définition du trait et des structs

use definition_traits::{NewsArticle, Summary, Tweet, notify, notifier};

fn main() {
    // Créons une instance de Tweet
    let montweet = Tweet {
        username: String::from("Hal Finney"),
        content: String::from("Running Bitcoin."),
        reply: false,
        retweet: false,
    };

    // appliquer la méthode summarize
    println!("{}", montweet.summarize());

    let article = NewsArticle {
        headline: String::from("Des pinguins en libertés !"),
        location: String::from("Paris"),
        author: String::from("René Girard"),
        content: String::from("Les pompiers seront chargés de jouer aux billes avec eux."),
    };

    println!("Un nouvel article {}", article.summarize());
    notify(article);
    notifier(montweet);
}
