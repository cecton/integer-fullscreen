#![allow(unused_imports)]

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::ffi;
use std::fmt;
use std::iter;
use std::mem;
use std::ptr;

use std::mem::{size_of, zeroed};

use bindings::{
    Windows::Win32::Debug::*, Windows::Win32::DisplayDevices::*, Windows::Win32::Gdi::*,
    Windows::Win32::MenusAndResources::*, Windows::Win32::SystemServices::*,
    Windows::Win32::WindowsAndMessaging::*,
};
use windows::IntoParam;

static SZ_CLASS: &'static [u8] = b"c\0l\0a\0s\0s\0\0\0";
static SZ_TITLE: &'static [u8] = b"t\0i\0t\0l\0e\0\0\0";
static SZ_TEXT: &'static [u8] = b"Hello, world!";

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT::NULL
            }
            WM_PAINT => {
                let mut ps = zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);
                let rect = RECT {
                    left: 0,
                    top: 0,
                    right: 200,
                    bottom: 200,
                };
                FillRect(hdc, &rect, CreateSolidBrush(0x00ff00ff));
                LRESULT::NULL
            }
            /*
            WM_NCHITTEST => {
                println!("something");
                HTTRANSPARENT
            }
            */
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn main() {
    unsafe {
        let mut class: windows::Param<'static, PWSTR> = "class".into_param();
        let h_instance: HINSTANCE = mem::transmute(GetModuleHandleA(PSTR::NULL));
        let wcex = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: WNDCLASS_STYLES::CS_VREDRAW | WNDCLASS_STYLES::CS_HREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: HICON::NULL,
            hCursor: HCURSOR::NULL,
            //hbrBackground: (COLOR_WINDOWFRAME) as HBRUSH,
            hbrBackground: CreateSolidBrush(0x00000000),
            lpszMenuName: PWSTR::NULL,
            lpszClassName: class.abi(),
            hIconSm: HICON::NULL,
        };
        match RegisterClassExW(&wcex) {
            0 => {
                panic!("Call to RegisterClassEx failed!");
            }
            _atom => {
                let window = CreateWindowExW(
                    WINDOW_EX_STYLE::WS_EX_LAYERED
                        | WINDOW_EX_STYLE::WS_EX_TOPMOST
                        | WINDOW_EX_STYLE::WS_EX_TRANSPARENT
                        | WINDOW_EX_STYLE::WS_EX_NOACTIVATE,
                    "class",
                    "title",
                    WINDOW_STYLE::WS_POPUP,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    300,
                    300,
                    HWND::NULL,
                    HMENU::NULL,
                    h_instance,
                    0 as *mut _,
                );
                if window.is_null() {
                    panic!("Call to CreateWindow failed: {}", GetLastError());
                } else {
                    //SetWindowLongPtrW(window, GWL_EXSTYLE, WS_EX_LAYERED as _);
                    if !SetLayeredWindowAttributes(
                        window,
                        0x00ff00ff,
                        0,
                        SetLayeredWindowAttributes_dwFlags::LWA_COLORKEY,
                    )
                    .as_bool()
                    {
                        panic!("failed: {}", GetLastError());
                    }
                    /*
                    let mut ps = zeroed();
                    let hdc = BeginPaint(window, &mut ps);
                    let rect = RECT {
                        left: 0,
                        top: 0,
                        right: 200,
                        bottom: 200,
                    };
                    FillRect(hdc, &rect, CreateSolidBrush(RGB(255, 0, 0)));
                    EndPaint(window, &mut ps);
                    */
                    ShowWindow(window, SHOW_WINDOW_CMD::SW_SHOWDEFAULT);
                    //FreeConsole();
                    let mut msg = zeroed();
                    while GetMessageW(&mut msg, HWND::NULL, 0, 0).as_bool() {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                };
            }
        };
    }
}
