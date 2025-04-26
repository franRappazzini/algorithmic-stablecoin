use anchor_lang::prelude::*;

#[event]
pub struct UpdateConfigEvent {
    pub message: String,
}
