use registers::WDOG;
use embedded_hal::watchdog;

pub struct Watchdog {
    wdog: WDOG,
}

pub trait WatchdogExt {
    fn constrain(self) -> Watchdog;
}

impl WatchdogExt for WDOG {
    fn constrain(self) -> Watchdog {
        Watchdog { wdog: self }
    }
}

impl watchdog::Watchdog for Watchdog {
    fn feed(&mut self) {
        self.wdog.cmd.write(|w| w.clear().set_bit());
    }
}

impl watchdog::WatchdogDisable for Watchdog {
    fn disable(&mut self) {
        self.wdog.ctrl.modify(|_, w| w.en().clear_bit());
    }
}
