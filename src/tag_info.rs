// struct TAG_INFO
// {
// 	u32 tag; // 4 byte value at offset 0xD0 in the PRX file
// 	const u32 *key; // "step1_result" use for XOR step
// 	u8 code;
// 	u8 codeExtra;
// };

// IN THE FUTURE ILL IMPLEMENT THIS. BUT FIRST WE HAVE TO KNOW IF THIS WORKS
// // struct TagInfo<'a> {
// //     tag: u32,
// //     key: &'a [u8],
// //     code: u8,
// //     codeExtra: Option<u8>
// // }

// // static tag_info: [TagInfo;1] =
// // [
// // 	TagInfo {
// //         tag: 0x00000000, key: g_key0, 0x42, None 
//     },
	// { 0x02000000, g_key2, 0x45 },
	// { 0x03000000, g_key3, 0x46 },
	// { 0x06000000, (u32*)g_keyDEVPSAR, 0x49 },
	// { 0x07000000, g_key_INDEXDAT1xx, 0x4A },
	// { 0x08000000, g_keyEBOOT1xx, 0x4B },
	// { 0x09000000, g_key_GAMESHARE1xx, 0x4C },
	// { 0x0A000000, g_key_GAMESHAREDEMO_150, 0x4D },
	// { 0x0B000000, g_keyUPDATER, 0x4E },
	// { 0x0C000000, g_keyDEMOS27X, 0x4F },
	// { 0x0E000000, (u32*)g_keyPSAR, 0x51},
	// { 0x0F000000, g_keyMEIMG250, 0x52 },
	// { 0x862648D1, g_keyMEIMG260, 0x52, 0x52 },
	// { 0x4467415d, g_key44, 0x59, 0x59 },
	// { 0x207bbf2f, g_key20, 0x5A, 0x5A },
	// { 0x3ace4dce, g_key3A, 0x5B, 0x5B },
	// { 0xC0CB167C, g_keyEBOOT2xx, 0x5D, 0x5D },
	// { 0x207BBF2F, g_keyUNK1, 0x5A, 0x5A },
	// { 0xBB67C59F, g_key_GAMESHARE2xx, 0x5E, 0x5E },
	// { 0x7F24BDCD, g_demokeys_280, 0x60, 0x60 }
// ];