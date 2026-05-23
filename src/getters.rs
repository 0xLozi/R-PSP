use crate::offset_keys;
use crate::kirk_core;


pub fn get_tag(boot_binario: &[u8]) -> Option<u32> {
    let offset_tag = 0xD0;

    let tag_bytes: [u8;4] = match boot_binario[offset_tag .. offset_tag + 4].try_into() {
        Ok(slice) => slice,
        Err(_) => {
            eprintln!("Error extrayendo los bytes del Tag...");
            return None;
        }
    };

    // Ahora parseamos a u32 Little Endian
    let tag = u32::from_le_bytes(tag_bytes);
    println!("Sub type (Tag) leido: 0x{:08X}", tag);

    Some(tag)

}

pub fn get_keys_offset(boot_binario: &[u8]) -> Result<usize, String>{
    let tag: u32 = get_tag(boot_binario).ok_or("No se pudo conseguir el tag")?;

    offset_keys::get_kirk_offset(tag)
    .ok_or("Tag desconocido... No hay ruta hacia las llaves".to_string())
}


pub fn get_aes_key(boot_binario: &[u8]) -> Option<Vec<u8>> {
    let keys_offset = get_keys_offset(&boot_binario).unwrap();

     // Hora de extraer los 32 bytes de la cabecera KIRK
    let mut llave_encriptada: [u8; 32] = match boot_binario[keys_offset .. keys_offset+32].try_into() {
        Ok(slice) => slice,
        Err(_) => {
            eprintln!("Error, no se pudieron extraer los 32 bytes...");
            return None;
        }
    };

    println!("Llave extraida: {:02X?}", llave_encriptada);
    println!("Iniciando desencriptación utilizando AES-128-CBC CON KIKR1-KEY");

    // ahora desencripto con la clave KIRK1 KEY GLOBAL Y CONSIGO LA CLAVE ÚNICA DE MI JUEGO, la cual debo utilizar (...) para poder conseguir el archivo elf
    // Iniciando motor
    println!("Iniciando motor AES-128-CBC....");
    kirk_core::decrypt_game_keys(&mut llave_encriptada);
    println!("Desencriptación de la llave del juego exitosa!!!");

    // Ahora tras la desencriptación tenemos que separar la clave aes de la cmac
    let aes_key_juego = &llave_encriptada[0..16];

    Some(aes_key_juego.to_vec())
   
}