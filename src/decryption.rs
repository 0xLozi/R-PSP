use crate::kirk_core;

struct TagInfo {
    tag: u32,
    key: [u32; 36],
    code: u8,
    code_extra: Option<u8>
}

static G_KEY_EBOOT_2XX: [u32; 36] = [
    0xDA8E36FA, 0x5DD97447, 0x76C19874, 0x97E57EAF, 0x1CAB09BD, 0x9835BAC6,
    0x03D39281, 0x03B205CF, 0x2882E734, 0xE714F663, 0xB96E2775, 0xBD8AAFC7,
    0x1DD3EC29, 0xECA4A16C, 0x5F69EC87, 0x85981E92, 0x7CFCAE21, 0xBAE9DD16,
    0xE6A97804, 0x2EEE02FC, 0x61DF8A3D, 0xDD310564, 0x9697E149, 0xC2453F3B,
    0xF91D8456, 0x39DA6BC8, 0xB3E5FEF5, 0x89C593A3, 0xFB5C8ABC, 0x6C0B7212,
    0xE10DD3CB, 0x98D0B2A8, 0x5FD61847, 0xF0DC2357, 0x7701166A, 0x0F5C3B68
];

// [Tu PRX]
//    ↓
// [1] Leer Sub Type (0xD0) → 0xC0CB167C
//    ↓
// [2] Buscar en TAG_INFO → Encontrar { tag: 0xC0CB167C, code: 0x5D, ... }
//    ↓
// [3] key_type = 0x5D → clave_kirk7 = KEYVAULT[0x5D] (16 bytes)
//    ↓
// [4] Armar kirk_block[0x90] desde offsets 0x110 y 0x80 del propio PRX
//    ↓
// [5] XOR 1: kirk_block[0..0x70] ^= keyblock_externo[0x14..0x84]
//    ↓
// [6] kirk7: AES-128-CBC con clave_kirk7 + IV=[0u8;16] sobre kirk_block[0..0x70]
//    ↓
// [7] XOR 2: kirk_block[0..0x70] ^= keyblock_externo[0x20..0x90]
//    ↓
// [8] Extraer claves REALES para el payload:
//     • AES Key = kirk_block[0x00..0x10]
//     • IV      = kirk_block[0x10..0x20]
//    ↓
// [9] AES-128-CBC sobre payload[0x150..fin] con claves extraídas
//    ↓
// [10] GZIP inflate → ELF limpio

pub fn another_test(boot_binario: &[u8]) {
    // [0] Extraer total_size y payload (ANTES de todo lo demás)
    let total_size = u32::from_le_bytes(boot_binario[0x2C..0x30].try_into().unwrap()) as usize;
    let mut payload_vec: Vec<u8> = boot_binario[0x150..total_size].to_vec();  // Payload mutable


    //[1]
    let sub_type: u32 = 0xC0CB167C;

    //[2]
	// { 0xC0CB167C, g_keyEBOOT2xx, 0x5D, 0x5D },
    // Esto es lo que encontré en el archivo, en el futuro implementaré mejor esto
    let tag_info =  TagInfo {
        tag: 0xC0CB167C,
        key: G_KEY_EBOOT_2XX,
        code: 0x5D,
        code_extra: Some(0x5D),
    };

    //[3]
    let kirk7_key: [u8;16] = kirk_core::get_key_vault(tag_info.code);

    //[4] Armar kirk_block[0x90] desde offsets 0x110 y 0x80 del PRX propio
    let mut kirk_block = [0u8; 0x90]; // Estos son 144 bytes exactos
    kirk_block[0x00 .. 0x40].copy_from_slice(&boot_binario[0x110 .. 0x150]);
    kirk_block[0x40 .. 0x90].copy_from_slice(&boot_binario[0x80 .. 0xD0]);
    
    // ME FALTÓ CONVERTIR EL KEY EBOOT A 144 bytes
    let g_key_bytes: [u8;144] = G_KEY_EBOOT_2XX
                    .iter()
                    .flat_map(|&v| v.to_le_bytes()) // -> cada u32 -> 4 bytes en LE
                    .collect::<Vec<u8>>()
                    .try_into()
                    .expect("msg");

    // [5] XOR 1: kirk_block[0..0x70] ^= keyblock_externo[0x14..0x84]
    for i in 0..0x70 {
        kirk_block[i] ^= g_key_bytes[i + 0x14];
    }

    // [6] kirk7: AES-128-CBC con clave_kirk7 + IV=[0u8;16] sobre kirk_block[0..0x70]
    let kirk7_key: [u8; 16] = get_key_vault(0x5D);
    kirk_core::decrypt_payload(&kirk7_key, &mut kirk_block[0..0x70]);


    // [7] XOR 2: kirk_block[0..0x70] ^= keyblock_externo[0x20..0x90]
    for i  in 0..0x70 {
        kirk_block[i] ^= g_key_bytes[i+0x20];
    }

    // [8] Extraigo las llaves reales
    let aes_real_key: [u8;16] = kirk_block[0x00 .. 0x10].try_into().unwrap();
    let iv_real: [u8;16] = kirk_block[0x10 .. 0x20].try_into().unwrap();

    kirk_core::decrypt_final(&aes_real_key, iv_real, &mut payload_vec);


    // [10] Verificar magic 
    if &payload_vec[0..4] == b"\x7FELF" {
        println!("✅ ELF válido");
    } else if &payload_vec[0..2] == b"\x1F\x8B" {
        println!("✅ GZIP detectado");
        // Aquí iría flate2::read::GzDecoder para descomprimir
    } else {
        println!("❌ Magic desconocido: {:?}", &payload_vec[0..4]);
    }

}





pub fn get_key_vault(dir: u8) -> [u8;16] {
    return kirk_core::get_key_vault(dir);
}