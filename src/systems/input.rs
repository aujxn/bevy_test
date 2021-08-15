use crate::components::*;
use crate::events::*;
use bevy::prelude::*;

/// System that watches keyboard and mouse events and forwards them
/// to the player action system.
pub fn input_system(
    wnds: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_action: EventWriter<PlayerAction>,
    q_controls: Query<&UserControls>,
) {
    let wnd = wnds.get_primary().unwrap();
    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform, result is the `world coords` of the mouse
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let mouse_world_coordinates = Vec3::new(pos_wld.x, pos_wld.y, 1.0);

        if let Ok(controls) = q_controls.single() {
            for mouse_button in mouse_input.get_pressed() {
                if let Some(action) = controls.mouse.get(mouse_button) {
                    player_action.send(PlayerAction::new(*action, mouse_world_coordinates));
                }
            }

            for key in keyboard_input.get_pressed() {
                if let Some(action) = controls.keyboard.get(key) {
                    player_action.send(PlayerAction::new(*action, mouse_world_coordinates));
                }
            }
        }
    }
}
