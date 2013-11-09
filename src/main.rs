mod graphics;

#[start]
fn start(argc: int, argv: **u8) -> int {
	std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
	graphics::initialize();
	let window = graphics::create_window();
	graphics::deinitialize();
}
