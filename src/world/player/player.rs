use napi::{bindgen_prelude::FromNapiValue, NapiValue};

use crate::protocol::types::vector3f::Vector3f;
use crate::utils::node_converter::*;
use crate::world::world::dimension::Dimension;

pub struct Player {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub username: String,
  pub xuid: String,
  pub uuid: String,
  pub position: Vector3f,
  pub dimension: Dimension,
}

impl Player {
  pub fn new(env: napi::Env, object: napi::JsObject) -> Self {
    // Get the username object
    let username_object = get_node_string(&object, "username");
    let username = match username_object {
      Ok(username) => username.into_utf8().unwrap().into_owned().unwrap(),
      Err(_) => "".to_string()
    };

    // Get the xuid object
    let xuid_object = get_node_string(&object, "xuid");
    let xuid = match xuid_object {
      Ok(xuid) => xuid.into_utf8().unwrap().into_owned().unwrap(),
      Err(_) => "".to_string()
    };

    // Get the uuid object
    let uuid_object = get_node_string(&object, "uuid");
    let uuid = match uuid_object {
      Ok(uuid) => uuid.into_utf8().unwrap().into_owned().unwrap(),
      Err(_) => "".to_string()
    };

    // Get the position object
    let position_object = get_node_object(&object, "position").unwrap();
    let position = Vector3f::new(position_object);

    // Get the dimension object
    let dimension_object = get_node_object(&object, "dimension").unwrap();
    let dimension = Dimension::new(env.clone(), dimension_object);

    // Return the Player instance
    Player { object, env, username, xuid, uuid, position, dimension }
  }

  /**
   * Gets the player's current gamemode.
  */
  pub fn get_gamemode(&self) -> i32 {
    // Get the gamemode object
    let gamemode_object = get_node_number(&self.object, "gamemode");
    let gamemode = match gamemode_object {
      Ok(gamemode) => gamemode.get_int32().unwrap(),
      Err(_) => 0
    };

    // Return the gamemode
    return gamemode
  }

  /**
   * Sets the player's gamemode.
  */
  pub fn set_gamemode(&self, gamemode: i32) {
    // Get the set_gamemode function
    let set_gamemode = get_node_func(&self.object, "setGamemode").unwrap();

    // Convert the gamemode to a JsNumber
    let gamemode = convert_to_js_number(&self.env, gamemode).unwrap();

    // Call the set_gamemode function
    set_gamemode.call::<napi::JsNumber>(Some(&self.object), &[gamemode]).unwrap();
  }

  /** 
   * Syncs the player's data with the server.
  */
  pub fn sync(&self) {
    // Get the sync function
    let sync = get_node_func(&self.object, "sync").unwrap();

    // Call the sync function
    sync.call::<napi::JsUndefined>(Some(&self.object), &[]).unwrap();
  }

  /**
   * Checks if the player is an operator.
  */
  pub fn is_op(&self) -> bool {
    // Get the is_op object
    let is_op_object = get_node_boolean(&self.object, "isOp");
    let is_op = match is_op_object {
      Ok(is_op) => is_op.get_value().unwrap(),
      Err(_) => false
    };

    // Return the is_op
    return is_op
  }

  /**
   * Spawns the player in the world.
  */
  pub fn spawn(&self) {
    // Get the spawn function
    let spawn = get_node_func(&self.object, "spawn").unwrap();

    // Call the spawn function
    spawn.call::<napi::JsUndefined>(Some(&self.object), &[]).unwrap();
  }

  /**
   * Respawns the player in the world.
  */
  pub fn respawn(&self) {
    // Get the respawn function
    let respawn = get_node_func(&self.object, "respawn").unwrap();

    // Call the respawn function
    respawn.call::<napi::JsUndefined>(Some(&self.object), &[]).unwrap();
  }

  /**
   * Kills the player.
  */
  pub fn kill(&self) {
    // Get the kill function
    let kill = get_node_func(&self.object, "kill").unwrap();

    // Call the kill function
    kill.call::<napi::JsUndefined>(Some(&self.object), &[]).unwrap();
  }

