use web_sys::js_sys::{self};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::CryptoKey;

use crate::wasm::{CryptoError, Cryptography};
use crate::wasm_bindgen_futures::JsFuture;

pub fn get_aes_gcm_params(iv: &[u8; 12], ad: &[u8], taglength: u32) -> js_sys::Object {
	crate::wasm::make_object(vec!(
		("name", JsValue::from("AES-GCM")),
		("iv", crate::wasm::slice_to_typed_array(iv).into()),
		("additionalData", crate::wasm::slice_to_typed_array(ad).into()),
		("tagLength", JsValue::from(taglength))
	)).unwrap()
}

fn get_aes_keygen_params(name: &str, length: u32) -> js_sys::Object {
	crate::wasm::make_object(vec!(
		("name", JsValue::from(name)),
		("length", JsValue::from(length)),
	)).unwrap()
}

impl Cryptography {
	pub async fn sha_hash<const SIZE: usize>(&self, data: &[u8]) -> Result<[u8; SIZE], CryptoError> {
		let result: JsValue = JsFuture::from(
			self.subtle.digest_with_str_and_u8_array(&("SHA-".to_string() + &(SIZE * 8).to_string()), data).or(Err(CryptoError::InvalidParameter))?
		).await.or(Err(CryptoError::InternalAsyncError))?;
		crate::wasm::crypto_value_to_slice::<SIZE>(result).ok_or(CryptoError::InternalSizeError)
	}

	pub async fn encrypt_aes(&self, key: &CryptoKey, data: &[u8], iv: &[u8; 12], ad: &[u8]) -> Result<Vec<u8>, CryptoError> {
		let result: JsValue = JsFuture::from(
			self.subtle.encrypt_with_object_and_u8_array(&get_aes_gcm_params(iv, ad, 128), key, data).or(Err(CryptoError::InvalidParameter))?
		).await.or(Err(CryptoError::InternalAsyncError))?;
		crate::wasm::crypto_value_to_vec(result).ok_or(CryptoError::InternalSizeError)
	}

	pub async fn decrypt_aes(&self, key: &CryptoKey, data: &[u8], iv: &[u8; 12], ad: &[u8]) -> Result<Vec<u8>, CryptoError> {
		let result: JsValue = JsFuture::from(
			self.subtle.decrypt_with_object_and_u8_array(&get_aes_gcm_params(iv, ad, 128), key, data).or(Err(CryptoError::InvalidParameter))?
		).await.or(Err(CryptoError::InternalAsyncError))?;
		crate::wasm::crypto_value_to_vec(result).ok_or(CryptoError::InternalSizeError)
	}

	pub async fn generate_aes_gcm_key(&self, length: u32, usages: &[&str]) -> Result<CryptoKey, CryptoError> {
		let result: JsValue = JsFuture::from(
			self.subtle.generate_key_with_object(
				&get_aes_keygen_params("AES-GCM", length),
				true,
				&usages.into_iter().map(|s: &&str| JsValue::from_str(s)).collect::<js_sys::Array>()
			).or(Err(CryptoError::InvalidParameter))?
		).await.or(Err(CryptoError::InternalAsyncError))?;
		
		Ok(result.dyn_into::<CryptoKey>().unwrap())
	}

	pub async fn import_key(&self, data: &[u8], usages: &[&str]) -> Result<CryptoKey, CryptoError> {
		let result: JsValue = JsFuture::from(
			self.subtle.import_key_with_str(
				"raw",
				&crate::wasm::slice_to_typed_array(data),
				"AES-GCM",
				false,
				&usages.into_iter().map(|s: &&str| JsValue::from_str(s)).collect::<js_sys::Array>()
			).or(Err(CryptoError::InvalidParameter))?
		).await.or(Err(CryptoError::InternalAsyncError))?;
		
		Ok(result.dyn_into::<CryptoKey>().unwrap())
	}
}