use serde_json;

#[derive(Serialize, Debug, Clone)]
pub struct Device {
  id: i64,
  user: i64,
  name: String,
  description: String,
  createdate: i64,
  changeddate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SaveDevice {
  id: Option<i64>,
  name: String,
  description: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Sensor {
  id: i64,
  device: i64,
  name: String,
  description: String,
  createdate: i64,
  changeddate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SaveSensor {
  id: Option<i64>,
  device: i64,
  name: String,
  description: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Measurement {
  id: i64,
  value: f64,
  sensor: i64,
  createdate: i64,
  measuredate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SaveMeasurement {
  value: f64,
  sensor: i64,
  measuredate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MeasurementQuery {
  pub sensor: i64,
  enddate: Option<i64>,
  lengthOfTime: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  pub id: i64,
  pub name: String,
  pub hashwd: String,
  pub salt: String,
  pub email: String,
  pub registration_key: Option<String>,
}
