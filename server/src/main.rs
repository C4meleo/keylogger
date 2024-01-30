use std::borrow::Cow;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Fonction pour créer un fichier output.txt ou l'ouvrir s'il existe déjà
fn create_file(filepath: &str) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(filepath)
}

fn remove_newlines(file_content: Cow<'_, str>) -> Cow<'_, str> {
    // Retire les sauts de ligne à la fin de chaque ligne
    let modified_content = file_content
        // Divise le contenu du fichier par lignes
        .lines()
        // Supprime les espaces à la fin de chaque ligne
        .map(|line| line.trim_end())
        // Collecte les lignes modifiées dans une seule chaîne de caractères
        .collect::<String>();
    // Retourne le résultat sous forme de Cow::Owned
    Cow::Owned(modified_content)
}

fn remove_quotes(file_content: Cow<'_, str>) -> Cow<'_, str> {
    // Supprime les guillemets doubles, gère les échappements "\" et ajoute un saut de ligne à la fin de chaque ligne
    let modified_content = file_content
        // Divise le contenu du fichier par lignes
        .lines()
        .map(|line| {
            let mut modified_line = line
                // Remplace \" par une chaîne temporaire
                .replace("\\\"", "<TEMP_STRING>")
                // Supprime les guillemets doubles restants
                .replace('\"', "")
                // Restaure \" à sa forme d'origine
                .replace("<TEMP_STRING>", "\"");
            modified_line.push('\n'); // Ajoute un saut de ligne à la fin de chaque ligne
            modified_line
        })
        // Collecte les lignes modifiées dans une seule chaîne de caractères
        .collect::<String>();
    // Retourne le résultat sous forme de Cow::Owned
    Cow::Owned(modified_content)
}

fn replace_backslash_n(file_content: Cow<'_, str>) -> Cow<'_, str> {
    // Remplace les occurrences de "\\n" par des sauts de ligne "\n"
    let modified_content = file_content.replace("\\n", "\n");
    // Retourne le résultat sous forme de Cow::Owned
    Cow::Owned(modified_content)
}

// Fonction pour gérer une connexion TCP entrante
fn handle_client(mut stream: TcpStream, file_path: &str) {
    // Tampon pour stocker les données lues du flux
    let mut buffer = [0; 1024];
    // Lecture des données du flux dans le tampon
    let size = stream.read(&mut buffer).unwrap();
    // Extraction du contenu du tampon en tant que chaîne UTF-8
    let file_content = &buffer[..size];
    let mut file_string = String::from_utf8_lossy(file_content);
    // Création ou ouverture du fichier output.txt
    let mut output = create_file(file_path).unwrap();
    file_string = replace_backslash_n(remove_newlines(remove_quotes(file_string)));
    // Écriture du contenu du fichier dans output.txt
    if let Err(err) = writeln!(output, "{}", file_string) {
        eprintln!("Failed to write to output file: {}", err);
    }
    // Affichage du contenu reçu dans la console
    print!("Contents of received file\n{}", file_string);
}

fn main() {
    // Création d'un serveur TCP qui écoute sur l'adresse 127.0.0.1:7070
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();
    let file_path = "output.txt";
    // Boucle principale pour accepter les connexions entrantes
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Affichage d'un message lorsque le client est connecté
                println!("Client connected");
                // Appel de la fonction handle_client pour gérer la connexion
                handle_client(stream, file_path);
            }
            Err(e) => {
                // Affichage d'un message en cas d'erreur lors de l'acceptation de la connexion
                println!("Failed accepting connection: {}", e);
            }
        }
    }
}
