use beagle_glfw::{self, glfwInit};

fn main() {
    println!("Hello, world!");

    unsafe {
        let result = glfwInit();

        print!("{}", result);
    }
}