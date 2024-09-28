# XGZP6897D - differential pressure driver
A rust device driver for the XGZP6897D differential pressure sensor.

[Datasheet](https://cfsensor.com/wp-content/uploads/2022/11/XGZP6897D-Pressure-Sensor-V2.7.pdf)
| [Product page](https://cfsensor.com/product/i2c-differential-pressure-sensor-xgzp6897d/)
| [Dimensions](https://cfsensor.com/wp-content/uploads/2022/11/6897D-DIMENSION.png)
## Usage
```rust
let mut device = XGZP6897D::new(i2c, xgzp6897d::DEVICE_ADDRESS, 4096f32);
loop {
    match device.read_sensor() {
        Ok((pressure, temperature)) => {
            log::info!(
                "Pressure: {:.3} Pa; Temperature: {:.2} °C", 
                pressure, 
                temperature
            );
        }
        Err(_) => log::error!("Failed to read XGZP6897D!"),
    }

    FreeRtos.delay_ms(1000);
}
``` 


## License
Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

