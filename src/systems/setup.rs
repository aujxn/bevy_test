use crate::components::*;
use crate::entities::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn().insert(UserControls::new());
    commands.spawn_bundle(PlayerBundle::new(&asset_server, &mut materials));
    commands.spawn_bundle(MobBundle::new(&asset_server, &mut materials));
    commands.spawn_bundle(DashBundle::new());

    let map_size = 30;
    let cell_size = 35.0;
    let cell = shapes::Rectangle {
        width: cell_size,
        height: cell_size,
        origin: shapes::RectangleOrigin::Center,
    };
    let drawmode = DrawMode::Outlined {
        fill_options: FillOptions::default(),
        outline_options: StrokeOptions::default().with_line_width(1.0),
    };

    let start_x = -1.0 * map_size as f32 * cell_size;
    let start_y = -1.0 * start_x;
    for x in 0..2 * map_size {
        for y in 0..2 * map_size {
            let color = ShapeColors {
                main: Color::rgba(0.0, 0.0, 0.0, 0.0),
                outline: Color::BLACK,
            };
            let x = start_x + (x as f32 * cell_size);
            let y = start_y - (y as f32 * cell_size);

            // make an obstacle to path around
            if (Vec2::new(-150.0, -150.0) - Vec2::new(x, y)).length() > 80.0 {
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &cell,
                        color,
                        drawmode,
                        Transform::from_xyz(x, y, 0.0),
                    ))
                    .insert(Cell);
            }
        }
    }

    let graph = crate::systems::movement::TileGraph::new(map_size, cell_size);
    commands.spawn().insert(graph);
}
