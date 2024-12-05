extern crate x11;

use std::process::Command;
use std::ptr;
use std::ffi::CString;
use x11::xlib::*;
use std::os::raw::c_int;
use std::time::{Duration, Instant};
use colorized::{Color, Colors};

fn main() {
    let display = unsafe { XOpenDisplay(ptr::null()) };
    let screen = unsafe { XDefaultScreen(display) };
    let root_window = unsafe { XRootWindow(display, screen) };

    let ww = unsafe { XDisplayWidth(display, screen) } as u32;
    let hh = unsafe { XDisplayHeight(display, screen) } as u32;

    println!("your resolution: {} x {}", ww, hh);

    let window = unsafe {
        XCreateSimpleWindow(
            display,
            root_window,
            0,
            0,
            ww,
            hh,
            0,
            0,
            0,
        )
    };

    let wm_type = unsafe { XInternAtom(display, CString::new("_NET_WM_WINDOW_TYPE").unwrap().as_ptr(), 0) };
    let wm_opacity = unsafe { XInternAtom(display, CString::new("_NET_WM_WINDOW_OPACITY").unwrap().as_ptr(), 0) };

    let mut value: c_int = 1;
    unsafe {
        XChangeProperty(
            display,
            window,
            wm_type,
            XA_ATOM,
            32,
            PropModeReplace,
            &mut value as *mut _ as *mut _,
            1,
        );

        let opacity: u32 = 0x80FFFFFF;
        XChangeProperty(
            display,
            window,
            wm_opacity,
            XA_CARDINAL,
            32,
            PropModeReplace,
            &opacity as *const _ as *mut _,
            1,
        );

        XSelectInput(display, window, ExposureMask | StructureNotifyMask | ButtonPressMask | ButtonReleaseMask | PointerMotionMask);
        XMapWindow(display, window);
        XFlush(display);
    }

    let mut is_drawing = false;
    let mut start_x = 0;
    let mut start_y = 0;
    let mut current_x = 0;
    let mut current_y = 0;

    let mut isFin = false;
    let mut rect_width = 0;
    let mut rect_heigt = 0;
    while !isFin {
        unsafe {
            let mut event: XEvent = std::mem::zeroed();
            while XPending(display) > 0 {
                XNextEvent(display, &mut event);

                match event.get_type() {
                    ButtonPress => {
                        let button_event: &XButtonEvent = &event.button;
                        if button_event.button == 1 {
                            is_drawing = true;
                            start_x = button_event.x;
                            start_y = button_event.y;
                            current_x = start_x;
                            current_y = start_y;
                        }
                    }
                    MotionNotify => {
                        if is_drawing {
                            let motion_event: &XMotionEvent = &event.motion;
                            current_x = motion_event.x;
                            current_y = motion_event.y;

                            // 重绘窗口
                            XClearWindow(display, window);
                            let gc = XCreateGC(display, window, 0, ptr::null_mut());
                            XSetForeground(display, gc, 0xFFFF00);
                            XFillRectangle(display, window, gc, (start_x).try_into().unwrap(), (start_y).try_into().unwrap(), (current_x - start_x) as u32, (current_y - start_y) as u32);
                            XFreeGC(display, gc);
                            XFlush(display);
                        }
                    }
                    ButtonRelease => {
                        let button_event: &XButtonEvent = &event.button;
                        if is_drawing && button_event.button == 1 {
                            is_drawing = false;
                            rect_width = current_x - start_x;
                            rect_heigt = current_y - start_y;
                            println!("矩形坐标: ({}x{}), 宽: {}, 高: {}", start_x, start_y, rect_width, rect_heigt);
                            isFin = true;
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        unsafe {
            XFlush(display);
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    unsafe {
        XDestroyWindow(display, window);
        XCloseDisplay(display);
    }
    let rect = format!("rect: ({}x{}), width: {}, height: {}", start_x, start_y, rect_width, rect_heigt);
    println!("{}", rect.color(Colors::BrightGreenFg));
    let ffmpeg1 = format!("ffmpeg -video_size {}x{} -framerate 25 -f x11grab -i :0.0+{},{} -vf format=yuv420p abc.mp4", rect_width, rect_heigt, start_x, start_y);
    println!("{}", ffmpeg1.color(Colors::BrightBlueBg).color(Colors::BlackFg));
    println!("{}", "pactl list sources short".color(Colors::BrightBlueFg));
    let ffmpeg2 = format!("ffmpeg -video_size {}x{} -framerate 25 -f x11grab -i :0.0+{},{} -f pulse -i bluez_output.98_DD_60_C6_9E_4F.1.monitor -vf format=yuv420p abc.mp4", rect_width, rect_heigt, start_x, start_y);
    println!("{}", ffmpeg2.color(Colors::BrightBlueBg).color(Colors::BlackFg));
}
