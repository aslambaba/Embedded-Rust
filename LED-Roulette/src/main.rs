#![deny(unsafe_code)]

#![no_main]
#![no_std]

use aux5::{entry, Delay,Leds,prelude::*}

![entry]

fn main(){
    let (delay,leds): Delay, Leds = aux::init();
    let period = 100_u16;

    loop{
        leds[0].on();
        delay.delay_ms(period);
        leds[0].off();

        leds[2].on();
        delay.delay_ms(period);
        leds[2].off();

        leds[4].on();
        delay.delay_ms(period);
        leds[4].off();
    
        leds[6].on();
        delay.delay_ms(period);
        leds[6].off();
    }
}