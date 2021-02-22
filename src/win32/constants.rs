use super::{c_types::*, typedef::*};

/// [MAKEINTRESOURCEW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
const fn MAKEINTRESOURCEW(i: WORD) -> LPCWSTR {
  i as ULONG_PTR as _
}

/// ID Cursor: Arrow
pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);

/// Redraw the *full* window if the client area's height changes.
pub const CS_VREDRAW: UINT = 0x0001;

/// Redraw the *full* window if the client area's width changes.
pub const CS_HREDRAW: UINT = 0x0002;

/// Each window of this class gets a unique device context.
pub const CS_OWNDC: UINT = 0x0020;

/// Forces a top-level window onto the taskbar when the window is visible.
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;

/// The window is an overlapped window.
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;

/// The window has a border with a raised edge.
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;

/// The window has a border with a raised edge.
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;

/// The window is an overlapped window.
///
/// Same as the `WS_TILEDWINDOW` style.
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED
  | WS_CAPTION
  | WS_SYSMENU
  | WS_THICKFRAME
  | WS_MINIMIZEBOX
  | WS_MAXIMIZEBOX;

/// The window is an overlapped window.
///
/// An overlapped window has a title bar and a border.
///
/// Same as the `WS_TILED`
/// style.
pub const WS_OVERLAPPED: DWORD = 0x00000000;

/// The window has a title bar (includes the `WS_BORDER` style).
pub const WS_CAPTION: DWORD = 0x00C00000;

/// The window has a window menu on its title bar.
///
/// The `WS_CAPTION` style must
/// also be specified.
pub const WS_SYSMENU: DWORD = 0x00080000;

/// The window has a sizing border.
///
/// Same as the `WS_SIZEBOX` style.
pub const WS_THICKFRAME: DWORD = 0x00040000;

/// The window has a minimize button.
///
/// Cannot be combined with the `WS_EX_CONTEXTHELP` style.
///
/// The `WS_SYSMENU` style must also be specified.
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;

/// The window has a maximize button.
///
/// Cannot be combined with the `WS_EX_CONTEXTHELP` style.
///
/// The `WS_SYSMENU` style must also be specified.
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;

/// Excludes the area occupied by child windows when drawing occurs within the
/// parent window.
///
/// This style is used when creating the parent window.
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;

/// Clips child windows relative to each other.
///
/// In other words, if this window is a child window, then its drawing is
/// clipped by the draw areas of other child windows of the same parent.
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;

/// The window is initially visible.
///
/// This style can be turned on and off by using the `ShowWindow` or
/// `SetWindowPos` functions.
pub const WS_VISIBLE: DWORD = 0x10000000;

/// Activates the window and displays it in its current size and position.
///
/// [ShowWindow](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
pub const SW_SHOW: c_int = 5;

/// Sent as a signal that a window or an application should terminate.
///
/// * `w_param`: not used.
/// * `l_param`: not used.
/// * **Return:** If processed, return 0.
///
/// [WM_CLOSE](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-close)
pub const WM_CLOSE: u32 = 0x0010;

/// Sent when a window is being destroyed.
///
/// It is sent to the window procedure of the window being destroyed after the
/// window is removed from the screen.
///
/// * `w_param`: not used.
/// * `l_param`: not used.
/// * **Return:** If processed, return 0.
///
/// [WM_DESTROY](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
pub const WM_DESTROY: u32 = 0x0002;

/// Sent when the system or another application makes a request to paint a
/// portion of an application's window.
///
/// * `w_param`: not used.
/// * `l_param`: not used.
/// * **Return:** If processed, return 0.
///
/// [WM_PAINT](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-paint)
pub const WM_PAINT: u32 = 0x000F;

/// Indicates a request to terminate the application.
///
/// This is generated when [`PostQuitMessage`] is called.
///
/// * `w_param`: The exit code given in the [`PostQuitMessage`] function.
/// * `l_param`: not used.
/// * **Return:** Not applicable. This message is never sent to a window
///   procedure.
///
/// [WM_QUIT](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-quit)
pub const WM_QUIT: u32 = 0x0012;

