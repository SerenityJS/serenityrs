use napi::{bindgen_prelude::FromNapiValue, NapiValue};

use crate::utils::node_converter::*;
use crate::world::world::world::World;

pub struct WorldTickSignal {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub world: World,
}

impl FromNapiValue for WorldTickSignal {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Create the world instance
    let world_object = get_node_object(&object, "world");
    let world = match world_object {
      Ok(world) => World::new(env.into(), world),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    Ok(WorldTickSignal { env: env.into(), object, world })
  }
}