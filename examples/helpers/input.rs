use crate::helpers::WorldCamera;
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection};

pub struct ClickEvent(pub UVec2);

pub fn on_click(
	query: Query<&Transform, With<WorldCamera>>,
	wnds: Res<Windows>,
	buttons: Res<Input<MouseButton>>,
	mut event_writer: EventWriter<ClickEvent>,
) {
	if !buttons.just_pressed(MouseButton::Left) {
		return;
	}

	let wnd = wnds.get_primary().unwrap();
	if let Some(pos) = wnd.cursor_position() {
		let cam = query.single().unwrap();
		let mut pos = window_to_world(&pos, wnd, cam).xy();
		pos /= Vec2::new(32.0, 32.0);

		let x = pos.x.floor() as u32;
		let y = pos.y.floor() as u32;

		let pos = UVec2::new(x, y);
		event_writer.send(ClickEvent(pos));
	}
}

pub fn click_to_coord(
	query: &Query<&Transform, With<Camera>>,
	wnds: &Windows,
	buttons: &Input<MouseButton>,
) -> Option<UVec2> {
	if !buttons.just_pressed(MouseButton::Left) {
		return None;
	}

	let wnd = wnds.get_primary().unwrap();
	if let Some(pos) = wnd.cursor_position() {
		let cam = query.single().unwrap();
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
