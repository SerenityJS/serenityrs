use crate::utils::node_converter::*;

pub struct PlayerHealthComponent {
  pub object: napi::JsObject,
  pub env: napi::Env,

  pub identifier: String,
  pub effective_min: i32,
  pub effective_max: i32,
  pub default_value: i32,
}

impl PlayerHealthComponent {
  pub fn new(env: napi::Env, object: napi::JsObject) -> Self {
    // Get the identifier
    let identifier_object = get_node_string(&object, "identifier").unwrap();
    let identifier = identifier_object.into_utf8().unwrap().into_owned().unwrap();

    // Get the effective min
    let effective_min_object = get_node_number(&object, "effectiveMin").unwrap();
    let effective_min = effective_min_object.get_int32().unwrap();

    // Get the effective max
    let effective_max_object = get_node_number(&object, "effectiveMax").unwrap();
    let effective_max = effective_max_object.get_int32().unwrap();

    // Get the default value
    let default_value_object = get_node_number(&object, "defaultValue").unwrap();
    let default_value = default_value_object.get_int32().unwrap();

    PlayerHealthComponent { env, object, identifier, effective_min, effective_max, default_value }
  }

  /**
   * Get the current value of the player health component
  */
  pub fn get_current_value(&self) -> i32 {
    // Get the get current value function
    let get_current_value = get_node_func(&self.object, "getCurrentValue").unwrap();

    // Call the get current value function
    let current_value_object = get_current_value.call::<napi::JsNumber>(Some(&self.object), &[]).unwrap();

    // Get the current value
    let current_value = current_value_object.coerce_to_number().unwrap().get_int32().unwrap();

    // Return the current value
    return current_value;
  }

  /**
   * Sets the current value of the player health component
  */
  pub fn set_current_value(&self, value: i32) {
    // Get the set current value function
    let set_current_value = get_node_func(&self.object, "setCurrentValue").unwrap();

    // Convert the value to a JsNumber
    let value = convert_to_js_number(&self.env, value).unwrap();

    // Call the set current value function
    set_current_value.call::<napi::JsNumber>(Some(&self.object), &[value]).unwrap();
  }

  /** 
   * Applies damage to the player health component
  */
  pub fn apply_damage(&self, damage: i32, cause: Option<i32>) {
    // Get the apply damage function
    let apply_damage = get_node_func(&self.object, "applyDamage").unwrap();

    // Convert the damage to a JsNumber
    let damage = convert_to_js_number(&self.env, damage).unwrap();

    // Convert the cause to a JsNumber
    let cause = match cause {
      Some(cause) => {
        let cause = convert_to_js_number(&self.env, cause).unwrap();
        Some(cause)
      },
      None => None
    };

    // Call the apply damage function
    match cause {
      Some(cause) => apply_damage.call::<napi::JsNumber>(Some(&self.object), &[damage, cause]).unwrap(),
      None => apply_damage.call::<napi::JsNumber>(Some(&self.object), &[damage]).unwrap()
    };
  }
}