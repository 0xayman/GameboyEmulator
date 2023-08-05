use std::{fs::File, io::Read};

use crate::constants::constants;

pub struct Cart {
    entry: [u8; 4],
    logo: [u8; 0x30],
    title: [char; 16],
    new_lic_code: u16,
    sgb_flag: u8,
    cart_type: u8,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    old_lic_code: u8,
    version: u8,
    checksum: u8,
    global_checksum: u16,
    data: Vec<u8>,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            entry: [0; 4],
            logo: [0; 0x30],
            title: [' '; 16],
            new_lic_code: 0,
            sgb_flag: 0,
            cart_type: 0,
            rom_size: 0,
            ram_size: 0,
            dest_code: 0,
            old_lic_code: 0,
            version: 0,
            checksum: 0,
            global_checksum: 0,
            data: Vec::new(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        return self.data[address as usize];
    }

    pub fn write(&mut self, address: u16, data: u8) {
        panic!("UNSUPPORTED CART WRITE {:04X} Data: {:02X}", address, data)
    }

    pub fn load(&mut self, filename: &str) -> bool {
        println!("Loading cart: {}", filename);

        if let Ok(mut file) = File::open(filename) {
            if file.read_to_end(&mut self.data).is_ok() {
                self.parse_header();
                return true;
            }
        }
        false
    }

    fn parse_header(&mut self) {
        self.entry.copy_from_slice(&self.data[0x0100..0x0104]);
        self.logo.copy_from_slice(&self.data[0x0104..0x0134]);
        self.title
            .iter_mut()
            .zip(self.data[0x134..0x144].iter().map(|&c| c as char))
            .for_each(|(t, c)| *t = c);
        let raw_new_lic_code = &self.data[0x144..0x146];
        self.new_lic_code = (raw_new_lic_code[0] as u16) | ((raw_new_lic_code[1] as u16) << 8);
        self.sgb_flag = self.data[0x146];
        self.cart_type = self.data[0x147];
        self.rom_size = self.data[0x148];
        self.ram_size = self.data[0x149];
        self.dest_code = self.data[0x14A];
        self.old_lic_code = self.data[0x14B];
        self.version = self.data[0x14C];
        self.checksum = self.data[0x14D];
        let raw_global_checksum = &self.data[0x14E..0x150];
        self.global_checksum =
            (raw_global_checksum[0] as u16) | ((raw_global_checksum[1] as u16) << 8);

        println!("Title: {}", self.title.iter().collect::<String>());
        println!("ROM Type: {}", constants::get_rom_type_name(self.cart_type));
        println!(
            "Licensee Code: {}",
            constants::get_lic_name(self.old_lic_code)
        );
        println!("ROM Size: {}KB", 32 << self.rom_size);
        println!("RAM Size: {}KB", 8 << self.ram_size);

        let mut checksum: i32 = 0;
        for i in 0x0134..0x014D {
            checksum -= self.data[i] as i32 + 1;
        }

        if self.data[0x014D] != (checksum & 0xFF) as u8 {
            println!("Checksum Failed");
        } else {
            println!("Checksum Passed, checksum = {}", self.checksum);
        }
    }
}
