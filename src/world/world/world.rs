use napi::{bindgen_prelude::FromNapiValue, NapiValue};

use crate::logger::logger::Logger;
use crate::utils::node_converter::*;

pub struct World {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub identifier: String,
  pub current_tick: u64,
  pub logger: Logger,
}

impl World {
  pub fn new(env: napi::Env, object: napi::JsObject) -> Self {
    // Get the logger object
    let logger_object = get_node_object(&object, "logger").unwrap();

    // Get the identifier object
    let identifier_object = get_node_string(&object, "identifier").unwrap();
    let identifier = identifier_object.into_utf8().unwrap().into_owned().unwrap();

    // Get the current_tick object
    let current_tick_object = get_node_bigint(&object, "currentTick").unwrap();
    let current_tick = current_tick_object.get_u64().unwrap().0;

    World { env, object, identifier, current_tick, logger: Logger::new(env.clone(), logger_object) }
  }

  pub fn send_message(&self, message: &str) {
    // Get the send_message function
    let send_message = get_node_func(&self.object, "sendMessage").unwrap();

    // Convert the message to a JsString
    let message = convert_to_js_string(&self.env, message).unwrap();

    // Call the send_message function
    send_message.call::<napi::JsString>(Some(&self.object), &[napi::JsString::from(message)]).unwrap();
  }
}

impl FromNapiValue for World {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Get the identifier object
    let identifier_object = get_node_string(&object, "identifier");
    let identifier = match identifier_object {
      Ok(identifier) => identifier.into_utf8().unwrap().into_owned().unwrap(),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Get the current_tick object
    let current_tick_object = get_node_bigint(&object, "currentTick");
    let current_tick = match current_tick_object {
      Ok(current_tick) => current_tick.get_u64().unwrap().0,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };


    // Create the logger instance
    let logger_object = get_node_object(&object, "logger");
    let logger = match logger_object {
      Ok(logger) => Logger::new(env.into(), logger),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    Ok(World { env: env.into(), object, identifier, current_tick, logger })
  }
}