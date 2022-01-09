pub mod background;
pub mod camera;
pub mod character;
pub mod collision;
pub mod enemy;
pub mod player;
pub mod restart_panel;
pub mod score_label;

pub const Z_AXIS: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Load,
    Setup,
    Game,
    Restart,
}

pub struct CollisionEvent;
pub struct GameOverEvent;
pub struct RestartEvent;
