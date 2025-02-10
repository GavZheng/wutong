pub fn binary(input: &str) -> [String; 3] {
    convert_base(input, 2, [8, 10, 16])
}

pub fn octal(input: &str) -> [String; 3] {
    convert_base(input, 8, [2, 10, 16])
}

pub fn decimal(input: &str) -> [String; 3] {
    convert_base(input, 10, [2, 8, 16])
}

pub fn hexadecimal(input: &str) -> [String; 3] {
    convert_base(input, 16, [2, 8, 10])
}

fn convert_base(input: &str, input_radix: u32, output_radixes: [u32; 3]) -> [String; 3] {
    let value = u64::from_str_radix(input, input_radix).unwrap_or(0);

    output_radixes.map(|radix| match radix {
        2 => format!("{:b}", value),
        8 => format!("{:o}", value),
        10 => value.to_string(),
        16 => format!("{:x}", value),
        _ => unreachable!(),
    })
}
