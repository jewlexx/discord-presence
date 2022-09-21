use serde_json::Value;
use uuid::Uuid;

pub fn create_json(mut value: serde_json::Value) -> String {
  let uuid = Uuid::new_v4().to_string();

  let payload = value.as_object_mut().unwrap();
  payload.insert("nonce".to_string(), Value::String(uuid));

  // TODO: RISKY NEED TO FIX ERROR HANDLING
  serde_json::to_string(&payload).unwrap()
}
