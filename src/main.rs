#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m;
use k64f::Peripherals;
use k64f::porte::pcr26::MUXW;
use core::borrow::{Borrow, BorrowMut};

fn delay(time: u16, p: &mut Peripherals) {
    let timer = p.LPTMR0.borrow();
    timer.cmr.write(|w| unsafe { w.compare().bits(time) });
    timer.csr.write(|w| w.ten().set_bit());
    while timer.csr.read().tcf().is_0() {

    }
    timer.csr.write(|w| w.tcf().set_bit());
}

#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();

    p.SIM.scgc5.write(|w| w.porte().set_bit().lptmr().set_bit());

    p.PORTE.pcr26.write(|w| w.mux().variant(MUXW::_001));

    p.GPIOE.pddr.write(|w| unsafe { w.bits(1 << 26) });

    p.LPTMR0.psr.write(|w| w.pcs()._01().pbyp().set_bit());
    p.LPTMR0.csr.write(|w| w.ten().set_bit());

    loop {
        p.GPIOE.ptor.write(|w| unsafe { w.bits(1 << 26) });
        delay(500, p.borrow_mut());
    }
}
