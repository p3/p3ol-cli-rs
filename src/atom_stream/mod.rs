use ascii_converter::*;
use fancy_regex::Regex;
use std::collections::VecDeque;
use std::str;

mod atom_name;

use p3ol_cli_rs::util::*;

pub struct Atom {
    pub protocol_id: u8,
    pub name: String,
    pub data: String,
}

pub struct AtomStream {
    pub atoms: Vec<Atom>,
}

impl AtomStream {
    pub fn new() -> Self {
        AtomStream { atoms: Vec::new() }
    }

    pub fn parse(&mut self, packet: &str) -> &mut Self {
        let separator: Regex = Regex::new(r"\K5a(.*?)(0d(?=5a)|0d$)").unwrap();

        for capture in separator.captures_iter(packet) {
            let packet: &str = capture.expect("No capture found").get(0).unwrap().as_str();
            let packet_bytes: Vec<String> = Self::to_bytes(packet);

            let token: Option<String> = Self::token(&packet_bytes);

            if token.is_some() && ["at", "At", "AT"].contains(&token.unwrap().as_str()) {
                self.parse_atoms(&packet_bytes);
            }
        }

        return self;
    }

    fn parse_atoms(&mut self, packet_bytes: &Vec<String>) {
        let mut bytes: VecDeque<String> = VecDeque::from(Self::payload(&packet_bytes));

        while bytes.is_empty() == false {
            let starting_byte: String = bytes.pop_front().unwrap();

            if bytes.is_empty() {
                break;
            }

            match Self::atom_style(&starting_byte) {
                0 => self.parse_full_style(&starting_byte, &mut bytes),
                _ => panic!("atom stye parsing is not implemented"),
            };
        }
    }

    fn payload(packet: &Vec<String>) -> Vec<String> {
        match Self::token(packet).unwrap().as_str() {
            "AT" => packet[12..].to_vec(),
            "At" => packet[13..].to_vec(),
            "at" => packet[14..].to_vec(),
            _ => packet.clone(),
        }
    }

    fn parse_full_style(&mut self, starting_byte: &str, bytes: &mut VecDeque<String>) {
        let protocol_bin: Vec<String> = hex_to_bin(vec![starting_byte.to_string()]);
        let protocol_id: u8 = bin_to_dec(&protocol_bin.first().unwrap()[3..=7]);

        let atom_byte: String = bytes.pop_front().unwrap();
        let atom_number: u8 = hex_to_dec(vec![atom_byte.to_string()]);

        let length_byte: String = bytes.pop_front().unwrap();
        let sizeof_args_bin: Vec<String> = hex_to_bin(vec![length_byte.to_string()]);
        let sizeof_args: u8 = sizeof_args_bin.first().unwrap()[0..1]
            .to_string()
            .parse::<u8>()
            .unwrap();

        let args_len: u8 = if sizeof_args == 0 {
            bin_to_dec(&sizeof_args_bin.first().unwrap()[1..])
        } else {
            let next_byte: String = bytes.pop_front().unwrap();
            let next_byte_bin: Vec<String> = hex_to_bin(vec![next_byte.to_string()]);

            let mut sizeof_args_bits: String = sizeof_args_bin.first().unwrap()[1..].to_owned();
            sizeof_args_bits.push_str(next_byte_bin.first().unwrap());

            bin_to_dec(&sizeof_args_bits)
        };

        let data: Vec<String> = if args_len > 0 {
            bytes.drain(..args_len as usize).collect::<Vec<String>>()
        } else {
            Vec::new()
        };

        let atom: Atom = Atom {
            protocol_id: protocol_id,
            name: atom_name::from(&protocol_id, &atom_number).to_string(),
            data: hexadecimal_to_string(&data).unwrap(),
        };

        self.atoms.push(atom);
    }

    fn atom_style(byte: &str) -> u8 {
        let bits: Vec<String> = hex_to_bin(vec![byte.to_string()]);

        bin_to_dec(&bits.first().unwrap()[0..3])
    }

    fn token(packet: &Vec<String>) -> Option<String> {
        if packet.len() < 10 {
            None
        } else {
            Some(hexadecimal_to_string(&packet[8..10].to_vec()).unwrap())
        }
    }

    fn to_bytes(packet: &str) -> Vec<String> {
        packet
            .as_bytes()
            .chunks(2)
            .map(|chunk: &[u8]| str::from_utf8(chunk).unwrap().to_string())
            .collect()
    }
}
