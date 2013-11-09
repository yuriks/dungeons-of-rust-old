extern mod glfw;

pub fn initialize() {
	glfw::init()
		.expect("Failed to initialize GLFW.");
}

pub fn deinitialize() {
	glfw::terminate();
}

pub fn create_window() -> glfw::Window {
	let window = glfw::Window::create(640, 480, "Dungeons of Rust", glfw::Windowed)
		.expect("Failed to create window.");
	window.make_context_current();
	return window;
}
