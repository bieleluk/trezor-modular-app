#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
extern crate alloc;

use prost::Message;
use trezor_app_sdk::{
    Error, Result, ResultExt, WireDecode, WireEncode, error, wire_handler, wire_receive_wire_start,
};

#[cfg(not(test))]
pub(crate) use alloc::{
    string::{String, ToString},
    vec::Vec,
};
#[cfg(test)]
pub(crate) use std::{
    string::{String, ToString},
    vec::Vec,
};

// Include generated code
pub(crate) mod proto;

#[macro_use]
pub(crate) mod translations;

mod get_public_key;
mod strutil;

use proto::{funnycoin::GetPublicKey, messages::MessageType};

/// Wire codec for [`wire_handler!`] — encodes/decodes messages via [`prost`].
///
/// This is the only place in the app that ties the SDK's serializer-agnostic
/// `wire_handler!` macro to prost; the SDK crate itself has no prost dependency.
struct ProstCodec;

impl<T: Message + Default> WireDecode<T> for ProstCodec {
    fn decode(data: &[u8]) -> Result<T> {
        T::decode(data).map_err(|_| Error::InvalidMessage)
    }
}

impl<T: Message> WireEncode<T> for ProstCodec {
    fn encode(val: &T) -> crate::Vec<u8> {
        val.encode_to_vec()
    }
}

// Generate all handler functions
wire_handler!(
    handle_get_public_key,
    ProstCodec,
    GetPublicKey,
    MessageType::PublicKey,
    get_public_key::get_public_key
);

// Application entry point - receives raw bytes, returns raw bytes
#[unsafe(no_mangle)]
pub fn app() -> Result<()> {
    loop {
        let (id, data) = wire_receive_wire_start().c()?;
        handle_wire_message(id as i32, &data).c()?
    }
}

/// Entry point for handling protobuf function calls
/// fn_id: function identifier
/// data: serialized protobuf request
/// Returns: serialized protobuf response
pub fn handle_wire_message(id: i32, data: &[u8]) -> Result<()> {
    match id.try_into() {
        Ok(MessageType::GetPublicKey) => handle_get_public_key(data),
        Ok(_) => {
            error!("Invalid function: {:?}", id);
            Err(Error::InvalidFunction)
        }
        Err(_) => {
            error!("Non existing message type: {:?}", id);
            Err(Error::InvalidFunction)
        }
    }
}

#[cfg(test)]
pub(crate) mod test_init {
    use std::sync::Once;
    use trezor_app_sdk::mock::{dummy_trezor_api_getter_t, sdk_init};
    pub static INIT: Once = Once::new();

    pub fn init_sdk() {
        INIT.call_once(|| unsafe {
            sdk_init(Some(dummy_trezor_api_getter_t));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_sdk() {
        test_init::init_sdk();
    }

    #[test]
    #[cfg(feature = "model_t3w1")]
    fn test_model_t3w1() {
        assert!(cfg!(feature = "model_t3w1"));
        assert!(!cfg!(feature = "model_t3t1"));
    }

    #[test]
    #[cfg(feature = "model_t3t1")]
    fn test_model_t3t1() {
        assert!(cfg!(feature = "model_t3t1"));
        assert!(!cfg!(feature = "model_t3w1"));
    }
}
