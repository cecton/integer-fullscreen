fn main() {
    windows::build!(
        Windows::Foundation::Numerics::{Vector2, Vector3},
        Windows::Foundation::TimeSpan,
        Windows::Graphics::SizeInt32,
        Windows::System::DispatcherQueueController,
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::UI::Composition::{
            AnimationIterationBehavior, CompositionAnimation, CompositionBatchTypes,
            CompositionBorderMode, CompositionColorBrush, CompositionContainerShape,
            CompositionEllipseGeometry, CompositionGeometry, CompositionNineGridBrush,
            CompositionScopedBatch, CompositionShape, CompositionShapeCollection,
            CompositionSpriteShape, Compositor, ContainerVisual, ShapeVisual, SpriteVisual,
            Vector3KeyFrameAnimation, VisualCollection,
        },
        Windows::UI::{Color, Colors},
        Windows::Win32::Debug::{GetLastError},
        Windows::Win32::DisplayDevices::{RECT},
        Windows::Win32::Gdi::{CreateSolidBrush, BeginPaint, EndPaint, FillRect},
        Windows::Win32::MenusAndResources::{HICON, HCURSOR},
        Windows::Win32::SystemServices::{CreateDispatcherQueueController, BOOL, GetModuleHandleA, PWSTR, HINSTANCE, LRESULT, FreeConsole},
        Windows::Win32::WindowsAndMessaging::{HWND, RegisterClassExW, WNDCLASSEXW, WNDCLASS_STYLES, WPARAM, LPARAM, GetMessageW, DispatchMessageW, TranslateMessage, CreateWindowExW, SHOW_WINDOW_CMD, ShowWindow, SetLayeredWindowAttributes_dwFlags, CW_USEDEFAULT, WINDOW_STYLE, WINDOW_EX_STYLE, SetLayeredWindowAttributes, PostQuitMessage, DefWindowProcW, WM_DESTROY, WM_PAINT},
        Windows::Win32::WinRT::ICompositorDesktopInterop,
    );
}
