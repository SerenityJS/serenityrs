use napi::JsObject;

pub fn get_node_func(object: &JsObject, name: &str) -> napi::Result<napi::JsFunction> {
  // Get the function from the object
  let func = match object.get_named_property::<napi::JsFunction>(name) {
    Ok(f) => f,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the function
  return Ok(func);
}

pub fn get_node_object(object: &JsObject, name: &str) -> napi::Result<napi::JsObject> {
  // Get the object from the object
  let obj = match object.get_named_property::<napi::JsObject>(name) {
    Ok(o) => o,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the object
  return Ok(obj);
}

pub fn get_node_string(object: &JsObject, name: &str) -> napi::Result<napi::JsString> {
  // Get the string from the object
  let string = match object.get_named_property::<napi::JsString>(name) {
    Ok(s) => s,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the string
  return Ok(string);
}

pub fn get_node_bigint(object: &JsObject, name: &str) -> napi::Result<napi::JsBigInt> {
  // Get the bigint from the object
  let bigint = match object.get_named_property::<napi::JsBigInt>(name) {
    Ok(b) => b,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the bigint
  return Ok(bigint);
}

pub fn get_node_number(object: &JsObject, name: &str) -> napi::Result<napi::JsNumber> {
  // Get the number from the object
  let number = match object.get_named_property::<napi::JsNumber>(name) {
    Ok(n) => n,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the number
  return Ok(number);
}

pub fn get_node_boolean(object: &JsObject, name: &str) -> napi::Result<napi::JsBoolean> {
  // Get the boolean from the object
  let boolean = match object.get_named_property::<napi::JsBoolean>(name) {
    Ok(b) => b,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the boolean
  return Ok(boolean);
}

pub fn get_node_float64(object: &JsObject, name: &str) -> napi::Result<napi::JsNumber> {
  // Get the number from the object
  let number = match object.get_named_property::<napi::JsNumber>(name) {
    Ok(n) => n,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the number
  return Ok(number);
}

pub fn create_node_object(env: &napi::Env) -> napi::Result<napi::JsObject> {
  // Create the object from the env
  let object = match env.create_object() {
    Ok(o) => o,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the object
  return Ok(object);
}

pub fn create_node_number(env: &napi::Env, value: i32) -> napi::Result<napi::JsNumber> {
  // Create the number from the env
  let number = match env.create_int32(value) {
    Ok(n) => n,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the number
  return Ok(number);
}

pub fn create_node_float64(env: &napi::Env, value: f64) -> napi::Result<napi::JsNumber> {
  // Create the number from the env
  let number = match env.create_double(value) {
    Ok(n) => n,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the number
  return Ok(number);
}

pub fn create_node_func(env: &napi::Env, name: &str, func: napi::Callback) -> napi::Result<napi::JsFunction> {
  // Create the function from the env
  let function = match env.create_function(name, func) {
    Ok(f) => f,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the function
  return Ok(function);
}

pub fn convert_to_js_string(env: &napi::Env, value: &str) -> napi::Result<napi::JsString> {
  // Create a string from the env
  let string = match env.create_string(value) {
    Ok(s) => s,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the string
  return Ok(string);
}

pub fn convert_to_js_number(env: &napi::Env, value: i32) -> napi::Result<napi::JsNumber> {
  // Create a number from the env
  let number = match env.create_int32(value) {
    Ok(n) => n,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the number
  return Ok(number);
}

pub fn convert_to_js_boolean(env: &napi::Env, value: bool) -> napi::Result<napi::JsBoolean> {
  // Create a boolean from the env
  let boolean = match env.get_boolean(value) {
    Ok(b) => b,
    Err(e) => return Err(napi::Error::new(
      napi::Status::GenericFailure,
      e.to_string()
    ))
  };

  // Return the boolean
  return Ok(boolean);
}