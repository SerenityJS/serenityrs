use napi::{bindgen_prelude::FromNapiValue, NapiValue};

use crate::utils::node_converter::*;

pub struct BlockType {
  pub object: napi::JsObject,
  pub env: napi::Env,

  pub identifier: String,
  pub custom: bool,
  pub loggable: bool,
  pub air: bool,
  pub liquid: bool,
  pub solid: bool,
}

impl BlockType {
  pub fn new(env: napi::Env, object: napi::JsObject) -> Self {
    // Get the identifier
    let identifier_object = get_node_string(&object, "identifier").unwrap();
    let identifier = identifier_object.into_utf8().unwrap().into_owned().unwrap();

    // Get the custom
    let custom_object = get_node_boolean(&object, "custom").unwrap();
    let custom = custom_object.get_value().unwrap();

    // Get the loggable
    let loggable_object = get_node_boolean(&object, "loggable").unwrap();
    let loggable = loggable_object.get_value().unwrap();

    // Get the air
    let air_object = get_node_boolean(&object, "air").unwrap();
    let air = air_object.get_value().unwrap();

    // Get the liquid
    let liquid_object = get_node_boolean(&object, "liquid").unwrap();
    let liquid = liquid_object.get_value().unwrap();

    // Get the solid
    let solid_object = get_node_boolean(&object, "solid").unwrap();
    let solid = solid_object.get_value().unwrap();

    BlockType { env, object, identifier, custom, loggable, air, liquid, solid }
  }
}

impl FromNapiValue for BlockType {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the BlockType instance
    Ok(BlockType::new(env.into(), object))
  }
}