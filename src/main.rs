use aes::Aes128;

use cbc::{
    Encryptor,
    Decryptor,
};

use cbc::cipher::{
    block_padding::Pkcs7,
    BlockEncryptMut,
    BlockDecryptMut,
    KeyIvInit,
};



fn main() {
    // Investigating how encrypt and decrypt works
    let plaintext = b"Hoal ente como aqndan";

    println!("PLAINTEXT");
    println!("{}", String::from_utf8_lossy(plaintext));

    // AES-128 KEY (16 bytes)
    let key = b"1234567890abcdef";

    // IV (16 Bytes)
    let iv = b"initvector123456";

    // Encryptor
    let encryptor = Encryptor::<Aes128>::new(key.into(), iv.into());

    // BUFFER
    let mut buffer = plaintext.to_vec();

    // espacio extra para el padding
    buffer.resize(buffer.len()+16, 0);

    // encrypt
    let ciphertext = encryptor.encrypt_padded_mut::<Pkcs7>(
        &mut buffer,
        plaintext.len(),
    ).unwrap();

    println!("CIPHERTEXT");

    println!("{:?}", ciphertext);


    // DECRYPTOR

    let decryptor =
        Decryptor::<Aes128>::new(key.into(), iv.into());

    // necesitamos buffer mutable
    let mut decrypt_buffer = ciphertext.to_vec();

    // DECRYPT

    let decrypted = decryptor
        .decrypt_padded_mut::<Pkcs7>(
            &mut decrypt_buffer
        )
        .unwrap();

    println!("\nDECRYPTED:");
    println!("{}", String::from_utf8_lossy(decrypted));
   
}