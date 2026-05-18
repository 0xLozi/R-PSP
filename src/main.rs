mod kirk_core;
use std::fs;
mod offset_keys;

const SIZE_MAGIC_NUMBER: usize = 4;
const ELF_REPRESENTATION: [u8;4] = [0x7F, 0x45, 0x4C, 0x46];

fn main() {
    let bytes = [0x70, 0xB7, 0x3E, 0x00];
    let value = u32::from_le_bytes(bytes);

    println!("{:#X}", value);

    // Primero leo los primeros 4 bytes: si dicen ELF seguimos
    let ruta: &str = "/home/snake/Downloads/lego_batman_game/PSP_GAME/SYSDIR/EBOOT.BIN";
    println!("Intentando cargar: {}", ruta);

    // Cargamos TODO el archivo crudo en un vector dinámico de bytes (Vec<u8>)
    let boot_binario: Vec<u8> = match fs::read(ruta) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Fallo leyendo el archivo: {}", e);
            return;
        }
    };

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

    println!("\n");
    println!("Ahora vamos a verificar el total size que nos dice el archivo binario. Esto para asegurarnos de que el archivo no se encuentra corrupto (segun la página: 0x2C)");

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
    println!("{}", total_size);

    println!("\n \n Ahora comparamos con el size del boot_binario: {} y el size que nos dice los 4 bytes extraidos: {}", boot_binario.len(), total_size);
    if boot_binario.len() as u32 == total_size {
        println!("Los tamaños coinciden VAMOOOOOOOOOS");
    } else {
        println!("los tamaños no coinciden.... PUEDE SER (seguramente) que el juego se encuentre corrupto");
    }







    // SIGUIENTE FASE: LECTURA DEL TAG
    // EN POCAS PALABRAS ES NECESARIO PARA SABER DONDE SE ENCUENTRA LA CABECERA KIRK
    let offset_tag = 0xD0;
    let tag_bytes: [u8;4] = match boot_binario[offset_tag .. offset_tag + 4].try_into() {
        Ok(slice) => slice,
        Err(_) => {
            eprintln!("Error extrayendo los bytes del Tag...");
            return;
        }
    };

    // Ahora parseamos a u32 Little Endian
    let tag = u32::from_le_bytes(tag_bytes);
    println!("Sub type (Tag) leido: 0x{:08X}", tag);

    let keys_offset = match offset_keys::get_kirk_offset(tag) {
        Some(offset) => offset,
        None => {
            println!("Tag desconocido... No hay ruta hacia las llaves....");
            return;
        }
    };
    println!("VAMOS!!! Las llaves encriptadas empiezan en el offset: 0x{:03X}", keys_offset);







    // Hora de extraer los 32 bytes de la cabecera KIRK
    let mut llaves_encriptadas: [u8; 32] = match boot_binario[keys_offset .. keys_offset+32].try_into() {
        Ok(slice) => slice,
        Err(_) => {
            eprintln!("Error, no se pudieron extraer los 32 bytes...");
            return;
        }
    };

    println!("Llaves extraidas: {:02X?}", llaves_encriptadas);
    println!("Iniciando desencriptación utilizando AES-128-CBC CON KIKR1-KEY");






}


