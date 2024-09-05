use napi::{bindgen_prelude::FromNapiValue, NapiValue};

use crate::utils::node_converter::*;
use crate::protocol::types::block_position::BlockPosition;
use crate::world::block::Block;

pub struct Dimension {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub identifier: String,
  pub view_distance: i32,
  pub simulation_distance: i32,
}

impl Dimension {
  pub fn new(env: napi::Env, object: napi::JsObject) -> Self {
    // Get the identifier
    let identifier_object = get_node_string(&object, "identifier").unwrap();
    let identifier = identifier_object.into_utf8().unwrap().into_owned().unwrap();


    // Get the view distance
    let view_distance_object = get_node_number(&object, "viewDistance").unwrap();
    let view_distance = view_distance_object.get_int32().unwrap();

    // Get the simulation distance
    let simulation_distance_object = get_node_number(&object, "simulationDistance").unwrap();
    let simulation_distance = simulation_distance_object.get_int32().unwrap();

    Dimension { env, object, identifier, view_distance, simulation_distance }
  }

  /**
   * Get the block at the given position.
  */
  pub fn get_block(&self, position: BlockPosition) -> Block {
    // Get the getBlock function
    let get_block = get_node_func(&self.object, "getBlock").unwrap();

    // Create the position object
    let position_object = position.to_js_object(self.env.clone());


    // Call the getBlock function
    let get_block_result = get_block.call::<napi::JsUnknown>(Some(&self.object), &[position_object.into_unknown()]).unwrap();

    // Coerce the result to an object
    let get_block_object = get_block_result.coerce_to_object().unwrap();

    // Create the Block instance
    Block::new(self.env.clone(), get_block_object)
  }
}

impl FromNapiValue for Dimension {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the Dimension instance
    Ok(Dimension::new(env.into(), object))
  }
}