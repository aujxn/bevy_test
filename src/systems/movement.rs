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
    graph: UnGraphMap<(i32, i32), ()>,
    occupied_tiles: std::collections::HashSet<(i32, i32)>,
    cell_size: f32,
}

impl TileGraph {
    pub fn new(map_size: i32, cell_size: f32) -> Self {
        let mut graph = UnGraphMap::new();
        let occupied_tiles = std::collections::HashSet::new();

        for x in -map_size..=map_size {
            for y in -map_size..=map_size {
                if (Vec2::new(-150.0, -150.0)
                    - Vec2::new(x as f32 * cell_size, y as f32 * cell_size))
                .length()
                    > 80.0
                {
                    graph.add_node((x, y));
                }
            }
        }

        let nodes: Vec<(i32, i32)> = graph.nodes().collect();
        for (x, y) in nodes {
            if x != map_size {
                graph.add_edge((x, y), (x + 1, y), ());
            }
            if y != -map_size {
                graph.add_edge((x, y), (x, y + 1), ());
            }
            if x != map_size && y != -map_size {
                graph.add_edge((x, y), (x + 1, y + 1), ());
            }
            if x != -map_size && y != -map_size {
                graph.add_edge((x, y), (x - 1, y - 1), ());
            }
        }
        Self {
            graph,
            occupied_tiles,
            cell_size,
        }
    }

    pub fn get_index(&self, x: f32, y: f32) -> (i32, i32) {
        let x = (x / self.cell_size).round() as i32;
        let y = (y / self.cell_size).round() as i32;
        (x, y)
    }

    pub fn get_coords(&self, x: i32, y: i32) -> Vec3 {
        Vec3::new(x as f32 * self.cell_size, y as f32 * self.cell_size, 1.0)
    }

    pub fn path(&self, start: (f32, f32), end: (f32, f32)) -> Option<VecDeque<(i32, i32)>> {
        let end = self.get_index(end.0, end.1);
        if let Some((_, path)) = petgraph::algo::astar(
            &self.graph,
            self.get_index(start.0, start.1),
            |target| target == end,
            |(_, (e0, e1), _)| {
                if self.occupied_tiles.contains(&(e0, e1)) {
                    10000
                } else {
                    (e0 - end.0).pow(2) + (e1 - end.1).pow(2)
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
                let direction =
                    self.get_coords(path_step.0, path_step.1) - char_transform.translation;
                let old_x = char_transform.translation.x;
                let old_y = char_transform.translation.y;

                char_transform.translation += move_speed * delta_seconds * direction.normalize();
                let new_x = char_transform.translation.x;
                let new_y = char_transform.translation.y;

                if self.get_index(new_x, new_y) == *path_step {
                    self.occupied_tiles.remove(&self.get_index(old_x, old_y));
                    self.occupied_tiles.insert(*path_step);
                    path.pop_front();
                }
            } else {
                *char_state = CharState::Idle;
            }
        }
    }
}
