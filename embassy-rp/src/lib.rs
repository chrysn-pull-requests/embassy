#![no_std]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

mod intrinsics;

pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod interrupt;
pub mod rom_data;
pub mod rtc;
pub mod spi;
#[cfg(feature = "time-driver")]
pub mod timer;
pub mod uart;
#[cfg(feature = "nightly")]
pub mod usb;

mod clocks;
mod reset;

// Reexports

pub use embassy_cortex_m::executor;
pub use embassy_cortex_m::interrupt::_export::interrupt;
pub use embassy_hal_common::{into_ref, Peripheral, PeripheralRef};
#[cfg(feature = "unstable-pac")]
pub use rp2040_pac2 as pac;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use rp2040_pac2 as pac;

embassy_hal_common::peripherals! {
    PIN_0,
    PIN_1,
    PIN_2,
    PIN_3,
    PIN_4,
    PIN_5,
    PIN_6,
    PIN_7,
    PIN_8,
    PIN_9,
    PIN_10,
    PIN_11,
    PIN_12,
    PIN_13,
    PIN_14,
    PIN_15,
    PIN_16,
    PIN_17,
    PIN_18,
    PIN_19,
    PIN_20,
    PIN_21,
    PIN_22,
    PIN_23,
    PIN_24,
    PIN_25,
    PIN_26,
    PIN_27,
    PIN_28,
    PIN_29,
    PIN_QSPI_SCLK,
    PIN_QSPI_SS,
    PIN_QSPI_SD0,
    PIN_QSPI_SD1,
    PIN_QSPI_SD2,
    PIN_QSPI_SD3,

    UART0,
    UART1,

    SPI0,
    SPI1,

    I2C0,
    I2C1,

    DMA_CH0,
    DMA_CH1,
    DMA_CH2,
    DMA_CH3,
    DMA_CH4,
    DMA_CH5,
    DMA_CH6,
    DMA_CH7,
    DMA_CH8,
    DMA_CH9,
    DMA_CH10,
    DMA_CH11,

    USB,

    RTC,
}

#[link_section = ".boot2"]
#[used]
static BOOT2: [u8; 256] = *include_bytes!("boot2.bin");

pub mod config {
    #[non_exhaustive]
    pub struct Config {}

    impl Default for Config {
        fn default() -> Self {
            Self {}
        }
    }
}

pub fn init(_config: config::Config) -> Peripherals {
    // Do this first, so that it panics if user is calling `init` a second time
    // before doing anything important.
    let peripherals = Peripherals::take();

    unsafe {
        clocks::init();
        #[cfg(feature = "time-driver")]
        timer::init();
        dma::init();
    }

    peripherals
}

/// Extension trait for PAC regs, adding atomic xor/bitset/bitclear writes.
trait RegExt<T: Copy> {
    unsafe fn write_xor<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    unsafe fn write_set<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    unsafe fn write_clear<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
}

impl<T: Default + Copy, A: pac::common::Write> RegExt<T> for pac::common::Reg<T, A> {
    unsafe fn write_xor<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        let ptr = (self.ptr() as *mut u8).add(0x1000) as *mut T;
        ptr.write_volatile(val);
        res
    }

    unsafe fn write_set<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        let ptr = (self.ptr() as *mut u8).add(0x2000) as *mut T;
        ptr.write_volatile(val);
        res
    }

    unsafe fn write_clear<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        let ptr = (self.ptr() as *mut u8).add(0x3000) as *mut T;
        ptr.write_volatile(val);
        res
    }
}
