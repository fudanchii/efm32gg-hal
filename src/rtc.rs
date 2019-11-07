/// HAL wrapper for EFM32 Real Time Clock.
pub struct RTC {
    rtc: ::registers::RTC,
}

pub trait RTCExt {
    fn constrain(self) -> RTC;
}

impl RTCExt for ::registers::RTC {
    /// Expose higher level access to peripheral.
    fn constrain(self) -> RTC {
        RTC { rtc: self }
    }
}

impl RTC {
    /// Reset all RTC state to hardware defaults.
    pub fn reset(&mut self) {
        self.rtc.freeze.reset();
        self.rtc.ctrl.reset();
        self.rtc.ien.reset();
        self.clear_all_interrupts();
        self.rtc.comp0.reset();
        self.rtc.comp1.reset();
        self.rtc.cnt.reset();
    }

    /// Cap RTC maximum counter value.
    ///
    /// This can optionally enable an interrupt when the maximum is reached.
    pub fn cap_counter(&mut self, value: u32, interrupt: bool) {
        let val = value.min(0x00FF_FFFF);
        self.rtc.comp0.write(|w| unsafe { w.comp0().bits(val) });
        self.rtc.ctrl.modify(|_, w| w.comp0top().set_bit());
        if interrupt {
            self.rtc.ien.modify(|_, w| w.comp0().set_bit());
        }
    }

    /// Start the RTC.
    pub fn start(&mut self) {
        self.rtc.ctrl.modify(|_, w| w.en().set_bit());
    }

    /// Stop the RTC.
    pub fn stop(&mut self) {
        self.rtc.ctrl.modify(|_, w| w.en().clear_bit());
    }

    /// Set RTC counter value (max: 0x00FFFFFF).
    pub fn set_counter(&mut self, value: u32) {
        let val = value.min(0x00FF_FFFF);
        self.rtc.comp0.write(|w| unsafe { w.comp0().bits(val) });
    }

    /// Read RTC counter value.
    pub fn read_counter(&self) -> u32 {
        let value = self.rtc.comp0.read().bits();
        value & 0x00FF_FFFF
    }

    /// Clear all active interrupts flags.
    pub fn clear_all_interrupts(&mut self) {
        self.rtc
            .ifc
            .write(|w| w.comp0().set_bit().comp1().set_bit().of().set_bit());
    }
}
