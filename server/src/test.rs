use base64::Engine;
use web_sys::CryptoKey;

use crate::wasm;

pub async fn test() -> String {
	crate::console_log!("test\n{:?}\ntest", crate::crypto::get_aes_gcm_params(&[0 as u8; 12], &[0 as u8; 0], 128));

	let crypto: wasm::Cryptography = wasm::get_crypto(&wasm::get_global().expect("no global")).expect("no crypto");

	let buffer: Vec<u8> = "hello".as_bytes().to_owned();

	let key: CryptoKey = crypto.generate_aes_gcm_key(256, &["encrypt", "decrypt"]).await.expect("no key");

	let encrypt: Vec<u8> = crypto.encrypt_aes(&key, &buffer, &[0 as u8; 12], &[0 as u8; 0]).await.expect("no enc");
	let decrypt: Vec<u8> = crypto.decrypt_aes(&key, &encrypt, &[0 as u8; 12], &[0 as u8; 0]).await.expect("no dec");

	let mut string: String = "".to_string();

	string.push_str(&base64::engine::general_purpose::URL_SAFE.encode(crypto.sha_hash::<32>(buffer.as_slice()).await.expect("failed hash")));
	string.push('\n');
	string.push_str(&base64::engine::general_purpose::URL_SAFE.encode(encrypt));
	string.push('\n');
	string.push_str(std::str::from_utf8(&decrypt).expect("bad encoding"));
	string
}