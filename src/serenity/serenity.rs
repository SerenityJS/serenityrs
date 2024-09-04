use napi::{bindgen_prelude::FromNapiValue, JsObject, NapiValue, Result};

use crate::{logger::logger::Logger, utils::node_converter::get_node_object};

use super::world::Worlds;

pub struct Serenity {
  pub object: JsObject,
  pub env: napi::Env,
  pub logger: Logger,
  pub worlds: Worlds
}

impl Serenity {
  pub fn start(&self) {
    self.logger.log("Starting the Serenity application");
  }
}

impl FromNapiValue for Serenity {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> Result<Self> {
    // Create the JsObject from the napi_value
    let object = match JsObject::from_raw(env.clone(), value.clone()) {
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

    // Create the worlds instance
    let worlds_object = get_node_object(&object, "worlds");
    let worlds = match worlds_object {
      Ok(worlds) => Worlds::new(env.into(), worlds),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    Ok(Serenity { logger, env: env.into(), object, worlds })
  }
}