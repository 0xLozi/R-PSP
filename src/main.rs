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
    decrypt_payload(&boot_binario);

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



// enum ScePrxEncryptType {
//     DECRYPT_MODE_NONE                       = 0,
//     DECRYPT_MODE_INTERNAL_MODULE            = 1,
//     DECRYPT_MODE_KERNEL_MODULE              = 2,
//     DECRYPT_MODE_VSH_MODULE                 = 3,
//     DECRYPT_MODE_USER_MODULE                = 4,
//     DECRYPT_MODE_UNKNOWN_5                  = 5,
//     DECRYPT_MODE_UNKNOWN_6                  = 6,
//     DECRYPT_MODE_PSAR_DEVTOOL               = 7,
//     DECRYPT_MODE_VSH_INDEX_DAT              = 8,
//     DECRYPT_MODE_UMD_GAME_EXEC              = 9,
//     DECRYPT_MODE_GAMESHARING_EXEC           = 10,
//     DECRYPT_MODE_GAMESHARING_EXEC_DEVTOOL   = 11,
//     DECRYPT_MODE_MS_UPDATER                 = 12,
//     DECRYPT_MODE_DEMO_EXEC                  = 13,
//     DECRYPT_MODE_APP_MODULE                 = 14,
//     DECRYPT_MODE_PSAR_RETAIL                = 15,
//     DECRYPT_MODE_ME_IMAGE                   = 16,
//     DECRYPT_MODE_UNKNOWN_17                 = 17,
//     DECRYPT_MODE_MS_GAME_PATCH              = 18,
//     DECRYPT_MODE_MS_GAME_PATCH_DEVTOOL      = 19,
//     DECRYPT_MODE_POPS_EXEC                  = 20,
//     DECRYPT_MODE_UNKNOWN_21                 = 21,
//     DECRYPT_MODE_UNKNOWN_22                 = 22,
//     DECRYPT_MODE_USER_NPDRM                 = 23,
//     DECRYPT_MODE_UNKNOWN_24                 = 24,
//}
    

