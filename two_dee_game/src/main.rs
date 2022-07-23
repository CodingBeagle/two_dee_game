use std::ffi::{CString};
use std::ptr;
use beagle_glfw::*;

fn main() {
    unsafe {
        if glfwInit() == 0 {
            panic!("Failed to initialize GLFW.");
        }

        // Create a window and an associated OpenGL context.
        let window_title = new_ffi_string("2D Game");
        let main_window = glfwCreateWindow(
            800,
            600,
            window_title.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut());

        // If main_window is NULL, window creation failed for some reason.
        if main_window.is_null() {
            panic!("Failed to create window!");
        }

        while glfwWindowShouldClose(main_window) == 0 {
            // Process events that are already in the event queue, then return immediately.
            // Processing events will cause the window and input callbacks associated with those events to be called.
            glfwPollEvents();
        }

        // Before terminating your application, you should terminate the GLFW library if it has been initialized
        // If you don't, global system settings changed by GLFW might not be restored properly.
        glfwTerminate();
    }
}

/*
    When communicating with unsafe bindings, I make use of the "CString" type: https://docs.rs/rustc-std-workspace-std/1.0.1/std/ffi/struct.CString.html
    This type represents an owned, C-compatible, nul-terminated string. 
    The important part for me right now being that it's nul-terminated, which many C APIs expect.
*/
fn new_ffi_string(str: &str) -> CString {
    let error_message = format!("Failed to generate CString from {}", str);
    CString::new(str).expect(&error_message)
}