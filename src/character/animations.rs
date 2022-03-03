use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{json};
use crate::animations::{AnimationMetadata, AnimationParams};
use crate::animated_sprite::AnimatedSprite;

/// This is used in stead of `AnimationParams`, as we have different data requirements, in the case
/// of a player character, compared to most other use cases. We want to have a default animation
/// set, for instance, that corresponds with the way the core game characters are animated, but
/// still have the possibility to declare custom animation sets, as well as have variation in size,
///
/// Refer to `crate::components::animation_player::AnimationParams` for detailed documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimationParams {
    #[serde(rename = "texture")]
    pub texture_id: String,
    #[serde(default = "json::default_scale")]
    pub scale: f32,
    #[serde(default, with = "json::vec2_def")]
    pub offset: Vec2,
    #[serde(default, with = "json::vec2_opt")]
    pub pivot: Option<Vec2>,
    #[serde(default, with = "json::uvec2_opt", skip_serializing_if = "Option::is_none")]
    pub frame_size: Option<UVec2>,
    #[serde(default, with = "json::color_opt", skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    #[serde(default)]
    pub animations: SpriteAnimations,
}

impl From<PlayerAnimationParams> for AnimationParams {
    fn from(other: PlayerAnimationParams) -> Self {
        AnimationParams {
            texture_id: other.texture_id,
            scale: other.scale,
            offset: other.offset,
            pivot: other.pivot,
            frame_size: other.frame_size,
            tint: other.tint,
            animations: other.animations.into(),
            should_autoplay: true,
            is_deactivated: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteAnimations {
    #[serde(default = "SpriteAnimations::default_idle_animation")]
    pub idle: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_move_animation")]
    pub moving: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_jump_animation")]
    pub jump: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_fall_animation")]
    pub fall: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_crouch_animation")]
    pub crouch: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_death_back_animation")]
    pub death_back: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_death_face_animation")]
    pub death_face: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_punch_animation")]
    pub punch: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_run_animation")]
    pub run: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_hurt_animation")]
    pub hurt: AnimationMetadata,
    #[serde(default = "SpriteAnimations::default_catching_animation")]
    pub catching: AnimationMetadata,
}


impl SpriteAnimations {
    pub fn default_idle_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::IDLE_ANIMATION_ID.to_string(),
            row: 0,
            frames: 7,
            fps: 12,
            is_looping: true,
        }
    }

    pub fn default_move_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::MOVE_ANIMATION_ID.to_string(),
            row: 1,
            frames: 7,
            fps: 10,
            is_looping: true,
        }
    }

    pub fn default_jump_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::JUMP_ANIMATION_ID.to_string(),
            row: 2,
            frames: 1,
            fps: 5,
            is_looping: false,
        }
    }

    pub fn default_fall_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::FALL_ANIMATION_ID.to_string(),
            row: 3,
            frames: 1,
            fps: 8,
            is_looping: true,
        }
    }

    pub fn default_crouch_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::CROUCH_ANIMATION_ID.to_string(),
            row: 4,
            frames: 1,
            fps: 8,
            is_looping: false,
        }
    }

    pub fn default_death_back_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::DEATH_BACK_ANIMATION_ID.to_string(),
            row: 5,
            frames: 7,
            fps: 10,
            is_looping: false,
        }
    }

    pub fn default_death_face_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::DEATH_FACE_ANIMATION_ID.to_string(),
            row: 6,
            frames: 7,
            fps: 10,
            is_looping: false,
        }
    }

    pub fn default_punch_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::PUNCH_ANIMATION_ID.to_string(),
            row: 9,
            frames: 8,
            fps: 10,
            is_looping: true,
        }
    }

    pub fn default_run_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::RUN_ANIMATION_ID.to_string(),
            row: 8,
            frames: 4,
            fps: 10,
            is_looping: true,
        }
    }

    pub fn default_hurt_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::HURT_ANIMATION_ID.to_string(),
            row: 7,
            frames: 2,
            fps: 3,
            is_looping: false,
        }
    }

    pub fn default_catching_animation() -> AnimationMetadata {
        AnimationMetadata {
            id: AnimatedSprite::CATCH_ANIMATION_ID.to_string(),
            row: 10,
            frames: 4,
            fps: 5,
            is_looping: true,
        }
    }
}

impl Default for SpriteAnimations {
    fn default() -> Self {
        SpriteAnimations {
            idle: Self::default_idle_animation(),
            moving: Self::default_move_animation(),
            jump: Self::default_jump_animation(),
            fall: Self::default_fall_animation(),
            crouch: Self::default_crouch_animation(),
            death_back: Self::default_death_back_animation(),
            death_face: Self::default_death_face_animation(),
            punch: Self::default_punch_animation(),
            run: Self::default_run_animation(),
            hurt: Self::default_hurt_animation(),
            catching: Self::default_catching_animation(),

        }
    }
}

fn find_metadata(vec: &Vec<AnimationMetadata>, animation_id: &str) -> AnimationMetadata {
    vec
        .iter()
        .find(|anim| anim.id == animation_id)
        .cloned()
        .unwrap()
}

impl From<Vec<AnimationMetadata>> for SpriteAnimations {
    fn from(vec: Vec<AnimationMetadata>) -> Self {
        SpriteAnimations {
            idle: find_metadata(&vec, AnimatedSprite::IDLE_ANIMATION_ID),
            moving: find_metadata(&vec, AnimatedSprite::MOVE_ANIMATION_ID),
            jump: find_metadata(&vec, AnimatedSprite::JUMP_ANIMATION_ID),
            fall: find_metadata(&vec, AnimatedSprite::FALL_ANIMATION_ID),
            crouch: find_metadata(&vec, AnimatedSprite::CROUCH_ANIMATION_ID),
            death_back: find_metadata(&vec, AnimatedSprite::DEATH_BACK_ANIMATION_ID),
            death_face: find_metadata(&vec, AnimatedSprite::DEATH_FACE_ANIMATION_ID),
            punch: find_metadata(&vec, AnimatedSprite::PUNCH_ANIMATION_ID),
            run: find_metadata(&vec, AnimatedSprite::RUN_ANIMATION_ID),
            hurt: find_metadata(&vec, AnimatedSprite::HURT_ANIMATION_ID),
            catching: find_metadata(&vec, AnimatedSprite::CATCH_ANIMATION_ID),
        }
    }
}

impl From<SpriteAnimations> for Vec<AnimationMetadata> {
    fn from(params: SpriteAnimations) -> Self {
        vec![
            params.idle,
            params.moving,
            params.jump,
            params.fall,
            params.crouch,
            params.death_back,
            params.death_face,
            params.punch,
            params.run,
            params.hurt,
            params.catching,
        ]
    }
}