pub fn decrypt_payload(boot_binario: &[u8]) {
    let encrypt_type: [u8;2] = boot_binario[0x7C .. 0x7C + 2].try_into().unwrap();
    let encrypt = u16::from_le_bytes(encrypt_type);
    println!("ENCRYPT TYPE: {}", encrypt);

    println!("Ahora toca leer el subtype");
    let sub_type: [u8;4] = boot_binario[0xD0 .. 0xD0+4].try_into().unwrap();
    let sub_type_le = u32::from_le_bytes(sub_type);
    println!("SUB TYPE: 0x{:08X}", sub_type_le);

    // Me fuí al PrxDecrypter.cpp a buscar mi tag y tiene esto:
    // 	{ 0xC0CB167C, g_keyEBOOT2xx, 0x5D, 0x5D },
    // como es un static const TAG_INFO g_tagInfo[], su estructura se compone de:
    //
    // struct TAG_INFO
    // {
    //     u32 tag; // 4 byte value at offset 0xD0 in the PRX file
    //     const u32 *key; // "step1_result" use for XOR step
    //     u8 code;
    //     u8 codeExtra;
    // };
    // static const u32 g_keyEBOOT2xx[] = {
	// 	0xDA8E36FA, 0x5DD97447, 0x76C19874, 0x97E57EAF, 0x1CAB09BD, 0x9835BAC6,
	// 	0x03D39281, 0x03B205CF, 0x2882E734, 0xE714F663, 0xB96E2775, 0xBD8AAFC7,
	// 	0x1DD3EC29, 0xECA4A16C, 0x5F69EC87, 0x85981E92, 0x7CFCAE21, 0xBAE9DD16,
	// 	0xE6A97804, 0x2EEE02FC, 0x61DF8A3D, 0xDD310564, 0x9697E149, 0xC2453F3B,
	// 	0xF91D8456, 0x39DA6BC8, 0xB3E5FEF5, 0x89C593A3, 0xFB5C8ABC, 0x6C0B7212,
	// 	0xE10DD3CB, 0x98D0B2A8, 0x5FD61847, 0xF0DC2357, 0x7701166A, 0x0F5C3B68};

    let g_keyEBOOT2xx: [u32; 36] = [
		0xDA8E36FA, 0x5DD97447, 0x76C19874, 0x97E57EAF, 0x1CAB09BD, 0x9835BAC6,
		0x03D39281, 0x03B205CF, 0x2882E734, 0xE714F663, 0xB96E2775, 0xBD8AAFC7,
		0x1DD3EC29, 0xECA4A16C, 0x5F69EC87, 0x85981E92, 0x7CFCAE21, 0xBAE9DD16,
		0xE6A97804, 0x2EEE02FC, 0x61DF8A3D, 0xDD310564, 0x9697E149, 0xC2453F3B,
		0xF91D8456, 0x39DA6BC8, 0xB3E5FEF5, 0x89C593A3, 0xFB5C8ABC, 0x6C0B7212,
		0xE10DD3CB, 0x98D0B2A8, 0x5FD61847, 0xF0DC2357, 0x7701166A, 0x0F5C3B68
    ];
    let code = get_key_vault(0x5D);
    

    println!("g_keyEBOOT2xx[0] = 0x{:08X}", g_keyEBOOT2xx[0]);
    println!("to_le_bytes() = {:02X?}", g_keyEBOOT2xx[0].to_le_bytes());
    // Esperado: [FA, 36, 8E, DA]

    let aes_key_aux: [u32; 4] = g_keyEBOOT2xx[0..4].try_into().unwrap();
    let vec_aes: Vec<u8> = aes_key_aux
                            .iter()
                            .flat_map(|&v| v.to_le_bytes())
                            .collect();
    let aes_key: [u8;16] = vec_aes.try_into().unwrap();


    let iv_key_aux: [u32; 4] = g_keyEBOOT2xx[4..8].try_into().unwrap();
    let vec_iv: Vec<u8> = iv_key_aux
                .iter()
                .flat_map(|&v| v.to_le_bytes())
                .collect();
    let iv_key: [u8;16] = vec_iv.try_into().unwrap();

    println!("AES Key (hex): {:02X?}", aes_key);  // La nueva, correcta
    println!("IV Key (hex): {:02X?}", iv_key);    // La extraída del keyblock

    // Offset: 0x140 Size: 0x10 Name: Data Key Notes: ?AES? Key
    // Por lo tanto desde ahí debo desencriptar. Despues:
    // Calcular total size y ese total size - 0x150  seria el payload len()
    // 150 porque el data key mide 0x10
    
    let data_offset = 0x150;
    let offset_size = 0x2C;

    // Despues extraemos los 4 bytes necesarios para saber cuánto es el size especifico
    let size_bytes: [u8; 4] = match boot_binario[offset_size .. offset_size + 4].try_into() {
        Ok(slice) => slice,
        Err(_) => {
            eprintln!("Error crítico: no se pudieron extraer los 4 bytes... por qué carajos es?");
            return; // para abortar la misión
        }
    };

    let total_size = u32::from_le_bytes(size_bytes);
    println!("TOTAL SIZE: {}", total_size);

    
    let payload= &boot_binario[0x150..total_size as usize];

    let mut payload_vec: Vec<u8> = payload.to_vec();

    // Si total_size coincide con el tamaño del archivo
    assert_eq!(total_size as usize, boot_binario.len());

    // let end = total_size.min(boot_binario.len() as u32) as usize;
    // let payload = &boot_binario[0x150..end];

    // FALTA REALIZAR EL XOR
    let xor_mask:[u32;28]= g_keyEBOOT2xx[8..36].try_into().unwrap();
    let vec: Vec<u8> = xor_mask
            .iter()
            .flat_map(|&v| v.to_le_bytes())
            .collect();

    let xor_converted:[u8; 112] = vec.try_into().unwrap();

    // HORA DE APLICAR XOR AL PAYLOAD AAAAAAAAAAAAAAA
    for i in 0 .. payload.len() {
        payload_vec[i] ^= xor_converted[i % 112];
    }


    // HORA DE DESENCRIPTAR
    kirk_core::decrypt_payload(&aes_key, &iv_key, &mut payload_vec);



    // payload_vec ya tiene el resultado descifrado (y mutado in-place)
    if payload_vec.len() >= 4 && &payload_vec[0..4] == b"\x7FELF" {
        println!("✅ Éxito: ELF válido detectado");
        // Aquí podés escribir payload_vec a un archivo .elf para analizarlo
    } else {
        println!("❌ Fallo: Magic incorrecto {:?}", &payload_vec[0..4]);
    }

    // Después de AES, antes de descomprimir:
    if &payload_vec[0..2] == b"\x1F\x8B" {
        println!("✅ Datos GZIP detectados, procediendo a descomprimir...");
    } else {
        println!("no son datos gzip...");
    }

    if &payload_vec[0..2] == b"\x1F\x8B" {
        println!("es un magic gzip? ni idea");
    } else {
        println!(" no es nada... sigamos");
    }


    /// AHORA SI, CREO QUE YA LO ENTENDI
    final_decryption(boot_binario);

}


