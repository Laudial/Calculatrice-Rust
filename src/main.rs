use std::io;

fn main() {
    loop {
        println!("Entrez la première opérande:");
        let a = read_number();

        println!("Entrez l'opérateur (+, -, *, /):");
        let operator = read_operator();

        println!("Entrez la seconde opérande:");
        let b = read_number();

        let result = match operator.as_str() {
            "+" => Calculator::add(a, b),
            "-" => Calculator::subtract(a, b),
            "*" => Calculator::multiply(a, b),
            "/" => Calculator::divide(a, b),
            _ => {
                println!("Opérateur invalide");
                continue;
            }
        };

        println!("Le résultat de l'opération est: {}", result);
    }
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().parse().expect("Entrez un nombre valide")
}

fn read_operator() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

struct Calculator;

impl Calculator {
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    fn divide(a: f64, b: f64) -> f64 {
        if b != 0.0 {
            a / b
        } else {
            println!("Erreur: division par zéro");
            0.0
        }
    }
}
