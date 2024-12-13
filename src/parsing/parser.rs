/*
    =BC
    Cree Knowledge B, C avec fact: true et insert dans la HashMap

    W => B
    Cree un Knowledge B avec en requirement W

    A | B => B
    Cree un Knowledge A avec en requiremement A OR B

    A | B => C | D
    Cree Knowledge C avec requirement A | B + C | D + C
    On check la condition de gauche, puis la condition de droite
    Si elles sont vrai, on check que la query initial soit vrai
    Ensuite, faire la meme avec D, A | B + C | D + D

    A <=> B
    Creer une nouvelle condition
    A => B
    B => A


    A | B <=> A

    A | B <=> A | B

    (A | B) + C => E
    Quand tu rencontres une parenthese (A | B) tu cree une knowledge (A | B)
    ensuite tu extraits A OR B et tu les mets en requirement de la knowledge (A | B)
    Cree la knowledge avec symbol (A | B) et en requirement tu mets A OR B
    Ensuite tu fais juste knowledge E requirement (A | B) AND C

    Y + C => (H + U)

    A | B => (Y + C)
    //
    A | B + (Y + C) + Y => Y
    A | B + (Y + C) + C => C
    ?C

*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

pub fn parse_file(file_path: &str) {
    match read_file(file_path) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
