use std::f32::RADIX;

use lazy_static::lazy_static;
use regex::Regex;
use slint::{include_modules, SharedString};

include_modules!();

lazy_static! {
    static ref VALID_EXPRESSION: Regex = Regex::new(r"(\+|-|\*|\/)[0-9]+").unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    // TASK: Adaugă logica pentru `on_add_to_text_area`, pentru a manevra cazurile "C", "=", și alte input-uri
    ui.on_add_to_text_area(move |current_text, new_input| {
        let ui = ui_handle.unwrap();
        
        match new_input.as_str() {
            "=" => {
                let result = evaluate(&current_text);
                ui.set_textarea(SharedString::from(result));
            }
            "Clear" => {
                let clear = SharedString::from("");
                ui.set_textarea(clear);
            }
            _ =>  {
                let mut textarea = ui.get_textarea();
                textarea.push_str(&new_input);
                ui.set_textarea(textarea);
            }
        }
        // TASK: Adaugă logica pentru cazurile:
        // - "C": Golirea zonei de text
        // - "=": Calcularea rezultatului și afișarea acestuia
        // - Altele: Adăugarea input-ului curent la zona de text
        // HINT: Folosește un `match` pentru a verifica valoarea `new_input`.
    });

    ui.run()
}

// TASK: Completează funcția `evaluate`, care verifică validitatea expresiei și returnează rezultatul sau un mesaj de eroare
// HINT: Folosește regex-ul `VALID_EXPRESSION` pentru a verifica dacă `input` este o expresie validă.
// Dacă expresia este validă, apelează funcția `compute`. Dacă nu, returnează un mesaj de eroare, cum ar fi "Invalid Expression".
fn evaluate(input: &str) -> String {
    match VALID_EXPRESSION.is_match(input) {
        true => {
            match compute(input) {
                Some(u) => u.to_string(),
                None => input.to_string()
            }
        }
        false => {
            println!("Invalid Expression");
            input.to_string()
        }
    }
}

// TASK: Implementează funcția `compute` pentru a realiza operațiile de bază (+, -, *, /) și a returna rezultatul
// HINT: Parcurge simbolurile de operare și folosește `.split()` pentru a împărți `input` în două părți: înainte și după simbol.
// Convertește fiecare parte în `f64` și returnează rezultatul în funcție de simbol.
fn compute(input: &str) -> Option<f64> {
    // TASK: Inițializează simbolurile de operare (+, -, *, /)
    // HINT: Creează o listă `let symbols = ["+", "-", "*", "/"];`
    // let mut result: f64 = 0.0;
    // let mut aux: f64 = 0.0;
    // let mut current_number = 0.0;
    // let mut decimals = false;
    // let mut power: f64 = 1.0;
    let symbols = ['+', '-', '*', '/'];
    let mut delimiter = 'a';
    for c in input.chars() {
        if symbols.contains(&c) {
            delimiter = c;
            break;
        }
    }
    let v: Vec<&str> = input.split(delimiter).collect();
    let nr1_chars = v[0].chars();
    let mut nr1: f64 = 0.0;
    for ch in nr1_chars {
        nr1 = nr1 * 10.0 + ch.to_digit(RADIX).unwrap() as f64;
    }
    let nr2_chars = v[1].chars();
    let mut nr2: f64 = 0.0;
    for ch in nr2_chars {
        nr2 = nr2 * 10.0 + ch.to_digit(RADIX).unwrap() as f64;
    }
    match delimiter {
        '+' => {
            return Some(nr1 + nr2);
        }
        '-' => {
            return Some(nr1 - nr2);
        }
        '*' => {
            return Some(nr1 * nr2);
        }
        '/' => {
            return Some(nr1 / nr2);
        }
        _ => {
            return None;
        }
    }
}
