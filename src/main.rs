use std::io;

fn main() {
    println!("Input atomic number: ");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

    let atomic_number_input: i32 = user_input.trim().parse().expect("Input is not a number.");

    if atomic_number_input > 118 {
        println!("INVALID ELEMENT");
        return;
    }

    println!("Electron Configuraton:");
    println!("{}", electron_configuration(atomic_number_input));
}

fn electron_configuration(atomic_number: i32) -> String {
    let sub = [
        "s", "s", "p", "s", "p", "s", "d", "p", "s", "d", "p", "s", "f", "d", "p", "s", "f", "d",
        "p", "f", "d", "f",
    ];
    let principal_quantum_number: [i32; 22] = [1, 2, 2, 3, 3, 4, 3, 4, 5, 4, 5, 6, 4, 5, 6, 7, 5, 6, 7, 6, 7, 7];
    let diff: i32;
    let mut electrons: [i32; 22] = [0; 22];
    let mut current_electron: i32 = 0;
    let mut l: usize = 0;
    let mut answer = String::new();

    while atomic_number != current_electron {
        match sub[l] {
            "s" => {
                current_electron += 2;
                electrons[l] += 2;
            }
            "p" => {
                current_electron += 6;
                electrons[l] += 6;
            }
            "d" => {
                current_electron += 10;
                electrons[l] += 10;
            }
            _ => {
                current_electron += 14;
                electrons[l] += 14;
            }
        }

        if current_electron > atomic_number {
            diff = current_electron - atomic_number;
            electrons[l] -= diff;
            break;
        }

        l += 1;
    }

    for i in 0..l + 1 {
        let electrons_string = electrons[i].to_string();
        let principal_quantum_string = principal_quantum_number[i].to_string();

        answer.push_str(&format!("{}{}{} ", principal_quantum_string, sub[i], electrons_string));
    }

    return answer;
}