  /**
   * Checks if the player is hungry.
  */
  pub fn is_hungry(&self) -> bool {
    // Get the is_hungry object
    let is_hungry_object = get_node_boolean(&self.object, "isHungry");
    let is_hungry = match is_hungry_object {
      Ok(is_hungry) => is_hungry.get_value().unwrap(),
      Err(_) => false
    };

    // Return the is_hungry
    return is_hungry
  }

  /**
   * Exhausts the player, decreasing their hunger.
  */
  pub fn exhaust(&self, amount: i32) {
    // Get the exhaust function
    let exhaust = get_node_func(&self.object, "exhaust").unwrap();

    // Convert the amount to a JsNumber
    let amount = convert_to_js_number(&self.env, amount).unwrap();

    // Call the exhaust function
    exhaust.call::<napi::JsNumber>(Some(&self.object), &[amount]).unwrap();
  }

  /**
   * Checks if the player has a specific component.
  */
  pub fn has_component(&self, component: &str) -> bool {
    // Get the has_component function
    let has_component = get_node_func(&self.object, "hasComponent").unwrap();

    // Convert the component to a JsString
    let component = convert_to_js_string(&self.env, component).unwrap();

    // Call the has_component function
    let result = has_component.call::<napi::JsString>(Some(&self.object), &[component]).unwrap();

    // Get the value from the result
    let value = result.coerce_to_bool().unwrap().get_value().unwrap();

    // Return the result
    return value
  }

  /**
   * Gets a component from the player.
  */
  pub fn get_component(&self, component: &str) -> napi::JsObject {
    // Get the get_component function
    let get_component = get_node_func(&self.object, "getComponent").unwrap();

    // Convert the component to a JsString
    let component = convert_to_js_string(&self.env, component).unwrap();

    // Call the get_component function
    let result = get_component.call::<napi::JsString>(Some(&self.object), &[component]).unwrap();

    // Coerce the result to an object
    let result_object = result.coerce_to_object().unwrap();

    // Return the PlayerComponent instance
    return result_object
  }

  /**
   * Removes a component from the player.
  */
  pub fn remove_component(&self, component: &str) {
    // Get the remove_component function
    let remove_component = get_node_func(&self.object, "removeComponent").unwrap();

    // Convert the component to a JsString
    let component = convert_to_js_string(&self.env, component).unwrap();

    // Call the remove_component function
    remove_component.call::<napi::JsString>(Some(&self.object), &[component]).unwrap();
  }

  /**
   * Sends a message to the player.
  */
  pub fn send_message(&self, message: &str) {
    // Get the send_message function
    let send_message = get_node_func(&self.object, "sendMessage").unwrap();

    // Convert the message to a JsString
    let message = convert_to_js_string(&self.env, message).unwrap();

    // Call the send_message function
    send_message.call::<napi::JsString>(Some(&self.object), &[message]).unwrap();
  }

  /**
   * Sends a toast message to the player.
  */
  pub fn send_toast(&self, title: &str, message: &str) {
    // Get the send_toast function
    let send_toast = get_node_func(&self.object, "sendToast").unwrap();

    // Convert the title to a JsString
    let title = convert_to_js_string(&self.env, title).unwrap();

    // Convert the message to a JsString
    let message = convert_to_js_string(&self.env, message).unwrap();

    // Call the send_toast function
    send_toast.call::<napi::JsString>(Some(&self.object), &[title, message]).unwrap();
  }

  /**
   * Teleports the player to a specific position.
  */
  pub fn teleport(&self, position: Vector3f) {
    // Get the teleport function
    let teleport = get_node_func(&self.object, "teleport").unwrap();

    // Convert the position to a JsObject
    let position: napi::JsObject = position.to_js_object(self.env);

    // Call the teleport function
    teleport.call::<napi::JsObject>(Some(&self.object), &[position]).unwrap();
  }

