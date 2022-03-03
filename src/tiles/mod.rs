use macroquad::color::Color;
use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};
use crate::tiles::animations::TileAnimationParams;

pub mod animations;

#[derive(Clone, Debug)]
pub struct Tile {
    pub(crate) pos: Vec2,
    pub(crate) color: Color,
    pub(crate) animation: usize,
}


impl Tile {
    pub const IDLE_ANIMATION_ID: &'static str = "idle";
    pub const ACTIVE_ANIMATION_ID: &'static str = "move";

    pub(crate) fn default() -> Tile {
        Tile {
            pos: Default::default(),
            color: Default::default(),
            animation: 0,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileParam {
    /// This is the id of the player character. This should be unique, or it will either overwrite
    /// or be overwritten, depending on load order, if not.
    pub id: String,
    /// This is the name of the player character, as shown in character selection
    pub name: String,
    /// This is the description for the player character, as shown in character selection
    #[serde(default)]
    pub description: String,
    /// This holds the animation and sprite parameters for the player character. This is flattened,
    /// meaning that, in JSON, you will declare the members of this struct directly in the
    /// `PlayerCharacterParams` entry.
    #[serde(flatten)]
    pub animation: TileAnimationParams,
}