use napi::{bindgen_prelude::FromNapiValue, NapiValue};
use napi_derive::napi;

use crate::utils::node_converter::*;

#[napi]
#[derive(Debug)]
pub struct Vector3f {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector3f {
  pub fn new(object: napi::JsObject) -> Self {
    // Get the x object
    let x_object = get_node_float64(&object, "x").unwrap();
    let x = x_object.get_double().unwrap();

    // Get the y object
    let y_object = get_node_float64(&object, "y").unwrap();
    let y = y_object.get_double().unwrap();

    // Get the z object
    let z_object = get_node_float64(&object, "z").unwrap();
    let z = z_object.get_double().unwrap();

    Vector3f { x, y, z }
  }
}

impl Vector3f {
  pub fn to_js_object(&self, env: napi::Env) -> napi::JsObject {
    let mut object = create_node_object(&env).unwrap();

    let x = create_node_float64(&env, self.x).unwrap();
    let y = create_node_float64(&env, self.y).unwrap();
    let z = create_node_float64(&env, self.z).unwrap();

    object.set_named_property("x", x).unwrap();
    object.set_named_property("y", y).unwrap();
    object.set_named_property("z", z).unwrap();

    object
  }
}

impl FromNapiValue for Vector3f {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the Vector3f instance
    Ok(Vector3f::new(object))
  }
}