use crate::components::*;
use crate::events::PlayerAction;
use bevy::prelude::*;

pub fn input_system(
    wnds: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_action: EventWriter<(PlayerAction, Vec3)>,
) {
    // calculate the world coords of the cursor
    let wnd = wnds.get_primary().unwrap();
    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let mouse_world_coordinates = Vec3::new(pos_wld.x, pos_wld.y, 1.0);

        if mouse_input.pressed(MouseButton::Right) {
            player_action.send((PlayerAction::Move, mouse_world_coordinates));
        }

        if keyboard_input.pressed(KeyCode::Q) {
            player_action.send((PlayerAction::Dash, mouse_world_coordinates));
        }

        if keyboard_input.pressed(KeyCode::W) {
            player_action.send((PlayerAction::Bow, mouse_world_coordinates));
        }
    }
}
