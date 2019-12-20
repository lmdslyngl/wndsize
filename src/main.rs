
use std::{env, ptr};
use winapi::um::winnt::WCHAR;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{LPARAM, BOOL, TRUE};
use winapi::um::winuser::{
    EnumWindows, GetWindowTextW, FindWindowW,
    SetWindowPos, SWP_NOMOVE};
use getopts::Options;

struct Args {
    showed_help: bool,
    cmd_list_windows: bool,
    cmd_resize_window: bool,
    window_width: i32,
    window_height: i32,
    window_title: String
}

extern "system" fn enum_windows_proc(window_handle: HWND, lparam: LPARAM) -> BOOL {
    let window_handle_list;
    unsafe {
        window_handle_list = &mut*(lparam as *mut Vec<HWND>);
    }
    window_handle_list.push(window_handle);

    TRUE
}

fn get_window_handle_list() -> Vec<HWND> {
    let mut window_handle_list: Vec<HWND> = Vec::new();
    unsafe {
        EnumWindows(
            Some(enum_windows_proc),
            &mut window_handle_list as *mut Vec<HWND> as LPARAM);
    }

    window_handle_list
}

fn get_window_title(window_handle: HWND) -> String {
    let mut window_title: [WCHAR; 256] = [0; 256];
    let window_title_len;
    unsafe {
        window_title_len = GetWindowTextW(
            window_handle,
            window_title.as_mut_ptr(),
            window_title.len() as i32);
    }

    utf16_to_utf8(&window_title[0..window_title_len as usize])
}

fn find_window_from_title(window_title: &String) -> Option<HWND> {
    let window_handle;
    unsafe {
        window_handle = FindWindowW(
            ptr::null_mut(),
            utf8_to_utf16(window_title).as_mut_ptr());
    }
    if window_handle == ptr::null_mut() {
        None
    } else {
        Some(window_handle)
    }
}

fn set_window_size(window_handle: HWND, width: i32, height: i32) {
    unsafe {
        SetWindowPos(
            window_handle, ptr::null_mut(),
            0, 0, width, height, SWP_NOMOVE);
    }
}

fn utf16_to_utf8(source: &[WCHAR]) -> String {
    String::from_utf16(source).unwrap()
}

fn utf8_to_utf16(source: &String) -> Vec<WCHAR> {
    source.encode_utf16().chain(Some(0)).collect()
}

fn parse_args() -> Result<Args, &'static str> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("l", "list-windows", "List title windows.");
    opts.optflag("r", "resize-window", "Resize window.");
    opts.optopt("w", "width", "Window width.", "WIDTH");
    opts.optopt("h", "height", "Window height.", "HEIGHT");
    opts.optopt("t", "title", "Window title.", "TITLE");
    opts.optflag("", "help", "Show usage.");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("help") {
        print!("{}", opts.usage("Usage: wndsize [options]"));
        return Ok(Args {
            showed_help: true,
            cmd_list_windows: false,
            cmd_resize_window: false,
            window_width: 0,
            window_height: 0,
            window_title: String::from("")
        })
    }

    // 必須オプションのチェック
    if !matches.opt_present("l") && !matches.opt_present("r") {
        return Err("One of the following values are required: \
            --list-windows or --resize-window")
    }
    if matches.opt_present("r") {
        if !matches.opt_present("w") ||
            !matches.opt_present("h") ||
            !matches.opt_present("t")
        {
            return Err("These arguments are required: \
                --width, --height, --title.")
        }
    }

    Ok(Args {
        showed_help: false,
        cmd_list_windows: matches.opt_present("l"),
        cmd_resize_window: matches.opt_present("r"),
        window_width: matches.opt_get("w").unwrap().unwrap_or(0),
        window_height: matches.opt_get("h").unwrap().unwrap_or(0),
        window_title: matches.opt_str("t").unwrap_or(String::from(""))
    })

}

fn show_windows_list() {
    let window_handles: Vec<HWND> = get_window_handle_list();
    for window_handle in window_handles {
        let window_title = get_window_title(window_handle);
        if 0 < window_title.len() {
            println!("{}", window_title);
        }
    }
}

fn resize_window(window_title: &String, width: i32, height: i32) {
    println!("window title: {}", window_title);
    println!("width: {}", width);
    println!("height: {}", height);

    match find_window_from_title(window_title) {
        Some(window_handle) => set_window_size(window_handle, width, height),
        None => println!("{}", "Not found window.")
    }
}

fn main() {
    match parse_args() {
        Ok(args) => {
            if args.showed_help {
                // ヘルプを出したときは何もしない
            } else if args.cmd_list_windows {
                show_windows_list();
            } else if args.cmd_resize_window {
                resize_window(
                    &args.window_title, args.window_width, args.window_height);
            }
        },
        Err(message) => println!("{}", message)
    }
}
