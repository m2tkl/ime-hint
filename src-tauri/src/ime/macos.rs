use std::ffi::c_void;

use super::ImeState;

type TISInputSourceRef = *const c_void;
type CFTypeRef = *const c_void;
type CFStringRef = *const c_void;
type CFBooleanRef = *const c_void;
type Boolean = u8;

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
    fn TISCopyCurrentASCIICapableKeyboardInputSource() -> TISInputSourceRef;
    fn TISGetInputSourceProperty(source: TISInputSourceRef, key: CFStringRef) -> *const c_void;

    static kTISPropertyInputSourceIsASCIICapable: CFStringRef;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(cf: CFTypeRef);
    fn CFEqual(a: CFTypeRef, b: CFTypeRef) -> Boolean;
    fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
}

unsafe fn release_if_not_null(cf: CFTypeRef) {
    if !cf.is_null() {
        CFRelease(cf);
    }
}

unsafe fn is_ascii_capable(source: TISInputSourceRef) -> Option<bool> {
    if source.is_null() {
        return None;
    }
    let value = TISGetInputSourceProperty(source, kTISPropertyInputSourceIsASCIICapable);
    if value.is_null() {
        return None;
    }
    Some(CFBooleanGetValue(value as CFBooleanRef) != 0)
}

pub fn current_state() -> ImeState {
    unsafe {
        let current = TISCopyCurrentKeyboardInputSource();
        if current.is_null() {
            return ImeState::Unknown;
        }

        let ascii = TISCopyCurrentASCIICapableKeyboardInputSource();
        if !ascii.is_null() {
            let is_ascii = CFEqual(current as CFTypeRef, ascii as CFTypeRef) != 0;
            release_if_not_null(ascii as CFTypeRef);
            release_if_not_null(current as CFTypeRef);
            return if is_ascii { ImeState::Off } else { ImeState::On };
        }

        let fallback = is_ascii_capable(current);
        release_if_not_null(current as CFTypeRef);
        match fallback {
            Some(true) => ImeState::Off,
            Some(false) => ImeState::On,
            None => ImeState::Unknown,
        }
    }
}
