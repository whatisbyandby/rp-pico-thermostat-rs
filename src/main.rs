#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;

use embedded_aht20::{Aht20, DEFAULT_I2C_ADDRESS};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

#[cfg(not(test))]
use panic_probe as _;

use rp_pico::{self as bsp, hal::fugit};

mod themostat;
use themostat::{ThermostatBuilder, ThermostatCommand};

use rp_pico::hal;

use bsp::hal::{clocks::init_clocks_and_plls, sio::Sio, watchdog::Watchdog};
use rp_pico::hal::pac;

use fugit::RateExtU32;

#[cfg_attr(not(test), entry)]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio2.into_push_pull_output();

    let mut thermostat = ThermostatBuilder::new().build();

    // Configure two pins as being I²C, not GPIO
    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio20.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio21.reconfigure();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        100.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let mut sensor = Aht20::new(i2c, DEFAULT_I2C_ADDRESS, timer).unwrap();

    loop {
        let measurement = sensor.measure().unwrap();
        info!(
            "Temperature: {} °C, Relative humidity: {} %",
            measurement.temperature.celcius(),
            measurement.relative_humidity
        );

        thermostat
            .run()
            .expect("Something went wrong in the thermostat loop");
        thermostat.execute(ThermostatCommand::SetTemperature(25.0));
        led_pin.set_high().unwrap();
        timer.delay_ms(1000u32);
        led_pin.set_low().unwrap();
        timer.delay_ms(1000u32);
    }
}
