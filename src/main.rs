// https://docs.microsoft.com/en-us/windows/desktop/api/winuser/
// Call EnumWindows to loop through all the windows, then call GetWindowPlacement to get out the information. It will require PInvoke to Windows API, but it's not that difficult, just can find all the information at the PInvoke site.

// Btw, here's a codeproject article for finding a specific window and getting/setting the show state of it, might be a good starting point (the code is in VB.Net, but you could probably just use one of the online VB.Net to C# converters if you don't know VB.Net)
// https://www.codeproject.com/Articles/22255/Find-and-Minimize-or-Maximize-or-Restore-a-Window
// https://github.com/pingzing/rust-vlc-finder/blob/80446376676e96c792100b98fb958aad558decd1/src/main.rs
// https://www.codeproject.com/Tips/1053658/Win-GUI-Programming-In-Rust-Language

// User32 Crate
// extern crate user32;
// WinAPI Crate
// https://docs.rs/winapi/*/x86_64-pc-windows-msvc/winapi/
extern crate winapi;

// Modules
use winapi::shared as Shared;
use Shared::minwindef::{WPARAM, LPARAM, DWORD, UINT};
use winapi::um::winuser as WindowsUser;
use winapi::um::highlevelmonitorconfigurationapi as MonitorConfig;
use winapi::um::physicalmonitorenumerationapi as PhysicalMonitorConfig;

fn main() {
  unsafe {  
    // Struct Declarations
    let window_rect = Shared::windef::RECT {
      left: 0,
      top: 0,
      right: 0,
      bottom: 0,
    };

    let point_min = Shared::windef::POINT { x: 0, y: 0 };

    let point_max = Shared::windef::POINT { x: 0, y: 0 };

    let mut window_placement = WindowsUser::WINDOWPLACEMENT {
      length: 0,
      flags: 0,
      showCmd: 0,
      ptMinPosition: point_min,
      ptMaxPosition: point_max,
      rcNormalPosition: window_rect,
    };

  
    // Monitor PointA:
    let mut monitor_point_max = Shared::windef::POINT { x: 0, y: 0 };

    // Monitor Handle
    let monitor_handle = WindowsUser::MonitorFromPoint(monitor_point_max, WindowsUser::MONITOR_DEFAULTTOPRIMARY);

    // Return the current foreground window
    let foreground_window_handle = WindowsUser::GetForegroundWindow();

    let mut message = WindowsUser::MSG {
      hwnd: foreground_window_handle,
      message: 0 as UINT,
      wParam: 0 as WPARAM,
      lParam: 0 as LPARAM,
      time: 0 as DWORD,
      pt: Shared::windef::POINT { x: 0, y: 0 },
    };

    // Set Keyboard Focus
    WindowsUser::SetFocus(foreground_window_handle);

    loop {   
        let pm = WindowsUser::GetMessageA(&mut message,foreground_window_handle,0, 0);
        if pm == 0 {
            break;
        }

        if message.message == WindowsUser::WM_QUIT {
            break;
        }
        
        if message.message == WindowsUser::WM_KEYDOWN {
          println!("PRESS DOWN");
          break;
        }

        WindowsUser::TranslateMessage(&mut message);
        WindowsUser::DispatchMessageA(&mut message);
    };      
    
    /*
    Return value from GetWindowPlacement
    WINDOWPLACEMENT {
      length: 44,
      flags: 2,
      showCmd: 3,
      ptMinPosition: POINT {
        x: -32000,
        y: -32000
      },
      ptMaxPosition: POINT {
        x: -1,
        y: -1
      },
      rcNormalPosition: RECT {
        left: 237,
        top: 124,
        right: 1477,
        bottom: 892
      }
    }
    */

    // WindowsUser::GetWindowPlacement(foreground_window_handle, &mut window_placement);

    // Set that to the width and height
    // let width = window_placement.rcNormalPosition.right - window_placement.rcNormalPosition.left;
    // let height = window_placement.rcNormalPosition.bottom - window_placement.rcNormalPosition.top;

    // let top_centered = (1080 / 2) - (1000 / 2);
    // let left_centered = (1920 / 2) - (1000 / 2);

    // // Window is maximized
    // match window_placement.showCmd as i32 {
    //   // Window is maximized
    //   WindowsUser::SW_MAXIMIZE => {
    //     WindowsUser::ShowWindow(foreground_window_handle, WindowsUser::SW_RESTORE);
    //   },
    //   // Default
    //   _ => (),
    // };

    // Set foreground window position
    // user32::SetWindowPos(
    //   foreground_window_handle, // Window handle
    //   mem::zeroed(), // A handle to the window to precede the positioned window in the Z order. This parameter must be a window handle or one of the following values.
    //   left_centered, // Left
    //   top_centered,  // Top
    //   1000,         // Width
    //   1000,        // Height
    //   0u32,          // Flags
    // );
  }
}

// Iterate over all windows
// unsafe extern "system" fn enumerate_callback(hwnd: winapi::HWND, _: winapi::LPARAM) -> winapi::BOOL {
//   let mut rect = Shared::windef::RECT {
//     left: 0,
//     top: 0,
//     right: 0,
//     bottom: 0,
//   };

//   user32::GetWindowRect(hwnd, &mut rect);
//   let width = rect.right - rect.left;
//   let height = rect.bottom - rect.top;

//   println!("Width: {} | Height: {}", width, height);

//   return winapi::TRUE;
// }
