pub mod encoders;
pub mod metrics;
pub mod visualization;
use encoders::{ami, manchester, polar_nrz, unipolar_nrz};
use metrics::{calculate_average_power, calculate_dc_component, calculate_transitions};
use std::io::{self, Write};

fn main() {
    loop {
        println!("Line Coding Simulator");
        println!("---------------------");
        println!("1. Encode Binary String");
        println!("2. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Available Line Coding Schemes: 1.unipolar, 2.polar, 3.manchester, 4.ami");
                print!("Enter the scheme: ");
                io::stdout().flush().unwrap();

                let mut scheme = String::new();
                io::stdin().read_line(&mut scheme).unwrap();
                let scheme = scheme.trim().to_lowercase();
                let scheme = scheme.as_str();

                print!("Enter the binary string for {} (e.g., 10101): ", scheme);
                io::stdout().flush().unwrap();

                let mut binary_input = String::new();
                io::stdin().read_line(&mut binary_input).unwrap();
                let binary_input = binary_input.trim();

                if !binary_input.chars().all(|c| c == '0' || c == '1') {
                    println!("Error: Input must only contain 0s and 1s.");
                    continue;
                }

                let result = match scheme {
                    "unipolar" | "1" => unipolar_nrz(binary_input),
                    "polar" | "2" => polar_nrz(binary_input),
                    "manchester" | "3" => manchester(binary_input),
                    "ami" | "4" => ami(binary_input),
                    _ => {
                        println!("Error: Unsupported scheme!");
                        continue;
                    }
                };

                println!("Encoded Signal: {:?}", result);

                let avg_power = calculate_average_power(&result);
                let dc_component = calculate_dc_component(&result);
                let transitions = calculate_transitions(&result);

                println!("\nMetrics:");
                println!("Average Power: {:.2}", avg_power);
                println!("DC Component: {:.2}", dc_component);
                println!("Number of Transitions: {}\n", transitions);

                visualization::plot_signal(result, "target/signal.png");
                println!("Signal plot saved as 'signal.png'.\n");
            }
            "2" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter 1 or 2.\n"),
        }
    }
}
