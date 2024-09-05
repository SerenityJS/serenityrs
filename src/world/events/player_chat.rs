use napi::bindgen_prelude::FromNapiValue;
use napi::{JsObject, NapiValue};

use crate::utils::node_converter::*;
use crate::world::world::world::World;
use crate::world::player::player::Player;

pub struct PlayerChatSignal {
  pub object: JsObject,
  pub env: napi::Env,
  pub world: World,
  pub player: Player,
  pub message: String
}

impl FromNapiValue for PlayerChatSignal {
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

    // Create the player instance
    let player_object = get_node_object(&object, "player");
    let player = match player_object {
      Ok(player) => Player::new(env.into(), player),
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

    Ok(PlayerChatSignal { env: env.into(), object, world, player, message })
  }
}