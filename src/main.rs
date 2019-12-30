extern crate winapi;
extern crate colored;

use colored::*;

mod process;
mod memory;

const OFFSET_DMG: u64 = 0x14187EE8B; // 8B 8A 50 01 00 00 49 8B 50 20 E9
const WINDOW_NAME: &str = "star wars battlefront ii";

fn main() {
    println!("{} - {}", "Tatooine".red(), "Star Wars Battlefront 2".green());

    let process_handle = process::get_process(WINDOW_NAME.to_string());
    
    loop {
        println!("Enter damage multiplier: ");
        let multiplier = get_input().trim().parse::<i32>().unwrap_or(0);

        let mut shellcode = generate_multiplier_shellcode(multiplier);
        match memory::write_mem::<[u8; 6]>(process_handle, OFFSET_DMG, &mut shellcode) {
            Ok(v) => println!("{} bytes written to game memory", v),
            Err(e) => println!("Failed with error code: {}", e)
        };
    }
}

fn generate_multiplier_shellcode(multiplier: i32) -> [u8; 6] {
    let b1: u8 = ((multiplier >> 24) & 0xff) as u8;
    let b2: u8 = ((multiplier >> 16) & 0xff) as u8;
    let b3: u8 = ((multiplier >> 8) & 0xff) as u8;
    let b4: u8 = (multiplier & 0xff) as u8;

    // little endian
    return [0xb9, b4, b3, b2, b1, 0x90];
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}
