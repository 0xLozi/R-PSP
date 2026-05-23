mod kirk_core;
use std::{fs};
mod offset_keys;
mod tag_info;
mod analysis;
mod getters;
mod decryption;

const SIZE_MAGIC_NUMBER: usize = 4;
const ELF_REPRESENTATION: [u8;4] = [0x7F, 0x45, 0x4C, 0x46];

fn main() -> Result<(), String> {
    // Primero leo los primeros 4 bytes: si dicen ELF seguimos
    let ruta: &str = "/home/snake/Downloads/lego_batman_game/PSP_GAME/SYSDIR/EBOOT.BIN";

    println!("Intentando cargar: {}", ruta);

    // Cargamos TODO el archivo crudo en un vector dinámico de bytes (Vec<u8>)
    let boot_binario: Vec<u8> = fs::read(ruta)
                                .map_err(|e| e.to_string())?;


    analysis::verificar_extension_binario(&boot_binario);
    println!("\n");
    analysis::verificar_size_archivo(&boot_binario);

    // decrypt_payload(&boot_binario);
    decryption::another_test(&boot_binario);

    Ok(())

}