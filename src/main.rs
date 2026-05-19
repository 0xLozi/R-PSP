mod kirk_core;
use std::{fs};
mod offset_keys;

const SIZE_MAGIC_NUMBER: usize = 4;
const ELF_REPRESENTATION: [u8;4] = [0x7F, 0x45, 0x4C, 0x46];

fn main() -> Result<(), String> {
    // Primero leo los primeros 4 bytes: si dicen ELF seguimos
    let ruta: &str = "/home/snake/Downloads/lego_batman_game/PSP_GAME/SYSDIR/EBOOT.BIN";

    println!("Intentando cargar: {}", ruta);

    // Cargamos TODO el archivo crudo en un vector dinámico de bytes (Vec<u8>)
    let boot_binario: Vec<u8> = fs::read(ruta)
                                .map_err(|e| e.to_string())?;


    verificar_extension_binario(&boot_binario);
    println!("\n");
    verificar_size_archivo(&boot_binario);

    let aes_key_juego = get_aes_key(&boot_binario)
    .ok_or("No se pudo obtener la aes key")?;

    println!("----------------------------------");
    println!("🗿DESENCRIPTACIÓN COMPLETADA LET'S GOOO🗿");
    println!("🔐 aes key: {:02X?}", aes_key_juego);
    println!("----------------------------------");


    // ==========================================================================
    // LOCALIZAR EL PAYLOAD (EL JUEGO ENCRIPTADO)
    // ==========================================================================
    println!("\n🔍 Localizando el Payload (código encriptado del juego)...");

    // NOTA: EL CONTRATO DE HARDWARE KIRK
    // Por qué no leemos los offsets 0x24 y 0x28 de la cabecera principal EBOOT?
    // Porque en juegos encriptados oficiales, esos bytes guardan configuraciones 
    // internas del chip KIRK (flags, firmware reqs), NO direcciones de memoria reales.
    // 
    // LA SOLUCIÓN: La arquitectura de Sony tiene un "Contrato de Hardware" estricto.
    // La estructura de configuración del chip KIRK siempre mide EXACTAMENTE 
    // 0x280 bytes (640 en decimal) contados a partir del inicio del bloque de llaves.
    // 
    // Por lo tanto, el Payload (el juego real) SIEMPRE arranca en: [Offset_Llaves + 0x280]
    // La función `get_data_offset` lee el Tag, busca el offset base de las llaves, 
    // le suma por contrato los 0x280 bytes, y nos devuelve la dirección
    let tag: u32 = get_tag(&boot_binario).ok_or("no se pudo conseguir el tag")?;

    let data_offset: usize = offset_keys::get_data_offset(tag)
    .ok_or("Error crítico: Versión de encriptación (Tag) no soportada")?;



    // El tamaño del juego es todo lo que queda del archivo a partir de ese punto
    let data_size = boot_binario.len() - data_offset;

    println!("🎯 ¡Mapeo exitoso para Tag 0x{:08X}!", tag);
    println!("📍 El juego encriptado arranca en el offset: 0x{:08X}", data_offset);
    println!("📦 Tamaño del bloque a desencriptar: {} bytes", data_size);

    Ok(())


}


fn verificar_size_archivo(boot_binario: &Vec<u8>) {
    // Esto para asegurarnos de que el archivo no se encuentra corrupto (segun la página: 0x2C)
    println!("Ahora vamos a verificar el total size que nos dice el archivo binario");
    // 1. Definimos de dónde a dónde queremos cortar (0x2C hasta 0x30)
    let offset_size = 0x2C;

    // Despues extraemos los 4 bytes necesarios para saber cuánto es el size especifico
    let size_bytes: [u8; 4] = match boot_binario[offset_size .. offset_size + 4].try_into() {
        Ok(slice) => slice,
        Err(_) => {
            eprintln!("Error crítico: no se pudieron extraer los 4 bytes... por qué carajos es?");
            return; // para abortar la misión
        }
    };

    // Y por último: imprimimos el total size
    let total_size = u32::from_le_bytes(size_bytes);
    println!("TOTAL SIZE: {}", total_size);

    println!("\n Ahora comparamos con el size del boot_binario: {} y el size que nos dice los 4 bytes extraidos: {}", boot_binario.len(), total_size);
    if boot_binario.len() as u32 == total_size {
        println!("Los tamaños coinciden VAMOOOOOOOOOS\n");
    } else {
        println!("los tamaños no coinciden.... PUEDE SER (seguramente) que el juego se encuentre corrupto");
    }


}

fn verificar_extension_binario(boot_binario: &[u8]) {
    println!("¡Archivo en memoria! Pesa: {} bytes", boot_binario.len());
    if boot_binario.len() > SIZE_MAGIC_NUMBER {
        // Ahora sacamos el magic number
        let magic_number = &boot_binario[0..4];

        // Y acá es donde verificamos si es un ELF 
        if magic_number == ELF_REPRESENTATION {
            println!("This is a clean ELF executable");
        } else {
            println!("This isn't a clean ELF executable...\n posibbly is an encrypted binary file...");
            println!("Let's see what is says the first 4 bytes of de BIN file");
            let assci_representation = String::from_utf8_lossy(magic_number);
            println!("{}", assci_representation);
        }
    }
}

fn get_tag(boot_binario: &[u8]) -> Option<u32> {
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

fn get_keys_offset(boot_binario: &[u8]) -> Result<usize, String>{
    let tag: u32 = get_tag(boot_binario).ok_or("No se pudo conseguir el tag")?;

    offset_keys::get_kirk_offset(tag)
    .ok_or("Tag desconocido... No hay ruta hacia las llaves".to_string())
}


fn get_aes_key(boot_binario: &[u8]) -> Option<Vec<u8>> {
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