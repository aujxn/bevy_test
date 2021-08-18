use crate::events::{Action, PlayerAction};
use bevy::prelude::*;
use petgraph::visit::EdgeRef;
use petgraph::{graph::NodeIndex, graphmap::UnGraphMap};
use rustc_hash::FxHashMap;

// Label components
pub struct Player;
pub struct Mob;
pub struct Projectile;
pub struct MainCamera;
pub struct Dash;
pub struct Cell;
pub struct Impassable;

// Player and mob components
pub struct Health(pub i64);
pub struct Energy(pub i64);
pub struct Experience(pub i64);
pub struct MovementSpeed(pub f32);

// Ability components
#[derive(PartialEq, Clone, Copy)]
pub enum CastAbility {
    Dash,
    Shoot,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ChannelAbility {
    Lazer,
}

pub struct Cooldown(pub f32);
pub struct Charges(pub i64);
pub struct MaxCharges(pub i64);
pub struct CastTime(pub f32);
pub struct CastTimer(pub Timer);
pub struct CooldownTimer(pub Timer);

#[derive(PartialEq, Clone, Copy)]
pub struct MouseCoords(pub Vec3);

#[derive(PartialEq)]
pub enum CharState {
    Casting((CastAbility, MouseCoords)),
    Moving(MouseCoords),
    Channeling((ChannelAbility, MouseCoords)),
    Idle,
}

impl CharState {
    pub fn can_cast(&self) -> bool {
        match self {
            Self::Casting(_) => false,
            Self::Moving(_) => true,
            Self::Channeling(_) => true,
            Self::Idle => true,
        }
    }

    pub fn can_move(&self) -> bool {
        match self {
            Self::Casting(_) => false,
            Self::Moving(_) => true,
            Self::Channeling(_) => false,
            Self::Idle => true,
        }
    }
}

impl From<PlayerAction> for CharState {
    fn from(action: PlayerAction) -> Self {
        match action.action {
            Action::Move => CharState::Moving(MouseCoords(action.mouse_coords)),
            Action::Cast(ability) => {
                CharState::Casting((ability, MouseCoords(action.mouse_coords)))
            }
            Action::Channel(ability) => {
                CharState::Channeling((ability, MouseCoords(action.mouse_coords)))
            }
        }
    }
}

// Map keyboard and mouse into player actions
pub struct UserControls {
    pub mouse: FxHashMap<MouseButton, Action>,
    pub keyboard: FxHashMap<KeyCode, Action>,
}

impl UserControls {
    pub fn new() -> Self {
        let mut mouse = FxHashMap::default();
        let mut keyboard = FxHashMap::default();

        mouse.insert(MouseButton::Right, Action::Move);
        keyboard.insert(KeyCode::Q, Action::Cast(CastAbility::Dash));
        keyboard.insert(KeyCode::W, Action::Cast(CastAbility::Shoot));
        keyboard.insert(KeyCode::E, Action::Channel(ChannelAbility::Lazer));

        Self { mouse, keyboard }
    }
}

pub struct TileGraph {
    graph: UnGraphMap<(i32, i32), ()>,
    map_size: i32,
    cell_size: f32,
}

impl TileGraph {
    pub fn new(map_size: i32, cell_size: f32) -> Self {
        let mut graph = UnGraphMap::new();

        for x in -map_size..=map_size {
            for y in -map_size..=map_size {
                graph.add_node((x, y));
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
            map_size,
            cell_size,
        }
    }

    fn get_index(&self, x: f32, y: f32) -> (i32, i32) {
        let x = (x / self.cell_size).round() as i32;
        let y = (y / self.cell_size).round() as i32;
        (x, y)
    }
    pub fn path(&self, start: (f32, f32), end: (f32, f32), blocked: &Vec<Vec3>) -> Option<Vec3> {
        let blocked: std::collections::HashSet<(i32, i32)> = blocked
            .iter()
            .map(|translation| self.get_index(translation.x, translation.y))
            .collect();

        let end = self.get_index(end.0, end.1);
        if let Some((_, path)) = petgraph::algo::astar(
            &self.graph,
            self.get_index(start.0, start.1),
            |target| target == end,
            |(_, (e0, e1), _)| {
                if blocked.contains(&(e0, e1)) {
                    i32::MAX
                } else {
                    (e0 - end.0).pow(2) + (e1 - end.1).pow(2)
                }
            },
            |_| 0,
        ) {
            println!("{:?}", path);
            if path.len() > 1 {
                return Some(Vec3::new(
                    path[1].0 as f32 * self.cell_size,
                    path[1].1 as f32 * self.cell_size,
                    1.0,
                ));
            }
        }
        None
    }
}
