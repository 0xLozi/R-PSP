use std::fs;

fn main() {
    // 1. Ruta al archivo BOOT.BIN 
    let ruta: &str = "/home/snake/Downloads/lego_batman_game/PSP_GAME/SYSDIR/EBOOT.BIN";

    println!("Intentando cargar: {}", ruta);

    // 2. Cargamos TODO el archivo crudo en un vector dinámico de bytes (Vec<u8>)
    let boot_bin = match fs::read(ruta) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Fallo leyendo el archivo: {}", e);
            return;
        }
    };

    println!("¡Archivo en memoria! Pesa: {} bytes", boot_bin.len());

    if boot_bin.len() >= 28 {
        let magic_number = &boot_bin[0..4];
        println!("Primeros 4 bytes: {:X?}", magic_number);

        // Acá verificamos que si es un ELF Puro (o sea desencriptado)
        if magic_number == [0x7F, 0x45, 0x4C, 0x46] {
            println!("Es un ejecutable ELF limpio");
        }

    }
   
}