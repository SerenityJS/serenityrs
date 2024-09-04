use napi::{bindgen_prelude::FromNapiValue, NapiValue};

use crate::logger::logger::Logger;
use crate::utils::node_converter::*;

pub struct Worlds {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub logger: Logger,
}

impl Worlds {
  pub fn new(env: napi::Env, object: napi::JsObject) -> Self {
    // Get the logger object
    let logger_object = get_node_object(&object, "logger").unwrap();

    Worlds { env, object, logger: Logger::new(env.clone(), logger_object) }
  }

  pub fn on(&self, event: i32, callback: napi::JsFunction) {
    // Get the on function
    let on = get_node_func(&self.object, "on").unwrap();

    let event = convert_to_js_number(&self.env, event).unwrap();

    let ev_val = event.into_unknown();
    let cb_val = callback.into_unknown();

    // Call the on function
    on.call(Some(&self.object), &[ev_val, cb_val]).unwrap();
  }
}

impl FromNapiValue for Worlds {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
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

    Ok(Worlds { logger, env: env.into(), object })
  }
}