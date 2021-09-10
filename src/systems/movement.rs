use crate::components::{CharState, MovementSpeed};
use bevy::prelude::*;
use petgraph::graphmap::UnGraphMap;
use std::collections::VecDeque;

/// System that moves the player and mobs. For anything that has
/// the `Moving` state, update their position based on their speed.
///
/// Uses the TileGraph struct to calculate paths if needed. If a
/// path exists, this system also validates that path before moving.
///
/// Because this system moves units it creates and maintains a data
/// structure to track which tiles are impassable for the pathing
/// function.
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut CharState, &MovementSpeed, &mut Transform)>,
    mut q_graph: Query<&mut TileGraph>,
) {
    let delta_seconds = time.delta_seconds();
    if let Ok(mut graph) = q_graph.single_mut() {
        for (mut state, speed, mut transform) in query.iter_mut() {
            match *state {
                // units that have entered moving state but no path
                // has been calculated yet
                CharState::Moving(destination, None) => {
                    if let Some(path) = graph.path(
                        (transform.translation.x, transform.translation.y),
                        (destination.0.x, destination.0.y),
                    ) {
                        // update the state to have the path we chose
                        *state = CharState::Moving(destination, Some(path));
                        graph.move_char(&mut transform, speed.0, delta_seconds, &mut state);
                    } else {
                        *state = CharState::Idle;
                    }
                }
                // units that already have a path
                CharState::Moving(destination, Some(_)) => {
                    // check if our current path is valid otherwise get a new one
                    if !graph.validate(&mut state) {
                        if let Some(path) = graph.path(
                            (transform.translation.x, transform.translation.y),
                            (destination.0.x, destination.0.y),
                        ) {
                            *state = CharState::Moving(destination, Some(path));
                        } else {
                            *state = CharState::Idle;
                        }
                    }

                    graph.move_char(&mut transform, speed.0, delta_seconds, &mut state);
                }
                // all other character states can be ignored.
                // potential optimazation could be to alter
                // the query so it only selects characters with
                // the correct state
                _ => (),
            }
        }
    }
}

pub struct TileGraph {
    graph: UnGraphMap<(i32, i32, i32), ()>,
    occupied_tiles: std::collections::HashSet<(i32, i32, i32)>,
    cell_size: f32,
    axial_to_world_transformation: bevy::math::Mat2,
    world_to_axial_transformation: bevy::math::Mat2,
}

impl TileGraph {
    pub fn new(map_size: i32, cell_size: f32) -> Self {
        let mut graph = UnGraphMap::new();
        let occupied_tiles = std::collections::HashSet::new();

        let tiles = (-map_size..=map_size)
            .map(|x| (-map_size..=map_size).map(move |y| (x, y)))
            .flatten()
            .map(|(x, y)| (-map_size..=map_size).map(move |z| (x, y, z)))
            .flatten()
            .filter(|coords| coords.0 + coords.1 + coords.2 == 0);

        for tile in tiles {
            graph.add_node(tile);
        }

        let nodes: Vec<(i32, i32, i32)> = graph.nodes().collect();
        // adds every edge two times but w/e it's fine
        for (x, y, z) in nodes {
            graph.add_edge((x, y, z), (x + 1, y - 1, z), ());
            graph.add_edge((x, y, z), (x + 1, y, z - 1), ());
            graph.add_edge((x, y, z), (x, y + 1, z - 1), ());
            graph.add_edge((x, y, z), (x - 1, y + 1, z), ());
            graph.add_edge((x, y, z), (x - 1, y, z + 1), ());
            graph.add_edge((x, y, z), (x, y - 1, z + 1), ());
        }

        let root3 = 3.0_f32.sqrt();
        let q_basis = Vec2::new(3.0 / 2.0, root3 / 2.0);
        let r_basis = Vec2::new(0.0, root3);
        let axial_to_world_transformation = bevy::math::Mat2::from_cols(q_basis, r_basis);
        let world_to_axial_transformation = axial_to_world_transformation.inverse();

        Self {
            graph,
            occupied_tiles,
            cell_size,
            axial_to_world_transformation,
            world_to_axial_transformation,
        }
    }

    /******************************************************************/
    /* Conversions between coordinate systems that are useful         */
    /******************************************************************/
    pub fn world_to_cube(&self, coords: (f32, f32)) -> (i32, i32, i32) {
        TileGraph::axial_to_cube(self.world_to_axial(coords))
    }
    pub fn world_to_axial(&self, coords: (f32, f32)) -> (i32, i32) {
        let vec = (self.world_to_axial_transformation * Vec2::new(coords.0, coords.1)
            / self.cell_size)
            .round();
        (vec.x as i32, vec.y as i32)
    }
    pub fn cube_to_world(&self, coords: (i32, i32, i32)) -> (f32, f32) {
        self.axial_to_world(TileGraph::cube_to_axial(coords))
    }
    pub fn cube_to_axial(coords: (i32, i32, i32)) -> (i32, i32) {
        (coords.0, coords.2)
    }
    pub fn axial_to_world(&self, coords: (i32, i32)) -> (f32, f32) {
        let vec = self.cell_size
            * self.axial_to_world_transformation
            * Vec2::new(coords.0 as f32, coords.1 as f32);
        (vec.x, vec.y)
    }
    pub fn axial_to_cube(coords: (i32, i32)) -> (i32, i32, i32) {
        (coords.0, -coords.0 - coords.1, coords.1)
    }

    pub fn path(&self, start: (f32, f32), end: (f32, f32)) -> Option<VecDeque<(i32, i32, i32)>> {
        let end = self.world_to_cube(end);
        if let Some((_, path)) = petgraph::algo::astar(
            &self.graph,
            self.world_to_cube(start),
            |target| target == end,
            |(_, tile, _)| {
                if self.occupied_tiles.contains(&tile) {
                    10000
                } else {
                    1
                }
            },
            |_| 0,
        ) {
            Some(path.into_iter().skip(1).collect())
        } else {
            // TODO: Pathing note --- if there is no valid path then a* fails
            // Right now we just return None when this happens but it would
            // be better if the path that resulted in the closest endpoint
            // to the desired location was produced.
            None
        }
    }

    pub fn validate(&self, char_state: &mut CharState) -> bool {
        if let CharState::Moving(_, Some(path)) = char_state {
            path.iter().all(|step| !self.occupied_tiles.contains(step))
        } else {
            false
        }
    }

    pub fn move_char(
        &mut self,
        char_transform: &mut Transform,
        move_speed: f32,
        delta_seconds: f32,
        char_state: &mut CharState,
    ) {
        if let CharState::Moving(_, Some(path)) = char_state {
            if let Some(path_step) = path.front() {
                let (x, y) = self.cube_to_world(*path_step);
                let direction = Vec3::new(x, y, 0.0) - char_transform.translation;
                let old_x = char_transform.translation.x;
                let old_y = char_transform.translation.y;

                char_transform.translation += move_speed * delta_seconds * direction.normalize();
                let new_x = char_transform.translation.x;
                let new_y = char_transform.translation.y;

                if self.world_to_cube((new_x, new_y)) == *path_step {
                    self.occupied_tiles
                        .remove(&self.world_to_cube((old_x, old_y)));
                    self.occupied_tiles.insert(*path_step);
                    path.pop_front();
                }
            } else {
                *char_state = CharState::Idle;
            }
        }
    }
}
