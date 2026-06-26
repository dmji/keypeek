use crate::layout_language::LayoutLanguage;

/// Detect the current system keyboard layout language.
/// On Windows this inspects the foreground window's keyboard layout.
/// On other platforms it always returns `English` (stub).
pub fn current_layout_language() -> LayoutLanguage {
    #[cfg(target_os = "windows")]
    {
        windows_layout_language()
    }
    #[cfg(not(target_os = "windows"))]
    {
        LayoutLanguage::English
    }
}

#[cfg(target_os = "windows")]
fn windows_layout_language() -> LayoutLanguage {
    type HKL = isize;

    extern "system" {
        fn GetForegroundWindow() -> isize;
        fn GetWindowThreadProcessId(hWnd: isize, lpdwProcessId: *mut u32) -> u32;
        fn GetKeyboardLayout(idThread: u32) -> HKL;
    }

    unsafe {
        let hwnd = GetForegroundWindow();
        let tid = GetWindowThreadProcessId(hwnd, std::ptr::null_mut());
        let hkl = GetKeyboardLayout(tid);
        // Low word = language ID; 0x0419 = Russian (Russia)
        let lang_id = (hkl as u32) & 0xFFFF;
        if lang_id == 0x0419 {
            LayoutLanguage::Russian
        } else {
            LayoutLanguage::English
        }
    }
}
