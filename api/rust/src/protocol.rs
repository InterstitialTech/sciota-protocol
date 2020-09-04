use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ServerResponse {
  pub what: String,
  pub content: Value,
}

#[derive(Deserialize, Debug)]
pub struct UserMessage {
  pub uid: String,
  pub pwd: String,
  pub what: String,
  pub data: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct PublicMessage {
  pub what: String,
  pub data: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct RegistrationData {
  pub email: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Device {
  pub id: i64,
  pub user: i64,
  pub name: String,
  pub description: String,
  pub createdate: i64,
  pub changeddate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SaveDevice {
  pub id: Option<i64>,
  pub name: String,
  pub description: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Sensor {
  pub id: i64,
  pub device: i64,
  pub name: String,
  pub description: String,
  pub createdate: i64,
  pub changeddate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SaveSensor {
  pub id: Option<i64>,
  pub device: i64,
  pub name: String,
  pub description: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Measurement {
  pub id: i64,
  pub value: f64,
  pub sensor: i64,
  pub createdate: i64,
  pub measuredate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SaveMeasurement {
  pub value: f64,
  pub sensor: i64,
  pub measuredate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MeasurementQuery {
  pub sensor: i64,
  pub enddate: Option<i64>,
  pub lengthOfTime: Option<i64>,
}
