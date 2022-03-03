use macroquad::prelude::*;
use crate::animations::AnimationPlayer;

#[derive(PartialEq)]
pub enum AnimatedSpriteState {
    Idle,
    Walking,
    Jumping,
    Ducking,
    Running,
    Catching,
    Hurting,
    Died,
    Throwing,
    Passing,
}

pub struct AnimatedSprite {
    pub(crate) id: u8,
    pub(crate) animation_player: AnimationPlayer,
    pub(crate) state: AnimatedSpriteState,

}

impl AnimatedSprite {
    pub(crate) fn set_animation(&mut self) {
        let state = match self.state {
            AnimatedSpriteState::Idle => AnimatedSprite::IDLE_ANIMATION_ID,
            AnimatedSpriteState::Walking => AnimatedSprite::MOVE_ANIMATION_ID,
            AnimatedSpriteState::Jumping => AnimatedSprite::JUMP_ANIMATION_ID,
            AnimatedSpriteState::Ducking => AnimatedSprite::CROUCH_ANIMATION_ID,
            AnimatedSpriteState::Running => AnimatedSprite::RUN_ANIMATION_ID,
            AnimatedSpriteState::Catching => AnimatedSprite::CATCH_ANIMATION_ID,
            AnimatedSpriteState::Hurting => AnimatedSprite::HURT_ANIMATION_ID,
            AnimatedSpriteState::Died => AnimatedSprite::DEATH_BACK_ANIMATION_ID,
            AnimatedSpriteState::Throwing => AnimatedSprite::CATCH_ANIMATION_ID,
            AnimatedSpriteState::Passing => AnimatedSprite::CATCH_ANIMATION_ID,
        };
        self.animation_player.set_animation(state);
        self.animation_player.update();
    }

}

impl AnimatedSprite {
    pub const IDLE_ANIMATION_ID: &'static str = "idle";
    pub const MOVE_ANIMATION_ID: &'static str = "move";
    pub const JUMP_ANIMATION_ID: &'static str = "jump";
    pub const FALL_ANIMATION_ID: &'static str = "fall";
    pub const CROUCH_ANIMATION_ID: &'static str = "crouch";
    pub const DEATH_BACK_ANIMATION_ID: &'static str = "death_back";
    pub const DEATH_FACE_ANIMATION_ID: &'static str = "death_face";
    pub const PUNCH_ANIMATION_ID: &'static str = "punch";
    pub const RUN_ANIMATION_ID: &'static str = "run";
    pub const HURT_ANIMATION_ID: &'static str = "hurt";
    pub const CATCH_ANIMATION_ID: &'static str = "catching";
    pub const CATCH_GRACE_TIME: f64 = 5.;

    pub fn new(id:u8, animation_player: AnimationPlayer) -> AnimatedSprite {
        AnimatedSprite {
            id,
            animation_player,
            state: AnimatedSpriteState::Idle,
        }
    }
}