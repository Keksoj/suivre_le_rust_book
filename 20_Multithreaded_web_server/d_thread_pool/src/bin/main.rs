// 2019-07-17
// Le même serveur, mais en multithread for the win.

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use d_thread_pool::ThreadPool;
use std::time::Duration;

fn main() {
    // On se connecte au port 7878 de l'hôte local (we bind to a port)
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Créons la piscine à fils ! Enfin, la thread pool
    let pool = ThreadPool::new(4);

    // On itère sur l'écouteur pour écouter chaque flux
    // On place une limite avec take(3) et puis on ferme le serveur
    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();
        // la méthode execute(), définie dans lib.rs, prend une closure comme
        // argument et l'envoie dans le canal
        // A l'autre bout du canal, les workers (wrapper pour thread) feront le boulot
        pool.execute(|| {
            // Cette closure ordonne à chaque worker d'appeler handle_connection,
            // ce que chaque worker fera bien gentiment.
            handle_connection(stream);
        });
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    // Lire le flux
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // Reconnaître une requête GET et inventer une requête /sleep
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Selon que la requête soit GET ou /sleep
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        println!("On a reçu une requête sleep, du coup on attend 5 secondes");
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
