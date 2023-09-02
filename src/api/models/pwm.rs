pub mod pwm_model {
    use std::sync::Mutex;
    use rppal::pwm::{Channel, Polarity, Pwm};
    use std::time::Duration;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use crate::api::models::globals::pwm;

    pub fn pwm_pin (channel: Channel, period_ms: u64, pulse_us: u64, enable: bool) -> Pwm {
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

    // pub unsafe fn update_pulse (pin_number: u8, pulse_us: i64) {
    //     let pin = PWM_STATE.get_mut(&pin_number).unwrap();
    //     let pwm_pin = pin.lock().unwrap();

    //     let pulse_duration = pwm_pin.pulse_width().unwrap();
    //     let current_pulse = pulse_duration.as_micros() as i64 + pulse_us;
        
    //     pwm_pin.set_pulse_width(Duration::from_micros(current_pulse as u64)).unwrap();
    // }

    pub static mut PWM_STATE: Lazy<HashMap<u8, Mutex<Pwm>>> = Lazy::new(||
        HashMap::from([
            (pwm::PIN_12, Mutex::new(
                pwm_pin(Channel::Pwm0, pwm::PERIOD, pwm::PULSE, false)
            )),
            (pwm::PIN_13, Mutex::new(
                pwm_pin(Channel::Pwm1, pwm::PERIOD, pwm::SERVO_AVG_PULSE, true)
            )),
        ])
    );
}