use napi::{bindgen_prelude::FromNapiValue, NapiValue};

use crate::utils::node_converter::*;
pub struct Player {
  pub object: napi::JsObject,
  pub env: napi::Env,
  pub username: String,
  pub xuid: String,
  pub uuid: String,
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


    Player { object, env, username, xuid, uuid }
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
    set_gamemode.call::<napi::JsNumber>(Some(&self.object), &[napi::JsNumber::from(gamemode)]).unwrap();
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
    send_message.call::<napi::JsString>(Some(&self.object), &[napi::JsString::from(message)]).unwrap();
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
    send_toast.call::<napi::JsString>(Some(&self.object), &[napi::JsString::from(title), napi::JsString::from(message)]).unwrap();
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

    // Get the username object
    let username_object = get_node_string(&object, "username");
    let username = match username_object {
      Ok(username) => username.into_utf8().unwrap().into_owned().unwrap(),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Get the xuid object
    let xuid_object = get_node_string(&object, "xuid");
    let xuid = match xuid_object {
      Ok(xuid) => xuid.into_utf8().unwrap().into_owned().unwrap(),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Get the uuid object
    let uuid_object = get_node_string(&object, "uuid");
    let uuid = match uuid_object {
      Ok(uuid) => uuid.into_utf8().unwrap().into_owned().unwrap(),
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    Ok(Player { env: env.into(), object, username, xuid, uuid })
  }
}