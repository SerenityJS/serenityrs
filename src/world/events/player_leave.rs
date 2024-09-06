use napi::bindgen_prelude::FromNapiValue;
use napi::NapiValue;

use crate::utils::node_converter::*;
use crate::world::world::world::World;
use crate::world::player::player::Player;

pub struct PlayerLeaveSignal {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub world: World,
  pub player: Player,
  pub reason: i32,
  pub message: String
}

impl PlayerLeaveSignal {
  pub fn new(env: napi::Env, object: napi::JsObject) -> napi::Result<Self> {
    // Create the world instance
    let world_object = get_node_object(&object, "world");
    let world = match world_object {
      Ok(world) => World::new(env.clone(), world),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Create the player instance
    let player_object = get_node_object(&object, "player");
    let player = match player_object {
      Ok(player) => Player::new(env.clone(), player),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Get the reason object
    let reason_object = get_node_number(&object, "reason");
    let reason = match reason_object {
      Ok(reason) => reason.get_int32().unwrap(),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Get the message object
    let message_object = get_node_string(&object, "message");
    let message = match message_object {
      Ok(message) => message.into_utf8().unwrap().into_owned().unwrap(),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the PlayerLeaveSignal instance
    Ok(PlayerLeaveSignal { env, object, world, player, reason, message })
  }
}

impl FromNapiValue for PlayerLeaveSignal {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the PlayerLeaveSignal instance
    Ok(PlayerLeaveSignal::new(env.into(), object)?)
  }
}