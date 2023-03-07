use std::{thread::sleep, time::Duration};

use adafruit_motorkit::{dc::DcMotor, init_pwm, Motor};
use gilrs::{Button, Event, EventType, Gilrs};

fn get_motor_rotation() {

}

fn main() {

    let mut pwm = init_pwm(None).unwrap();
    let mut dc_motor1 = DcMotor::try_new(&mut pwm, Motor::Motor1).unwrap();
    let mut dc_motor2 = DcMotor::try_new(&mut pwm, Motor::Motor2).unwrap();
    let mut gilrs = Gilrs::new().unwrap();
    let mut active_gamepad = None;
    loop {
        // Examine new events
        while let Some(Event { id, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::AxisChanged(x, y, z) => {
                    // change motor speed with the axis values, allowing the car to rotate
                    dc_motor1.set_throttle(&mut pwm, y);
                    dc_motor2.set_throttle(&mut pwm, -y );

                }
                EventType::Connected => {
                    active_gamepad = Some(id);
                }
                _ => {}
            }
        }
        dc_motor1.stop(&mut pwm);
        dc_motor2.stop(&mut pwm);
    }
}
