use std::{error::Error};
use std::thread;
use std::time::Duration;

use blurz::{BluetoothAdapter, BluetoothDevice, BluetoothDiscoverySession, BluetoothSession};
use ruuvi_sensor_protocol::{Acceleration, AccelerationVector, SensorValues};

static RUUVI_MANUFACTURER_ID: u16 = 0x0499;
static DISCOVERY_WAIT: u64 = 0;

#[derive(Debug)]
pub struct RuuviDevice {
  acceleration: AccelerationVector,
}

impl RuuviDevice {
  fn new(device: BluetoothDevice) -> Option<Self> {
    device
      .get_manufacturer_data()
      .ok()
      .and_then(|data| data.get(&RUUVI_MANUFACTURER_ID).map(|d| d.clone()))
      .and_then(|data| {
        SensorValues::from_manufacturer_specific_data(RUUVI_MANUFACTURER_ID, &data).ok()
      })
      .and_then(|values| values.acceleration_vector_as_milli_g())
      .and_then(|acc| {
        Some(RuuviDevice {
          acceleration: acc.clone(),
        })
      })
  }

  pub fn get_acceleration(&self) -> AccelerationVector {
    self.acceleration
  }
}

pub fn find_ruuvi_devices() -> Result<Vec<RuuviDevice>, Box<dyn Error>> {
  let session = BluetoothSession::create_session(None)?;
  let adapter = BluetoothAdapter::init(&session)?;
  let discovery_session = BluetoothDiscoverySession::create_session(&session, adapter.get_id())?;
  discovery_session.start_discovery()?;
  thread::sleep(Duration::from_secs(DISCOVERY_WAIT));
  let rv = adapter.get_device_list().map(|device_ids| {
    device_ids
      .iter()
      .map(|device_id| BluetoothDevice::new(&session, device_id.to_string()))
      .filter_map(|device| RuuviDevice::new(device))
      .collect::<Vec<RuuviDevice>>()
  });
  discovery_session.stop_discovery()?;
  rv
}
