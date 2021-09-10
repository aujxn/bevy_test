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

    let map_size = 10;
    let cell_size = 25.0;
    let cell = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(cell_size),
        ..shapes::RegularPolygon::default()
    };
    let drawmode = DrawMode::Outlined {
        fill_options: FillOptions::default(),
        outline_options: StrokeOptions::default().with_line_width(1.0),
    };

    let root3 = 3.0_f32.sqrt();
    let q_basis = Vec2::new(3.0 / 2.0, root3 / 2.0);
    let r_basis = Vec2::new(0.0, root3);
    let basis_matrix = bevy::math::Mat2::from_cols(q_basis, r_basis);

    // easiest way to get a full hexagon map
    let tiles = (-map_size..=map_size)
        .map(|x| (-map_size..=map_size).map(move |y| (x, y)))
        .flatten()
        .map(|(x, y)| (-map_size..=map_size).map(move |z| (x, y, z)))
        .flatten()
        .filter(|coords| coords.0 + coords.1 + coords.2 == 0);

    for tile in tiles {
        let color = ShapeColors {
            main: Color::rgba(0.0, 0.0, 0.0, 0.0),
            outline: Color::BLACK,
        };

        let (q, r) = cube_to_axial(tile);
        let position = cell_size * basis_matrix * Vec2::new(q as f32, r as f32);

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &cell,
                color,
                drawmode,
                Transform::from_xyz(position.x, position.y, 0.0),
            ))
            .insert(Cell);
    }

    let graph = crate::systems::movement::TileGraph::new(map_size, cell_size);
    commands.spawn().insert(graph);
}

fn cube_to_axial(coords: (i32, i32, i32)) -> (i32, i32) {
    (coords.0, coords.2)
}

fn axial_to_cube(coords: (i32, i32)) -> (i32, i32, i32) {
    (coords.0, -coords.0 - coords.1, coords.1)
}
