use std::collections::HashMap;
use std::any::Any;

extern crate piston_window;

use piston_window::*;

fn main() {
	let mut window: PistonWindow = WindowSettings::new("Hello Calculator", [600, 600]).exit_on_esc(true).build().unwrap();
	while let Some(event) = window.next() {
		if let Some(_args) = event.render_args() {
			window.draw_2d(&(event), |context, graphics, device| {
								clear([0.1, 0.1, 0.1, 1.0], graphics);
							});
		}
	}
}

