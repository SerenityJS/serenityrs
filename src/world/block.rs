use napi::{bindgen_prelude::FromNapiValue, NapiValue};

use crate::utils::node_converter::*;
use super::world::dimension::Dimension;
use crate::block::block_type::BlockType;

pub struct Block {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub dimension: Dimension,
}

impl Block {
  pub fn new(env: napi::Env, object: napi::JsObject) -> Self {
    // Get the dimension
    let dimension_object = get_node_object(&object, "dimension").unwrap();
    let dimension = Dimension::new(env.clone(), dimension_object);

    // Return the Block instance
    Block { env, object, dimension }
  }

  /**
   * Gte he block type of the block.
  */
  pub fn get_type(&self) -> BlockType {
    // Get the getType function
    let get_type = get_node_func(&self.object, "getType").unwrap();

    // Call the getType function
    let get_type_result = get_type.call::<napi::JsUnknown>(Some(&self.object), &[]).unwrap();

    // Coerce the result to an object
    let get_type_object = get_type_result.coerce_to_object().unwrap();

    // Create the BlockType instance
    BlockType::new(self.env.clone(), get_type_object)
  }
}

impl FromNapiValue for Block {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the Block instance
    Ok(Block::new(env.into(), object))
  }
}