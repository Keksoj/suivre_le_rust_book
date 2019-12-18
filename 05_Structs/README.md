### Chapitre 5 : Les Structs

Bonjour.

Les structs ou structures sont un type de donnée qui permettent de nommer et regrouper des valeurs dans... bah, dans un struct.
Ça ressemble mais ce n'est pas la même chose qu'un n-uplet.
Avec les **enums**, les **structs** sont les outils à préférer pour gérer la donnée dans Rust.

######Différence avec un n-uplet
On donne un nom à chaque variable, voilà la différence.
On définit un struct comme ça : `struct Nomdustruct {}` et entre accolades on définit chaque variable ainsi : `nom: type,`. Exemple avec le struct User :

```rust
struct User {
    username: String,
    email: String,
    age: u8,
    active: bool,
}
```

Utiliser ce struct avec des valeurs concrètes dedans, c'est en créer une **instance**. On fait `let nom = Struct {}` Ici, `sophie` est une instance du struct User.

```rust
let sophie = User {
    age: 25,
    username: String::from("Sophie"),
    active: true,
    email: String::from("sophie.lambert@skynet.com"),
}
```
On peut mettre les variables dans le désordre, c'est plutôt pratique. Pas d'index, seulement les *noms* de variables.

Pour utiliser une variable, rien de plus simple. `sophie.email` va retourner la chaîne de caractères `sophie.lamber@skynet.com`.

On peut créer une instance modifiable avec `let mut nom = Struct {}`. Exemple :


```rust
let mut jean = User {
    age: 30,
    username: String::from("Jean"),
    active: true,
    email: String::from("jean.ai.marre@metatron.com"),
}
```
On peut modifier cette instance avec cette notation :
```
jean.email = String::from("autre.email.de.jean@metatron.com");
```
Ne pas oublier le point virgule `;` ! C'est une déclaration.

###### Créer un struct avec une fonction
On peut définir une fonction qui construit un struct :
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        age: 1,
    }
}
```
qui se simplifie, avec la syntaxe `field init shorthand`, pour donner ceci :
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        age: 1,
    }
}
```
