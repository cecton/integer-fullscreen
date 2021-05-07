#![windows_subsystem = "windows"]
#![allow(unused_imports)]

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use once_cell::sync::OnceCell;
use std::convert::TryFrom;
use std::ffi;
use std::fmt;
use std::iter;
use std::mem;
use std::ptr;

use bindings::{
    Windows::Win32::Debug::*, Windows::Win32::DisplayDevices::*, Windows::Win32::Gdi::*,
    Windows::Win32::KeyboardAndMouseInput::*, Windows::Win32::Magnification::*,
    Windows::Win32::MenusAndResources::*, Windows::Win32::SystemServices::*,
    Windows::Win32::WindowsAndMessaging::*,
};
use windows::IntoParam;

const HOTKEY_TOGGLE_FULLSCREEN: i32 = 100;
static mut APP: OnceCell<App> = OnceCell::new();

#[derive(Debug)]
struct App {
    window: HWND,
    h_instance: HINSTANCE,
    target: Option<Target>,
}

#[derive(Debug)]
struct Target {
    target_hwnd: HWND,
    target_rect: RECT,
    magnification: Magnification,
}

macro_rules! panic {
    ($message:expr) => {{
        std::panic!("{}, last error={:?}", $message, GetLastError());
    }};
}

macro_rules! println {
    ($($tokens:tt)*) => {{
        #[cfg(debug_assertions)]
        std::println!($($tokens)*);
    }};
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT::NULL
            }
            WM_PAINT => {
                if let Some(target) = APP.get().and_then(|x| x.target.as_ref()) {
                    let mut ps = mem::zeroed();
                    let hdc = BeginPaint(hwnd, &mut ps);
                    FillRect(hdc, &target.target_rect, CreateSolidBrush(0x00ff00ff));
                }
                LRESULT::NULL
            }
            WM_RBUTTONUP => {
                PostQuitMessage(0);
                LRESULT::NULL
            }
            WM_HOTKEY => {
                let (id, _) = mem::transmute::<_, (i32, u32)>(wparam);
                match id {
                    HOTKEY_TOGGLE_FULLSCREEN => {
                        println!("toggle fullscreen");
                        toggle_fullscreen();
                    }
                    _ => {}
                }
                LRESULT::NULL
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn main() {
    unsafe {
        let mut class: windows::Param<'static, PWSTR> = "class".into_param();
        let h_instance: HINSTANCE = mem::transmute(GetModuleHandleA(PSTR::NULL));
        let cursor = InvisibleCursor::new(h_instance);
        let wcex = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: WNDCLASS_STYLES::CS_VREDRAW | WNDCLASS_STYLES::CS_HREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: HICON::NULL,
            hCursor: cursor.as_hcursor(),
            hbrBackground: CreateSolidBrush(0x00000000),
            lpszMenuName: PWSTR::NULL,
            lpszClassName: class.abi(),
            hIconSm: HICON::NULL,
        };
        match RegisterClassExW(&wcex) {
            0 => {
                panic!("could not register class");
            }
            _atom => {
                let window = CreateWindowExW(
                    WINDOW_EX_STYLE::WS_EX_LAYERED
                        | WINDOW_EX_STYLE::WS_EX_TOPMOST
                        | WINDOW_EX_STYLE::WS_EX_NOACTIVATE,
                    "class",
                    "title",
                    WINDOW_STYLE::WS_POPUP,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    1,
                    1,
                    HWND::NULL,
                    HMENU::NULL,
                    h_instance,
                    0 as *mut _,
                );
                if window.is_null() {
                    panic!("could not create window");
                } else {
                    if !SetLayeredWindowAttributes(
                        window,
                        0x00ff00ff,
                        0,
                        LAYERED_WINDOW_ATTRIBUTES_FLAGS::LWA_COLORKEY,
                    )
                    .as_bool()
                    {
                        panic!("could not set layered window attributes");
                    }
                    APP.set(App {
                        window,
                        h_instance,
                        target: None,
                    })
                    .unwrap();
                    RegisterHotKey(
                        window,
                        HOTKEY_TOGGLE_FULLSCREEN,
                        HOT_KEY_MODIFIERS::MOD_CONTROL | HOT_KEY_MODIFIERS::MOD_ALT,
                        'F' as u32,
                    );
                    let mut msg = mem::zeroed();
                    while GetMessageW(&mut msg, HWND::NULL, 0, 0).as_bool() {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                };
            }
        };
    }
}

fn toggle_fullscreen() {
    unsafe {
        let mut app = APP.get_mut().unwrap();

        if app.target.take().is_some() {
            ShowWindow(APP.get().unwrap().window, SHOW_WINDOW_CMD::SW_HIDE);
        } else {
            let mut p = POINT::default();
            if !GetCursorPos(&mut p).as_bool() {
                panic!("could not get cursor position");
            }
            let target_hwnd = WindowFromPoint(p);
            BringWindowToTop(target_hwnd);
            SetForegroundWindow(target_hwnd);
            let target_rect = get_client_rect_with_position(target_hwnd);
            println!(
                "{:?} left={} top={} right={} bottom={} {}x{}",
                target_hwnd,
                target_rect.left,
                target_rect.top,
                target_rect.right,
                target_rect.bottom,
                (target_rect.right - target_rect.left),
                (target_rect.bottom - target_rect.top),
            );

            let (screen_w, screen_h, _, (screen_x, screen_y), _) =
                current_display_settings(target_hwnd).unwrap();
            println!(
                "screen size: {}x{}+{}+{}",
                screen_w, screen_h, screen_x, screen_y
            );
            let ratio_h = screen_w
                .checked_div(u32::try_from(target_rect.right - target_rect.left).unwrap_or(0))
                .unwrap_or(screen_w);
            let ratio_v = screen_h
                .checked_div(u32::try_from(target_rect.bottom - target_rect.top).unwrap_or(0))
                .unwrap_or(screen_h);
            let ratio = ratio_h.min(ratio_v);
            println!("ratios: h={} v={} => {}", ratio_h, ratio_v, ratio);

            SetWindowPos(
                app.window,
                HWND_TOPMOST,
                screen_x,
                screen_y,
                screen_w as i32,
                screen_h as i32,
                SET_WINDOW_POS_FLAGS::SWP_SHOWWINDOW,
            );

            app.target = Some(Target {
                magnification: Magnification::new(
                    ratio as f32,
                    target_rect.left
                        - (screen_w as i32 / ratio as i32 - (target_rect.right - target_rect.left))
                            / 2,
                    target_rect.top
                        - (screen_h as i32 / ratio as i32 - (target_rect.bottom - target_rect.top))
                            / 2,
                ),
                target_hwnd,
                target_rect,
            });
        }
    }
}

fn current_display_settings(window: HWND) -> Result<(u32, u32, u32, (i32, i32), u32)> {
    unsafe {
        let hmonitor = MonitorFromWindow(window, MONITOR_FROM_FLAGS::MONITOR_DEFAULTTONEAREST);
        let mut lpmi = MONITORINFOEXA::default();
        lpmi.__AnonymousBase_winuser_L13554_C43.cbSize = mem::size_of::<MONITORINFOEXA>() as u32;
        if !GetMonitorInfoA(hmonitor, mem::transmute(&mut lpmi)).as_bool() {
            panic!("could not get monitor info");
        }
        let mut dev_mode: DEVMODEA = mem::zeroed();
        dev_mode.dmSize = mem::size_of::<DEVMODEA>() as u16;
        let dw_flags = 0;
        if !EnumDisplaySettingsExA(
            mem::transmute::<_, PSTR>(&lpmi.szDevice),
            ENUM_DISPLAY_SETTINGS_MODE::ENUM_CURRENT_SETTINGS,
            &mut dev_mode as *mut _,
            dw_flags,
        )
        .as_bool()
        {
            return Err(anyhow::Error::msg("could not get current display settings"));
        }
        let position = {
            let position = dev_mode.Anonymous1.Anonymous2.dmPosition;
            (position.x, position.y)
        };
        let orientation = dev_mode.Anonymous1.Anonymous2.dmDisplayOrientation;
        Ok((
            dev_mode.dmPelsWidth,
            dev_mode.dmPelsHeight,
            dev_mode.dmDisplayFrequency,
            position,
            orientation,
        ))
    }
}

#[derive(Debug)]
struct Magnification;

impl Magnification {
    fn new(mag_level: f32, x_offset: i32, y_offset: i32) -> Self {
        unsafe {
            MagInitialize();
            if mag_level > 1.0 {
                println!("{} {} {}", mag_level, x_offset, y_offset);
                if !MagSetFullscreenTransform(mag_level, x_offset, y_offset).as_bool() {
                    panic!("could not set magnification");
                }
            }
            Self
        }
    }
}

impl Drop for Magnification {
    fn drop(&mut self) {
        unsafe {
            MagUninitialize();
        }
    }
}

#[derive(Debug)]
struct InvisibleCursor(HCURSOR);

impl InvisibleCursor {
    fn new(h_instance: HINSTANCE) -> Self {
        unsafe {
            static BYTE_ARRAY: &[u8] = &[0];
            Self(CreateCursor(
                h_instance,
                0,
                0,
                1,
                1,
                mem::transmute(BYTE_ARRAY.as_ptr()),
                mem::transmute(BYTE_ARRAY.as_ptr()),
            ))
        }
    }

    fn as_hcursor(&self) -> HCURSOR {
        self.0
    }
}

impl Drop for InvisibleCursor {
    fn drop(&mut self) {
        unsafe {
            DestroyCursor(self.0);
        }
    }
}

unsafe fn get_client_rect_with_position(hwnd: HWND) -> RECT {
    let mut p = POINT { x: 0, y: 0 };
    ClientToScreen(hwnd, &mut p);
    let mut client_rect = RECT::default();
    GetClientRect(hwnd, &mut client_rect);
    RECT {
        left: p.x,
        top: p.y,
        right: p.x + client_rect.right,
        bottom: p.y + client_rect.bottom,
    }
}
