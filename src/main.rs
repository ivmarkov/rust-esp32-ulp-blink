#![no_std]
#![no_main]
#![feature(core_intrinsics, start)]

use riscv_rt::entry;
use esp_idf_hal::prelude::*;
use esp_idf_hal::delay;
use esp_idf_hal::ulp::sys::cpu::*;

extern crate panic_halt;

#[no_mangle]
static mut CYCLES: u32 = 10;

#[entry]
unsafe fn start() -> ! {
    main()
}

fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut delay = delay::Ulp;
    let mut led = pins.gpio16.into_output().unwrap();

    while get_cycles() > 0 {
        led.set_high().unwrap();
        delay.delay_ms(1000 as u32);

        led.set_low().unwrap();

        decr_cycles();
    }

    unsafe {
        wakeup_main_processor();
        shutdown()
    }
}

fn get_cycles() -> u32 {
    unsafe {
        core::ptr::read_volatile(&mut CYCLES)
    }
}

fn decr_cycles() {
    unsafe {
        let cycles = core::ptr::read_volatile(&mut CYCLES);

        if cycles > 0 {
            core::ptr::write_volatile(&mut CYCLES, cycles - 1);
        }
    }
}
