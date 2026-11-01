//! Keep the display awake while the user is in voice or asked for it
//! (Preferences). Port of the Legacy Viewer's prevent-sleep habit for long
//! voice sessions without mouse/keyboard input.

#[cfg(windows)]
pub fn set_prevent_display_sleep(on: bool) {
    use windows_sys::Win32::System::Power::{
        ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, SetThreadExecutionState,
    };
    // SAFETY: SetThreadExecutionState only toggles the calling thread's
    // execution state flags; arguments are constants.
    unsafe {
        if on {
            SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED);
        } else {
            SetThreadExecutionState(ES_CONTINUOUS);
        }
    }
}

#[cfg(not(windows))]
pub fn set_prevent_display_sleep(_on: bool) {}
