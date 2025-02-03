mod base_conversion;

use clap::{Arg, ArgGroup, Command};

fn main() {
    let matches = Command::new("Wutong")
        .version("0.1.0-alpha")
        .about("Wutong - A Swiss Army Knife of Developers.🌳")
        .author("Gavin Zheng<gav.zheng@outlook.com>")
        .subcommand(
            Command::new("bc")
                .about("Mathematical base conversion.")
                .arg(
                    Arg::new("binary")
                        .long("bin")
                        .help("Binary number input."),
                )
                .arg(
                    Arg::new("octal")
                        .long("oct")
                        .help("Octal number input."),
                )
                .arg(
                    Arg::new("decimal")
                        .long("dec")
                        .help("Decimal number input."),
                )
                .arg(
                    Arg::new("hexadecimal")
                        .long("hex")
                        .help("Hexadecimal number input."),
                )
                .group(
                    ArgGroup::new("base_conversion_options")
                        .args(["binary", "octal", "decimal", "hexadecimal"])
                        .required(true)
                        .multiple(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("bc", subcommand_bc)) => {
            match () {
                _ if subcommand_bc.contains_id("binary") => {
                    let result = base_conversion::math::binary(
                        subcommand_bc.get_one::<String>("binary").unwrap(),
                    );
                    println!(
                        "Binary:      {} \nOctal:       {} \nDecimal:     {} \nHexadecimal: {}",
                        subcommand_bc.get_one::<String>("binary").unwrap(), // Binary
                        result[0],                                          // Octal
                        result[1],                                          // Decimal
                        result[2]                                           // Hexadecimal
                    )
                }
                _ if subcommand_bc.contains_id("octal") => {
                    let result = base_conversion::math::octal(
                        subcommand_bc.get_one::<String>("octal").unwrap(),
                    );

                    println!(
                        "Binary:      {}\nOctal:       {}\nDecimal:     {} \nHexadecimal: {}",
                        result[0],                                         // Binary
                        subcommand_bc.get_one::<String>("octal").unwrap(), // Octal
                        result[1],                                         // Decimal
                        result[2]                                          // Hexadecimal
                    );
                }
                _ if subcommand_bc.contains_id("decimal") => {
                    let result = base_conversion::math::decimal(
                        subcommand_bc.get_one::<String>("decimal").unwrap(),
                    );
                    println!(
                        "Binary:      {}\nOctal:       {}\nDecimal:     {}\nHexadecimal: {}",
                        result[0],                                           // Binary
                        result[1],                                           // Octal
                        subcommand_bc.get_one::<String>("decimal").unwrap(), // Decimal
                        result[2]                                            // Hexadecimal
                    );
                }
                _ if subcommand_bc.contains_id("hexadecimal") => {
                    let result = base_conversion::math::hexadecimal(
                        subcommand_bc.get_one::<String>("hexadecimal").unwrap(),
                    );
                    println!(
                        "Binary:      {}\nOctal:       {}\nDecimal:     {}\nHexadecimal: {}",
                        result[0],                                               // Binary
                        result[1],                                               // Octal
                        result[2],                                               // Decimal
                        subcommand_bc.get_one::<String>("hexadecimal").unwrap()  // Hexadecimal
                    )
                }
                _ => panic!("Invalid base conversion option"),
            };
        }
        _ => {}
    }
}
