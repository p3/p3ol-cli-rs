use ascii_converter::*;

pub fn bin_to_dec(bin: &str) -> u8 {
    let bits: u32 = bin.to_string()
        .parse::<u32>()
        .unwrap();

    binary_to_decimal(&vec![bits])
        .unwrap()
        .first()
        .unwrap()
        .clone()
}

pub fn hex_to_dec(bytes: Vec<String>) -> u8 {
    hexadecimal_to_decimal(&bytes)
        .unwrap()
        .first()
        .cloned()
        .unwrap()
}

pub fn hex_to_bin(bytes: Vec<String>) -> Vec<String> {
    let binary: Vec<u32> = hexadecimal_to_binary(&bytes).unwrap();

    binary.into_iter()
        .map(|b: u32| format!("{:08}", b))
        .collect()
}
