use std::{error, path::Path, fs::File, io::Write};

mod utils;

pub fn run() {
    let (script, pages)= get_script();

    println!("====================");
    println!("{}", script);
    println!("====================");
    println!("Bitte füge das Skript oben (ohne die Gleichheitszeichen) in die Developer-Konsole des Buches ein.");
    println!("Es sollte einige Dateien heruntergeladen werden.");

    utils::read_line("Warte, bis alle Dateien heruntergeladen worden sind, und drücke dann Enter");

    let folder_path = utils::read_line("Gib nun den Dateiname des Ordners an, in dem die .svg Dateien sind: ");
    let folder_path = Path::new(&folder_path);

    println!("Konvertiere...");

    for i in 1..pages + 1 {
        let path = folder_path.join(format!("{}.svg", i)).to_str().unwrap().to_string();
        let out_path: String = folder_path.join(format!("{}.pdf", i)).to_str().unwrap().to_string();

        let file_contents = svg_to_pdf(&path).unwrap();
        let mut file = File::create(out_path).unwrap();
        file.write(&file_contents).unwrap();

        println!("{}/{}", i, pages + 1);
    }

    println!("Fertig!")
}

fn get_script() -> (String, isize) {
    let mut script: String = include_str!("../downloader.js").to_string();

    let number: isize;

    loop {
        match utils::read_line("Anzahl Seiten: ").parse::<isize>() {
            Ok(num) => {
                number = num;
                break;
            },
            Err(_) => println!("Der eingegebene Text konnte nicht als Zahl interpretiert werden. Bitte versuche es erneut"),
        }
    }

    script = script.replace("NaN", &number.to_string());

    (script, number)
}

fn svg_to_pdf(path: &str) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let svg = std::fs::read_to_string(path)?;
    let pdf = svg2pdf::convert_str(&svg, svg2pdf::Options::default())?;

    Ok(pdf)
}