/// Non-client Create.
///
/// * `w_param`: not used.
/// * `l_param`: is a `*const CREATESTRUCTW`
/// * **Return:** `TRUE` to continue window creation, or `FALSE` to halt window
///   creation.
///
/// Note(`chrisd` on `#windows-dev`): You have to draw the non-client area when
/// this event comes in. If you're not able to do that yourself, then you should
/// call `DefWindowProcW` as part of handling this event. Otherwise you won't
/// get your window title to display.
///
/// * [WM_NCCREATE](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
/// * [CREATESTRUCTW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_NCCREATE_CONTINUE_CREATION: LRESULT = 1 as _;
pub const WM_NCCREATE_HALT_CREATION: LRESULT = 0 as _;

/// Sent when an application requests that a window be created by calling the
/// `CreateWindowEx` or `CreateWindow` function.
///
/// This message is sent *before* the `CreateWindow{Ex}` function returns.
///
/// The window procedure of the new window receives this message after the
/// window is created, but before the window becomes visible.
///
/// * `w_param`: not used.
/// * `l_param`: is a `*const CREATESTRUCTW`
/// * **Return:** 0 to continue window creation, or -1 to halt window creation.
///
/// See
/// * [WM_CREATE](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// * [CREATESTRUCTW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
pub const WM_CREATE: u32 = 0x0001;
pub const WM_CREATE_CONTINUE_CREATION: LRESULT = 0 as _;
pub const WM_CREATE_HALT_CREATION: LRESULT = -1 as _;

pub const PFD_TYPE_RGBA: u8 = 0;
pub const PFD_TYPE_COLORINDEX: u8 = 1;
pub const PFD_MAIN_PLANE: u8 = 0;
pub const PFD_OVERLAY_PLANE: u8 = 1;
pub const PFD_UNDERLAY_PLANE: u8 = u8::MAX /* was (-1) */;
pub const PFD_DOUBLEBUFFER: u32 = 0x00000001;
pub const PFD_STEREO: u32 = 0x00000002;
pub const PFD_DRAW_TO_WINDOW: u32 = 0x00000004;
pub const PFD_DRAW_TO_BITMAP: u32 = 0x00000008;
pub const PFD_SUPPORT_GDI: u32 = 0x00000010;
pub const PFD_SUPPORT_OPENGL: u32 = 0x00000020;
pub const PFD_GENERIC_FORMAT: u32 = 0x00000040;
pub const PFD_NEED_PALETTE: u32 = 0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE: u32 = 0x00000100;
pub const PFD_SWAP_EXCHANGE: u32 = 0x00000200;
pub const PFD_SWAP_COPY: u32 = 0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS: u32 = 0x00000800;
pub const PFD_GENERIC_ACCELERATED: u32 = 0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW: u32 = 0x00002000;
pub const PFD_DIRECT3D_ACCELERATED: u32 = 0x00004000;
pub const PFD_SUPPORT_COMPOSITION: u32 = 0x00008000;

/// use with [`ChoosePixelFormat`] only
pub const PFD_DEPTH_DONTCARE: u32 = 0x20000000;

/// use with [`ChoosePixelFormat`] only
pub const PFD_DOUBLEBUFFER_DONTCARE: u32 = 0x40000000;

/// use with [`ChoosePixelFormat`] only
pub const PFD_STEREO_DONTCARE: u32 = 0x80000000;

/// [GetWindowLongPtrW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw): User Data
pub const GWLP_USERDATA: c_int = -21;

/// Message **are not** removed from the queue after processing by
/// [`PeekMessageW`]
pub const PM_NOREMOVE: UINT = 0x0000;

/// Message **are** removed from the queue after processing by [`PeekMessageW`]
pub const PM_REMOVE: UINT = 0x0001;

/// Prevents the system from releasing any thread that is waiting for this
/// thread to go idle.
///
/// Combine with either [`PM_NOREMOVE`] or [`PM_REMOVE`]
pub const PM_NOYIELD: UINT = 0x0002;

// TODO(PeekMessageW): PM_QS_INPUT, PM_QS_PAINT, PM_QS_POSTMESSAGE,
// PM_QS_SENDMESSAGE
