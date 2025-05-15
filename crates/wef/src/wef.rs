use std::ffi::c_void;

use crate::{AppHandler, Error, ffi::*, settings::Settings};

/// Initialize the CEF browser process.
///
/// This function should be called on the main application thread to
/// initialize the CEF browser process.
pub fn init<T>(settings: Settings<T>) -> Result<(), Error>
where
    T: AppHandler,
{
    unsafe {
        extern "C" fn destroy_handler<T>(user_data: *mut c_void) {
            unsafe { _ = Box::from_raw(user_data as *mut T) }
        }

        let c_settings = CSettings {
            locale: to_cstr_ptr_opt(settings.locale.as_deref()),
            cache_path: to_cstr_ptr_opt(settings.cache_path.as_deref()),
            root_cache_path: to_cstr_ptr_opt(settings.root_cache_path.as_deref()),
            external_message_pump: settings.external_message_pump,
            callbacks: CAppCallbacks {
                on_schedule_message_pump_work: crate::app_hander::on_schedule_message_pump_work::<T>,
            },
            userdata: Box::into_raw(Box::new(settings.handler)) as _,
            destroy_userdata: destroy_handler::<T>,
        };

        if !wef_init(&c_settings) {
            return Err(Error::InitializeBrowserProcess);
        }
    }

    Ok(())
}

/// Executes the CEF subprocess.
///
/// This function should be called from the application entry point function
/// to execute a secondary process. It can be used to run secondary
/// processes from the browser client executable.
///
/// If called for the browser process (identified by no "type" command-line
/// value) it will return immediately with a value of `false`.
///
/// If called for a recognized secondary process it will block until the
/// process should exit and then return `true`.
///
/// # Examples
///
/// ```rust, no_run
/// use lbcef::CefError;
///
/// fn main() -> Result<(), CefError> {
///     if lbcef::exec_process() {
///         return;
///     }
///
///     lbcef::init(CefSettings::default());
///     // ... event loop
///     lbcef::shutdown();
/// }
/// ```
pub fn exec_process() -> Result<bool, Error> {
    Ok(unsafe { wef_exec_process() })
}

/// Shuts down the CEF library.
///
/// # Panics
///
/// This function **MUST NOT** be called while any `CefBrowser` instances are
/// still alive. If there are any `CefBrowser` objects that have not been
/// dropped properly at the time of calling this function, it will likely lead
/// to a crash or undefined behavior.
pub fn shutdown() {
    unsafe { wef_shutdown() };
}

/// Perform a single iteration of CEF message loop processing.
///
/// This function is  provided for cases where the CEF message loop must be
/// integrated into an existing application message loop.
pub fn do_message_loop_work() {
    unsafe { wef_do_message_loop_work() };
}
