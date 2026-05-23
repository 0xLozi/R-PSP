const SIZE_MAGIC_NUMBER: usize = 4;
const ELF_REPRESENTATION: [u8;4] = [0x7F, 0x45, 0x4C, 0x46];

pub fn verificar_size_archivo(boot_binario: &Vec<u8>) {
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

pub fn verificar_extension_binario(boot_binario: &[u8]) {
    println!("¡Archivo en memoria! Pesa: {} bytes", boot_binario.len());
    if boot_binario.len() > SIZE_MAGIC_NUMBER {
        // Ahora sacamos el magic number
        let extension = &boot_binario[0..4];

        // Y acá es donde verificamos si es un ELF 
        if extension == ELF_REPRESENTATION {
            println!("This is a clean ELF executable");
        } else {
            println!("This isn't a clean ELF executable...\n posibbly is an encrypted binary file...");
            println!("Let's see what is says the first 4 bytes of de BIN file");
            let assci_representation = String::from_utf8_lossy(extension);
            println!("{}", assci_representation);
        }
    }

    let ct: [u8; 2] = boot_binario[0x6..0x8].try_into().unwrap();
    let compression_type = u16::from_le_bytes(ct) & 0xF00;  // 👈 máscara importante
    let type_compression = match compression_type {
        0x000 => "GZIP",
        0x100 => "2RLZ",
        0x200 => "KL4E",
        0x300 => "Plain (sin compresión)",
        _ => "Desconocido",
    };
    println!("Compression type: {}", type_compression);
}