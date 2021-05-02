fn main() {
    windows::build!(
        Windows::Win32::Debug::{
            GetLastError,
        },
        Windows::Win32::DisplayDevices::{
            DEVMODEA,
            POINT,
            RECT,
        },
        Windows::Win32::Gdi::{
            BeginPaint,
            ClientToScreen,
            CreateSolidBrush,
            ENUM_DISPLAY_SETTINGS_MODE,
            EndPaint,
            EnumDisplaySettingsExA,
            FillRect,
            GetMonitorInfoA,
            MONITORINFOEXA,
            MONITOR_FROM_FLAGS,
            MonitorFromWindow,
        },
        Windows::Win32::Magnification::{
            MagInitialize,
            MagSetFullscreenTransform,
            MagUninitialize,
        },
        Windows::Win32::MenusAndResources::{
            HCURSOR,
            HICON,
        },
        Windows::Win32::KeyboardAndMouseInput::{
            HOT_KEY_MODIFIERS,
            RegisterHotKey,
        },
        Windows::Win32::SystemServices::{
            BOOL,
            GetModuleHandleA,
            HINSTANCE,
            LRESULT,
            PWSTR,
        },
        Windows::Win32::WindowsAndMessaging::{
            BringWindowToTop,
            CW_USEDEFAULT,
            CreateCursor,
            CreateWindowExW,
            DefWindowProcW,
            DestroyCursor,
            DispatchMessageW,
            GetClientRect,
            GetCursorPos,
            GetMessageW,
            HTTRANSPARENT,
            HWND,
            HWND_TOPMOST,
            LAYERED_WINDOW_ATTRIBUTES_FLAGS,
            LPARAM,
            PostQuitMessage,
            RegisterClassExW,
            SET_WINDOW_POS_FLAGS,
            SHOW_WINDOW_CMD,
            SetForegroundWindow,
            SetLayeredWindowAttributes,
            SetWindowPos,
            ShowWindow,
            TranslateMessage,
            WINDOW_EX_STYLE,
            WINDOW_STYLE,
            WM_DESTROY,
            WM_HOTKEY,
            WM_PAINT,
            WM_RBUTTONUP,
            WNDCLASSEXW,
            WNDCLASS_STYLES,
            WPARAM,
            WindowFromPoint,
        },
    );
}
