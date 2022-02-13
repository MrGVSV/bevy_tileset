use crate::helpers::WorldCamera;
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;

/// The position of the cursor at the time of this event
/// and whether it is pressed or not
#[allow(dead_code)]
pub struct ClickEvent(pub UVec2, pub bool);

#[allow(unused)]
pub fn on_click(
	query: Query<&Transform, With<WorldCamera>>,
	wnds: Res<Windows>,
	buttons: Res<Input<MouseButton>>,
	mut event_writer: EventWriter<ClickEvent>,
) {
	let just_pressed = buttons.just_pressed(MouseButton::Left);
	let just_released = buttons.just_released(MouseButton::Left);

	if !just_pressed && !just_released {
		return;
	}

	let wnd = wnds.get_primary().unwrap();
	if let Some(pos) = wnd.cursor_position() {
		let cam = query.single();
		let mut pos = window_to_world(&pos, wnd, cam).xy();
		pos /= Vec2::new(32.0, 32.0);

		let x = pos.x.floor() as u32;
		let y = pos.y.floor() as u32;

		let pos = UVec2::new(x, y);
		event_writer.send(ClickEvent(pos, just_pressed));
	}
}

#[allow(dead_code)]
pub fn get_mouse_pos(
	query: &Query<&Transform, With<WorldCamera>>,
	wnds: &Windows,
) -> Option<UVec2> {
	let wnd = wnds.get_primary().unwrap();
	if let Some(pos) = wnd.cursor_position() {
		let cam = query.single();
		let mut pos = window_to_world(&pos, wnd, cam).xy();
		pos /= Vec2::new(32.0, 32.0);

		let x = pos.x.floor() as u32;
		let y = pos.y.floor() as u32;

		let pos = UVec2::new(x, y);

		return Some(pos);
	}

	None
}

pub fn window_to_world(position: &Vec2, window: &Window, camera: &Transform) -> Vec4 {
	// get the size of the window
	let size = Vec2::new(window.width() as f32, window.height() as f32);

	// the default orthographic projection is in pixels from the center;
	// just undo the translation
	let p = *position - size / 2.0;

	// apply the camera transform
	camera.compute_matrix() * p.extend(0.0).extend(1.0)
}
