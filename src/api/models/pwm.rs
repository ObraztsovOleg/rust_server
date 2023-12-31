pub mod pwm_model {
    use std::sync::Mutex;
    use rppal::pwm::{Channel, Polarity, Pwm};
    use std::time::Duration;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use crate::api::models::globals::pwm;

    pub static mut INTERRUPT: bool = false;

    pub fn pwm_pin_period (channel: Channel, period_ms: u64, pulse_us: u64, enable: bool) -> Pwm {
        return match Pwm::with_period (
            channel,
            Duration::from_millis(period_ms),
            Duration::from_micros(pulse_us),
            Polarity::Normal,
            enable,
        ) {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        }
    }

    pub fn pwm_pin_frequency (channel: Channel, frequency: f64, duty_cycle: f64, enable: bool) -> Pwm {
        return match Pwm::with_frequency (
            channel,
            frequency,
            duty_cycle,
            Polarity::Normal,
            enable,
        ) {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        }
    }

    pub static mut PWM_STATE: Lazy<HashMap<u8, Mutex<Pwm>>> = Lazy::new(||
        HashMap::from([
            (pwm::PIN_12, Mutex::new(
                pwm_pin_frequency(Channel::Pwm0, pwm::FREQUENCY, pwm::DUTY_CYCLE_MIN, false)
            )),
            (pwm::PIN_13, Mutex::new(
                pwm_pin_period(Channel::Pwm1, pwm::PERIOD, pwm::SERVO_AVG_PULSE, true)
            )),
        ])
    );
}