use napi::bindgen_prelude::FromNapiValue;
use napi::NapiValue;

use crate::utils::node_converter::*;
use crate::logger::logger::Logger;

pub struct Plugin {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub logger: Logger,
  pub path: String,
}

impl Plugin {
  pub fn new(env: napi::Env, object: napi::JsObject) -> napi::Result<Self> {
    // Create the logger instance
    let logger_object = get_node_object(&object, "logger");
    let logger = match logger_object {
      Ok(logger) => Logger::new(env.clone(), logger),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Get the path object
    let path_object = get_node_string(&object, "path");
    let path = match path_object {
      Ok(path) => path.into_utf8().unwrap().into_owned().unwrap(),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    Ok(Plugin { env, object, logger, path })
  }
}

impl FromNapiValue for Plugin {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the Plugin instance
    Ok(Plugin::new(env.into(), object)?)
  }
}