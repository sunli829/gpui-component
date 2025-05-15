use std::{ffi::c_void, time::Duration};

/// Represents a handler for application events.
#[allow(unused_variables)]
pub trait AppHandler: Send {
    /// Called from any thread when work has been scheduled for the browser
    /// process main (UI) thread.
    ///
    /// This callback is used in combination with
    /// [`crate::Settings::external_message_pump`] and
    /// [`crate::do_message_loop_work`] in cases where the CEF message loop must
    /// be integrated into an existing application message loop.
    ///
    /// This callback should schedule a [`crate::do_message_loop_work`] call to
    /// happen on the main (UI) thread.
    ///
    /// `delay` is the requested delay.
    fn on_schedule_message_pump_work(&mut self, delay: Duration) {}
}

impl AppHandler for () {}

pub(crate) extern "C" fn on_schedule_message_pump_work<T: AppHandler>(
    userdata: *mut c_void,
    delay_ms: i64,
) {
    unsafe {
        let callbacks = &mut *(userdata as *mut T);
        callbacks.on_schedule_message_pump_work(Duration::from_millis(delay_ms.max(0) as u64));
    }
}
