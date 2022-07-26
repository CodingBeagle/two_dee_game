use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use glad::{gl};
use beagle_glfw::*;

fn main() {
    unsafe {
        if glfwInit() == 0 {
            panic!("Failed to initialize GLFW.");
        }

        // Set window & context creation hints

        // I don't want a resizable window
        glfwWindowHint(GLFW_RESIZABLE as i32, GLFW_FALSE as i32);

        // We want OpenGL 3.3, with no forward compatibility, and the core profile.
        // The context should also be DEBUG context for now (to be able to get more error details when something in the context failsd)
        // Core Profile = Deprecated and removed features will NOT be available in the OpenGL context (i.e, no forward compatibility)
        glfwWindowHint(GLFW_OPENGL_API as i32, GLFW_TRUE as i32); // We want The OpenGL client API (as opposed to OPENGL ES)
        glfwWindowHint(GLFW_DOUBLEBUFFER as i32, GLFW_TRUE as i32); // We want a double-buffered framebuffer
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR as i32, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR as i32, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE as i32 , GLFW_OPENGL_CORE_PROFILE as i32);
        glfwWindowHint(GLFW_OPENGL_DEBUG_CONTEXT as i32, GLFW_TRUE as i32);

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
            panic!("Failed to create window: {}", get_latest_glfw_error_description());
        }

        // Make the OpenGL context of our window current
        // This is required before we can use the context, and before we can do things suchas loading extensions
        glfwMakeContextCurrent(main_window);

        // Load OpenGL extensions using GLAD
        gl::load(|e| -> *const std::ffi::c_void 
            {
                let f = CString::new(e).expect("Failed to create CString");
                glfwGetProcAddress(f.as_ptr()).expect("Failed to get OpenGL extension function pointer") as *const std::os::raw::c_void
            });

        println!("**** OpenGL Context Info ****");
        println!("OpenGL Version: {}", raw_string_to_borrowed_string(gl::GetString(gl::VERSION) as _));
        println!("*****************************");

        while glfwWindowShouldClose(main_window) == 0 {
            // Process events that are already in the event queue, then return immediately.
            // Processing events will cause the window and input callbacks associated with those events to be called.
            glfwSwapBuffers(main_window);
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

// When working with strings, I don't have to worry about conversion between *const i8 and *const u8. So long as I don't care about 
// Arithmetic operations on each byte.
// Reason being: i8 and u8 are both of the same size, so no mis-alignment will occur when doing pointer arithmetic.
// Additionally, whether something is signed or unsigned only matters when interpreting the data as NUMBERS, but since this is strings
// we don't care about that.
fn raw_string_to_borrowed_string<'a>(raw_string: *const c_char) -> &'a str {
    unsafe {
        CStr::from_ptr(raw_string).to_str().expect("Failed to create str!")
    }
}

fn get_latest_glfw_error_description() -> String {
    unsafe {
        let mut error_description_raw: *const i8 = ptr::null_mut();
        glfwGetError(&mut error_description_raw);
        let error_description = CString::from_raw(error_description_raw as *mut i8);
        error_description.into_string().expect("Failed to convert GLFW Error description into String")
    }
}