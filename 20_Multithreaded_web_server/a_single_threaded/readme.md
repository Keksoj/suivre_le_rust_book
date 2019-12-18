## Lisons la requête HTTP

Voir le code dans `src/main.rs`, et plus précisément:

```rust
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

Quand on se connecte à `127.0.0.1:7878` depuis mon firefox, ça nous imprime:

```
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:67.0) Gecko/20100101 Firefox/67.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3
Accept-Encoding: gzip, deflate
DNT: 1
Connection: keep-alive
Upgrade-Insecure-Requests: 1
```

Les requêtes HTTP sont faites de texte, et on ce format:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

Voyons la première ligne:
* La méthode, c'est soit `GET` (ce qui est notre cas) soit `POST`.
* l'URI de la requête, c'est `/` dans notre cas.
* ensuite, la version d'http utilisée par le client
* `CRLF` signifie *carriage return and line feed*, soit "retour de chariot et nouvelle ligne". Des termes qui datent de l'époque des machines à écrire ! `\r` pour *carriage return* et `\n` pour *nouvelle ligne*.

Toutes ces lignes suivantes sont des `headers`, des en-têtes:
* Host:
* Use-Agent:
* Accept:
* Accept-Language:
* Accept-Encoding:
* DNT:
* Connection:
* Upgrade-Insecure-Requests:

Pas de `message-body` pour une requête HTTP !

## Envoyons une réponse

Les réponses HTTP ont ce format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

La première ligne est une `status line` qui contient:
* un code numérique, le `status code`, qui résume le résultat de la requête
* une `reason phrase` qui fournit un équivalent texte du `status code`

Par exemple:

```
HTTP/1.1 200 OK\r\n\r\n
```

* Version 1.1
* `status code` de 200, qui indique le succès
* `reason phrase` qui dit OK

Écrivons ça dans le code:

```rust
let response = "HTTP/1.1 200 OK\r\n\r\n"; // type: &str
stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```