pub fn final_decryption(boot_binario: &[u8]) {

    // Primero hay que extraer el kirk header
    let kirk_header: &[u8] = boot_binario[0x00..0x150].try_into().unwrap();

    
    let mut g_key_eboot_2xx: [u32; 36] = [
		0xDA8E36FA, 0x5DD97447, 0x76C19874, 0x97E57EAF, 0x1CAB09BD, 0x9835BAC6,
		0x03D39281, 0x03B205CF, 0x2882E734, 0xE714F663, 0xB96E2775, 0xBD8AAFC7,
		0x1DD3EC29, 0xECA4A16C, 0x5F69EC87, 0x85981E92, 0x7CFCAE21, 0xBAE9DD16,
		0xE6A97804, 0x2EEE02FC, 0x61DF8A3D, 0xDD310564, 0x9697E149, 0xC2453F3B,
		0xF91D8456, 0x39DA6BC8, 0xB3E5FEF5, 0x89C593A3, 0xFB5C8ABC, 0x6C0B7212,
		0xE10DD3CB, 0x98D0B2A8, 0x5FD61847, 0xF0DC2357, 0x7701166A, 0x0F5C3B68
    ];



    let offset_size = 0x2C;
    let size_bytes: [u8; 4] = match boot_binario[offset_size .. offset_size + 4].try_into() {
        Ok(slice) => slice,
        Err(_) => {
            eprintln!("Error crítico: no se pudieron extraer los 4 bytes... por qué carajos es?");
            return; // para abortar la misión
        }
    };
    let total_size = u32::from_le_bytes(size_bytes);
    let payload= &boot_binario[0x150..total_size as usize];
    let mut payload_vec: Vec<u8> = payload.to_vec();




    // ESTO ES PARA TYPE 1 OK?
    // Paso 1: Construir el "KIRK block temporal" (0x90 = 144 bytes)
    let mut kirk_block = [0u8; 0x90]; // 144 bytes
    kirk_block[0x00 .. 0x40].copy_from_slice(&boot_binario[0x110 .. 0x150]);
    kirk_block[0x40..0x90].copy_from_slice(&boot_binario[0x80..0xD0]);

    let g_key_eboot_2xx: Vec<u8> = g_key_eboot_2xx.iter().flat_map(|&v|v.to_le_bytes()).collect();

    let mut g_key_eboot_2xx: [u8;144] = g_key_eboot_2xx.try_into().unwrap();


    println!("mondongo");

    // XOR 1: kirk_block[0..0x70] ^= g_key_bytes[0x14..0x84]
    for i in 0..0x70 {
        kirk_block[i] ^= g_key_eboot_2xx[i + 0x14];
    }

    
}


pub fn get_key_vault(dir: usize) -> [u8;16] {
    return kirk_core::get_key_vault(dir);

}