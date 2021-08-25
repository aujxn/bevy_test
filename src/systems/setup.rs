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
    let mob_bundle = MobBundle::new(&asset_server, &mut materials, &mut commands);
    commands.spawn_bundle(mob_bundle);
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

    ui(commands, materials, asset_server);
}

/// this ui is scary right now
fn ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,

    asset_server: Res<AssetServer>,
) {
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::FlexEnd,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Bottom section
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Left side
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                padding: Rect::all(Val::Px(3.0)),
                                align_items: AlignItems::FlexStart,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // heath area
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                                        align_items: AlignItems::FlexStart,
                                        flex_direction: FlexDirection::Row,
                                        padding: Rect::all(Val::Px(2.0)),
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    // Health bar border
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(80.0),
                                                    Val::Percent(100.0),
                                                ),
                                                align_items: AlignItems::FlexStart,
                                                flex_direction: FlexDirection::Row,
                                                padding: Rect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            material: materials.add(Color::BLACK.into()),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            // Health bar red fill
                                            parent.spawn_bundle(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Percent(100.0),
                                                    ),
                                                    align_items: AlignItems::FlexStart,
                                                    flex_direction: FlexDirection::Row,
                                                    ..Default::default()
                                                },
                                                material: materials.add(Color::RED.into()),
                                                ..Default::default()
                                            });
                                        });
                                    // Health number area
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(20.0),
                                                    Val::Percent(100.0),
                                                ),
                                                padding: Rect::all(Val::Px(3.0)),
                                                align_items: AlignItems::FlexStart,
                                                flex_direction: FlexDirection::RowReverse,
                                                ..Default::default()
                                            },
                                            material: materials
                                                .add(Color::rgb(0.52, 0.52, 0.52).into()),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            // hp text
                                            parent.spawn_bundle(TextBundle {
                                                style: Style {
                                                    margin: Rect::all(Val::Px(5.0)),
                                                    ..Default::default()
                                                },
                                                text: Text::with_section(
                                                    "100 / 100",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 30.0,
                                                        color: Color::WHITE,
                                                    },
                                                    Default::default(),
                                                ),
                                                ..Default::default()
                                            });
                                        });
                                });
                            // energy area
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                                        align_items: AlignItems::FlexStart,
                                        flex_direction: FlexDirection::Row,
                                        padding: Rect::all(Val::Px(2.0)),
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    // Energy bar border
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(80.0),
                                                    Val::Percent(100.0),
                                                ),
                                                align_items: AlignItems::FlexStart,
                                                flex_direction: FlexDirection::Row,
                                                padding: Rect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            material: materials.add(Color::BLACK.into()),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            // Energy bar gold fill
                                            parent.spawn_bundle(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Percent(100.0),
                                                    ),
                                                    align_items: AlignItems::FlexStart,
                                                    flex_direction: FlexDirection::Row,
                                                    ..Default::default()
                                                },
                                                material: materials.add(Color::GOLD.into()),
                                                ..Default::default()
                                            });
                                        });
                                    // Energy number area
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(20.0),
                                                    Val::Percent(100.0),
                                                ),
                                                padding: Rect::all(Val::Px(3.0)),
                                                align_items: AlignItems::FlexStart,
                                                flex_direction: FlexDirection::RowReverse,
                                                ..Default::default()
                                            },
                                            material: materials
                                                .add(Color::rgb(0.52, 0.52, 0.52).into()),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            // energy text
                                            parent.spawn_bundle(TextBundle {
                                                style: Style {
                                                    margin: Rect::all(Val::Px(5.0)),
                                                    ..Default::default()
                                                },
                                                text: Text::with_section(
                                                    "100 / 100",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 30.0,
                                                        color: Color::WHITE,
                                                    },
                                                    Default::default(),
                                                ),
                                                ..Default::default()
                                            });
                                        });
                                });
                        });
                    // Right side
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                padding: Rect::all(Val::Px(3.0)),
                                align_items: AlignItems::FlexStart,
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // abilities
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(75.0), Val::Percent(100.0)),
                                        padding: Rect::all(Val::Px(3.0)),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    for _ in 0..5 {
                                        let val = 0.55;
                                        parent.spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(25.0),
                                                    Val::Percent(100.0),
                                                ),
                                                padding: Rect::all(Val::Px(3.0)),
                                                align_items: AlignItems::FlexStart,
                                                flex_direction: FlexDirection::Row,
                                                aspect_ratio: Some(1.0),
                                                ..Default::default()
                                            },
                                            material: materials
                                                .add(Color::rgb(val, val, val).into()),
                                            ..Default::default()
                                        });
                                    }
                                });

                            // batteries
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                                        padding: Rect::all(Val::Px(3.0)),
                                        align_items: AlignItems::FlexStart,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    for _ in 0..3 {
                                        let val = 0.55;
                                        parent.spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(25.0),
                                                    Val::Percent(100.0),
                                                ),
                                                padding: Rect::all(Val::Px(3.0)),
                                                align_items: AlignItems::FlexStart,
                                                flex_direction: FlexDirection::Row,
                                                ..Default::default()
                                            },
                                            material: materials
                                                .add(Color::rgb(val, val, val).into()),
                                            ..Default::default()
                                        });
                                    }
                                });
                        });
                });
        });
}
