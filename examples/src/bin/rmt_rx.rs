//! Demonstrates decoding pulse sequences with RMT
//!
//! The following wiring is assumed:
//! - Input pin  => GPIO4
//! - Output pin => GPIO5
//!
//! Connect GPIO5 to GPIO4

//% CHIPS: esp32 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    prelude::*,
    rmt::{PulseCode, Rmt, RxChannel, RxChannelConfig, RxChannelCreator},
};
use esp_println::{print, println};

const WIDTH: usize = 80;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut out = Output::new(peripherals.GPIO5, Level::Low);

    cfg_if::cfg_if! {
        if #[cfg(feature = "esp32h2")] {
            let freq = 32.MHz();
        } else {
            let freq = 80.MHz();
        }
    };

    let rmt = Rmt::new(peripherals.RMT, freq).unwrap();

    let rx_config = RxChannelConfig {
        clk_divider: 1,
        idle_threshold: 10000,
        ..RxChannelConfig::default()
    };

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "esp32", feature = "esp32s2"))] {
            let mut channel = rmt.channel0.configure(peripherals.GPIO4, rx_config).unwrap();
        } else if #[cfg(feature = "esp32s3")] {
            let mut channel = rmt.channel7.configure(peripherals.GPIO4, rx_config).unwrap();
        } else {
            let mut channel = rmt.channel2.configure(peripherals.GPIO4, rx_config).unwrap();
        }
    }

    let delay = Delay::new();

    let mut data: [u32; 48] = [PulseCode::empty(); 48];

    loop {
        for x in data.iter_mut() {
            x.reset()
        }

        let transaction = channel.receive(&mut data).unwrap();

        // simulate input
        for i in 0u32..5u32 {
            out.set_high();
            delay.delay_micros(i * 10 + 20);
            out.set_low();
            delay.delay_micros(i * 20 + 20);
        }

        match transaction.wait() {
            Ok(channel_res) => {
                channel = channel_res;
                let mut total = 0usize;
                for entry in &data[..data.len()] {
                    if entry.length1() == 0 {
                        break;
                    }
                    total += entry.length1() as usize;

                    if entry.length2() == 0 {
                        break;
                    }
                    total += entry.length2() as usize;
                }

                for entry in &data[..data.len()] {
                    if entry.length1() == 0 {
                        break;
                    }

                    let count = WIDTH / (total / entry.length1() as usize);
                    let c = if entry.level1() { '-' } else { '_' };
                    for _ in 0..count + 1 {
                        print!("{}", c);
                    }

                    if entry.length2() == 0 {
                        break;
                    }

                    let count = WIDTH / (total / entry.length2() as usize);
                    let c = if entry.level2() { '-' } else { '_' };
                    for _ in 0..count + 1 {
                        print!("{}", c);
                    }
                }

                println!();
            }
            Err((_err, channel_res)) => {
                channel = channel_res;
                println!("Error");
            }
        }

        delay.delay_millis(1500);
    }
}
