mod base;
mod base_conversion;
mod md5;

use clap::{Arg, ArgGroup, Command};

fn main() {
    let matches = Command::new("Wutong")
        .version("0.1.0-alpha")
        .about("Wutong - A Swiss Army Knife of Developers.🌳")
        .author("Gavin Zheng<gav.zheng@outlook.com>")
        .subcommand(
            Command::new("bc")
                .about("Mathematical base conversion.")
                .arg(Arg::new("binary").long("bin").help("Binary number input."))
                .arg(Arg::new("octal").long("oct").help("Octal number input."))
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
        .subcommand(
            Command::new("base")
                .about("Base encoding and decoding.")
                .arg(
                    Arg::new("encode")
                        .short('e')
                        .long("encode")
                        .help("Encode the entered text."),
                )
                .arg(
                    Arg::new("decode")
                        .short('d')
                        .long("decode")
                        .help("Decode the entered text."),
                )
                .group(
                    ArgGroup::new("base_encoding_options")
                        .args(["encode", "decode"])
                        .required(true)
                        .multiple(false),
                ),
        )
        .subcommand(
            Command::new("md5")
                .about("MD5 hashing.")
                .arg(
                    Arg::new("text")
                        .short('t')
                        .long("text")
                        .help("Input text to be hashed."),
                )
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
        Some(("base", subcommand_base)) => match () {
            _ if subcommand_base.contains_id("encode") => {
                let result = base::text::encode::text_encode(
                    subcommand_base.get_one::<String>("encode").unwrap(),
                );
                println!("Base16: {} \nBase64: {}", result[0], result[1]);
            }
            _ if subcommand_base.contains_id("decode") => {
                let result = base::text::decode::text_decode(
                    subcommand_base.get_one::<String>("decode").unwrap(),
                );
                println!("Base16: {} \nBase64: {}", result[0], result[1]);
            }
            _ => panic!("Invalid option"),
        },
        Some(("md5", subcommand_md5)) => {
            let result = md5::text::md5_text(subcommand_md5.get_one::<String>("text").unwrap());
            println!("{}", result);
        }
        _ => {}
    }
}
