/// Recibe el Sub Type (Tag) del EBOOT y devuelve el offset absoluto 
/// donde comienzan las llaves KIRK (AES y CMAC).
/// Toda información sobre los tags las encontré en: https://www.psdevwiki.com/psp/PRX_File_Format
pub fn get_kirk_offset(tag: u32) -> Option<usize> {
    match tag {
        // --- PSP Header V1 (Type 0, 1, 8) ---
        // offset: 0x110 (o 0xD0 + 0x40 de padding por el tema de las firmas para así verificar que era una copia genuina, pero como yo decido que chota hacer con eso opto por que me la rrecontra suda y no le voy a hacer caso a esa firmaxdxdxddx)
        0x03000000 | 0x04000000 | 0x05000000 | 0x06000000 | 
        0x08000000 | 0x09000000 | 0x0A000000 | 0x0C000000 | 
        0x0D000000 | 0x0E000000 | 0x0F000000 => Some(0x110),

        // Type 1 y Type 8 antiguos
        0x3ACE4DCE | 0xBB67C59F | 0x7F24BDCD | 
        0x1BC8D12B | 0x862648D1 | 0x02000000 | 0x0B000000 => Some(0x110),

        // --- PSP Header V1 MODERNO (Type 1 especial) ---
        0xC0CB167C => Some(0x150),

        // --- PSP Header V2 (Type 2, 3, 4, etc) ---
        // Tienen firmas más grandes, el bloque KIRK se empuja a 0x2B0
        0x457B05F0 | 0x457B06F0 | 0x457B08F0 | 0x457B0AF0 | 0x457B0BF0 | 
        0x457B0CF0 | 0x457B10F0 | 0x76202403 | 0x628910F0 | 0x8B9B10F0 | 
        0x5A5C10F0 | 0xE42C2303 | 0x8004FD03 | 0xD91605F0 | 0xD91606F0 | 
        0xD9160AF0 | 0xD9160BF0 | 0xD91610F0 | 0xD91611F0 => Some(0x2B0),
        
        // Si encontramos un tag que no mapeamos aún
        _ => None,
    }
}


/// Recibe el Sub Type (Tag) del EBOOT, calcula el offset base de KIRK
/// y le suma el tamaño fijo de la estructura (0x280) para devolver
/// la dirección absoluta exacta donde arranca el juego encriptado (Payload).
pub fn get_data_offset(tag: u32) -> Option<usize> {
    // Buscamos primero donde empieza la cabecera KIRK usando mi función existente
    match get_kirk_offset(tag) {
        // Contrato de hardware! Sumamos 0x280 bytes fijos a la base
        Some(kirk_offset) => Some(kirk_offset + 0x280),
        // Si el Tag no existe en la lista, devolvemos None
        None => None,
    }
}