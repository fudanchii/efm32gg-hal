use registers::WDOG;
use embedded_hal::watchdog;

/// Our HAL struct for efm32's watchdog.
/// Wrap WDOG struct from PAC so there's only 1 instance of watchdog.
pub struct Watchdog {
    wdog: WDOG,
}

pub trait WatchdogExt {
    fn constrain(self) -> Watchdog;
}

impl WatchdogExt for WDOG {
    /// Constrain low level peripheral WDOG and expose higher level access
    /// which implements embedded_hal's watchdog API.
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
