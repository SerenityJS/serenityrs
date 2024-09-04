use colored::{Colorize, CustomColor};
use napi::{bindgen_prelude::FromNapiValue, JsObject, JsString, NapiValue};

use crate::utils::node_converter::*;

pub struct Logger {
  pub object: JsObject,
  pub env: napi::Env
}

impl Logger {
  /** 
   * Create a new Logger instance
  */
  pub fn new(env: napi::Env, object: JsObject) -> Self {
    Logger { object, env }
  }

  /** 
   * Logs a message to the console
  */
  pub fn log(&self, message: &str) {
    // Get the node function
    let func = get_node_func(&self.object, "log");

    // Check if the function is an error
    if func.is_err() {
      return println!("Failed to get the log function");
    }

    // Format the message
    let message = format!("{}{}{} {}", "[".black(), "Rust".custom_color(CustomColor::new(183, 65, 14)), "]".black(), message);

    // Create a string for the message
    let message = convert_to_js_string(&self.env, &message).unwrap();

    // Unwrap the function
    let func = func.unwrap();

    // Call the function
    func.call::<JsString>(Some(&self.object), &[JsString::from(message)]).unwrap();
  }

  /** 
   * Logs an information message to the console
  */
  pub fn info(&self, message: &str) {
    // Get the node function
    let func = get_node_func(&self.object, "info");

    // Check if the function is an error
    if func.is_err() {
      return println!("Failed to get the info function");
    }

    // Format the message
    let message = format!("{}{}{} {}", "[".black(), "Rust".custom_color(CustomColor::new(183, 65, 14)), "]".black(), message);

    // Create a string for the message
    let message = convert_to_js_string(&self.env, &message).unwrap();

    // Unwrap the function
    let func = func.unwrap();

    // Call the function
    func.call::<JsString>(Some(&self.object), &[JsString::from(message)]).unwrap();
  }

  /** 
   * Logs a warning message to the console
  */
  pub fn warn(&self, message: &str) {
    // Get the node function
    let func = get_node_func(&self.object, "warn");

    // Check if the function is an error
    if func.is_err() {
      return println!("Failed to get the warn function");
    }

    // Format the message
    let message = format!("{}{}{} {}", "[".black(), "Rust".custom_color(CustomColor::new(183, 65, 14)), "]".black(), message);

    // Create a string for the message
    let message = convert_to_js_string(&self.env, &message).unwrap();

    // Unwrap the function
    let func = func.unwrap();

    // Call the function
    func.call::<JsString>(Some(&self.object), &[JsString::from(message)]).unwrap();
  }

  /** 
   * Logs an error message to the console
  */
  pub fn error(&self, message: &str) {
    // Get the node function
    let func = get_node_func(&self.object, "error");

    // Check if the function is an error
    if func.is_err() {
      return println!("Failed to get the error function");
    }

    // Format the message
    let message = format!("{}{}{} {}", "[".black(), "Rust".custom_color(CustomColor::new(183, 65, 14)), "]".black(), message);

    // Create a string for the message
    let message = convert_to_js_string(&self.env, &message).unwrap();

    // Unwrap the function
    let func = func.unwrap();

    // Call the function
    func.call::<JsString>(Some(&self.object), &[JsString::from(message)]).unwrap();
  }

  /** 
   * Logs a success message to the console
  */
  pub fn success(&self, message: &str) {
    // Get the node function
    let func = get_node_func(&self.object, "success");

    // Check if the function is an error
    if func.is_err() {
      return println!("Failed to get the success function");
    }

    // Format the message
    let message = format!("{}{}{} {}", "[".black(), "Rust".custom_color(CustomColor::new(183, 65, 14)), "]".black(), message);

    // Create a string for the message
    let message = convert_to_js_string(&self.env, &message).unwrap();

    // Unwrap the function
    let func = func.unwrap();

    // Call the function
    func.call::<JsString>(Some(&self.object), &[JsString::from(message)]).unwrap();
  }

  /** 
   * Logs a debug message to the console
  */
  pub fn debug(&self, message: &str) {
    // Get the node function
    let func = get_node_func(&self.object, "debug");

    // Check if the function is an error
    if func.is_err() {
      return println!("Failed to get the debug function");
    }

    // Format the message
    let message = format!("{}{}{} {}", "[".black(), "Rust".custom_color(CustomColor::new(183, 65, 14)), "]".black(), message);

    // Create a string for the message
    let message = convert_to_js_string(&self.env, &message).unwrap();

    // Unwrap the function
    let func = func.unwrap();

    // Call the function
    func.call::<JsString>(Some(&self.object), &[JsString::from(message)]).unwrap();
  }
}

impl FromNapiValue for Logger {
  unsafe fn from_napi_value(env: napi::sys::napi_env, value: napi::sys::napi_value) -> napi::Result<Self> {
    // Create the JsObject from the napi_value
    let object = match JsObject::from_raw(env.clone(), value.clone()) {
      Ok(obj) => obj,
      Err(e) => return Err(napi::Error::new(
        napi::Status::GenericFailure,
        e.to_string()
      ))
    };

    // Return the Logger instance
    return Ok(Logger::new(env.into(), object));
  }
}