  /**
   * Transfers the player to a different server.
  */
  pub fn transfer(&self, address: &str, port: i32) {
    // Get the transfer function
    let transfer = get_node_func(&self.object, "transfer").unwrap();

    // Convert the address to a JsString
    let address = convert_to_js_string(&self.env, address).unwrap();

    // Convert the port to a JsNumber
    let port = convert_to_js_number(&self.env, port).unwrap();

    // Call the transfer function
    transfer.call::<napi::JsUnknown>(Some(&self.object), &[address.into_unknown(), port.into_unknown()]).unwrap();
  }

  /**
   * Gets the player's total experience.
  */
  pub fn get_total_experience(&self) -> i32 {
    // Get the total_experience object
    let total_experience_object = get_node_number(&self.object, "getTotalExperience");
    let total_experience = match total_experience_object {
      Ok(total_experience) => total_experience.get_int32().unwrap(),
      Err(_) => 0
    };

    // Return the total_experience
    return total_experience
  }

  /**
   * Adds experience to the player.
  */
  pub fn add_experience(&self, amount: i32) {
    // Get the add_experience function
    let add_experience = get_node_func(&self.object, "addExperience").unwrap();

    // Convert the amount to a JsNumber
    let amount = convert_to_js_number(&self.env, amount).unwrap();

    // Call the add_experience function
    add_experience.call::<napi::JsNumber>(Some(&self.object), &[amount]).unwrap();
  }

  /**
   * Checks if the player has a specific ability.
  */
  pub fn has_ability(&self, ability: i32) -> bool {
    // Get the has_ability function
    let has_ability = get_node_func(&self.object, "hasAbility").unwrap();

    // Convert the ability to a JsNumber
    let ability = convert_to_js_number(&self.env, ability).unwrap();

    // Call the has_ability function
    let result = has_ability.call::<napi::JsNumber>(Some(&self.object), &[ability]).unwrap();

    // Get the value from the result
    let value = result.coerce_to_bool().unwrap().get_value().unwrap();

    // Return the result
    return value
  }

  /** 
   * Gets the value of a specific ability.
  */
  pub fn get_ability(&self, ability: i32) -> bool {
    // Get the get_ability function
    let get_ability = get_node_func(&self.object, "getAbility").unwrap();

    // Convert the ability to a JsNumber
    let ability = convert_to_js_number(&self.env, ability).unwrap();

    // Call the get_ability function
    let result = get_ability.call::<napi::JsNumber>(Some(&self.object), &[ability]).unwrap();

    // Get the value from the result
    let value = result.coerce_to_bool().unwrap().get_value().unwrap();

    // Return the result
    return value
  }

  /**
   * Sets the value of a specific ability.
  */
  pub fn set_ability(&self, ability: i32, value: bool) {
    // Get the set_ability function
    let set_ability = get_node_func(&self.object, "setAbility").unwrap();

    // Convert the ability to a JsNumber
    let ability = convert_to_js_number(&self.env, ability).unwrap();

    // Convert the value to a JsBoolean
    let value = convert_to_js_boolean(&self.env, value).unwrap();

    // Call the set_ability function
    set_ability.call::<napi::JsUnknown>(Some(&self.object), &[ability.into_unknown(), value.into_unknown()]).unwrap();
  }

  /** 
   * Plays an effect animation for the player.
  */
  pub fn play_effect_animation(&self, animation: i32) {
    // Get the play_effect_animation function
    let play_effect_animation = get_node_func(&self.object, "playEffectAnimation").unwrap();

    // Convert the animation to a JsNumber
    let animation = convert_to_js_number(&self.env, animation).unwrap();

    // Call the play_effect_animation function
    play_effect_animation.call::<napi::JsNumber>(Some(&self.object), &[animation]).unwrap();
  }
}

impl FromNapiValue for Player {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match napi::JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the Player instance
    Ok(Player::new(env.into(), object))
  }
}