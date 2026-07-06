#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
extern crate alloc;

use prost::Message;
use trezor_app_sdk::{
    CORE_SERVICE, Error, IpcMessage, Result, ResultExt, debug, error,
    service::{self, CoreIpcService},
    util::Timeout,
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

/// Macro to generate handler functions
macro_rules! wire_handler {
    ($handler_name:ident, $request_type:ty, $response_msg:expr, $handler_fn:path) => {
        #[inline(never)]
        fn $handler_name(request_data: &[u8]) -> Result<()> {
            let request = <$request_type>::decode(request_data)
                .map_err(|_| Error::InvalidMessage)
                .c()?;

            let response = $handler_fn(request);

            match response {
                Ok(resp) => {
                    let response_bytes = resp.encode_to_vec();
                    let message = IpcMessage::new(
                        ($response_msg as i32)
                            .try_into()
                            .map_err(|_| Error::InvalidMessage)
                            .c()?,
                        &response_bytes,
                    );
                    message
                        .send(service::CORE_SERVICE_REMOTE, CoreIpcService::WireEnd.into())
                        .map_err(Into::into)
                        .c()?;
                }
                Err(e) => {
                    let message = IpcMessage::new(e.code(), e.message().as_bytes());
                    trezor_app_sdk::error!("{}", e);
                    message
                        .send(
                            service::CORE_SERVICE_REMOTE,
                            CoreIpcService::WireError.into(),
                        )
                        .map_err(Into::into)
                        .c()?;
                }
            }

            Ok(())
        }
    };
}

// Generate all handler functions
wire_handler!(
    handle_get_public_key,
    GetPublicKey,
    MessageType::PublicKey,
    get_public_key::get_public_key
);

// Application entry point - receives raw bytes, returns raw bytes
#[unsafe(no_mangle)]
pub fn app() -> Result<()> {
    loop {
        debug!("Waiting for next WireStart message from core service");
        let message = CORE_SERVICE
            .receive(Timeout::max())
            .map_err(Into::into)
            .c()?;
        match message.service().into() {
            CoreIpcService::WireStart => handle_wire_message(&message).c()?,
            _ => {
                error!(
                    "Invalid service invoked: {:?}, message id {:?}, data {:?}",
                    message.service(),
                    message.id(),
                    message.data()
                );
                return Err(Error::InvalidFunction);
            }
        };
    }
}

/// Entry point for handling protobuf function calls
/// fn_id: function identifier
/// data: serialized protobuf request
/// Returns: serialized protobuf response
pub fn handle_wire_message(message: &IpcMessage) -> Result<()> {
    match (message.id() as i32).try_into() {
        Ok(MessageType::GetPublicKey) => handle_get_public_key(message.data()),
        Ok(_) => {
            error!("Invalid function: {:?}", message.id());
            Err(Error::InvalidFunction)
        }
        Err(_) => {
            error!("Non existing message type: {:?}", message.id());
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
