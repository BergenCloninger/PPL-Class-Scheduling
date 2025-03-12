use web_sys::{js_sys::{self, ArrayBuffer, Uint8Array}, wasm_bindgen::{JsCast, JsValue}, Crypto, ServiceWorkerGlobalScope, SubtleCrypto};

pub fn get_global() -> Option<ServiceWorkerGlobalScope> {
	js_sys::global().dyn_into::<ServiceWorkerGlobalScope>().ok()
}

#[allow(dead_code)]
pub struct Cryptography {
	pub crypto: Crypto,
	pub subtle: SubtleCrypto
}

pub fn get_crypto(global: &ServiceWorkerGlobalScope) -> Option<Cryptography> {
	global.crypto().ok().map(|crypto: Crypto| -> Cryptography { Cryptography { subtle: crypto.subtle(), crypto } })
}

pub fn crypto_value_to_vec(value: JsValue) -> Option<Vec<u8>> {
	let buffer: ArrayBuffer = value.dyn_into::<ArrayBuffer>().ok()?;
	let typed_buffer: Uint8Array = Uint8Array::new(&buffer); 
	Some(typed_buffer.to_vec()) // copies :(
}

pub fn crypto_value_to_slice<const SIZE: usize>(value: JsValue) -> Option<[u8; SIZE]> {
	crypto_value_to_vec(value)?.try_into().ok()
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CryptoError {
	InvalidParameter,
	InternalSizeError,
	InternalAsyncError
}

pub fn make_object_entry(key: &str, value: JsValue) -> JsValue {
	JsValue::from([JsValue::from(key), value].to_vec())
}

pub fn make_object(entries: Vec<(&str, JsValue)>) -> Option<js_sys::Object> {
	let list: js_sys::Array = js_sys::Array::new();

	for (key, value) in entries {
		list.push(&make_object_entry(key, value));
	}
	
	js_sys::Object::from_entries(&JsValue::from(list)).ok()
}

pub fn slice_to_typed_array(data: &[u8]) -> Uint8Array {
	let data_js: Uint8Array = Uint8Array::new_with_length(data.len() as u32);
	data_js.copy_from(data);
	data_js
}