#![feature(hash_drain_filter)]
#![feature(variant_count)]
#![allow(dead_code)]

pub(crate) mod bank;
pub(crate) mod board;
pub(crate) mod building;
pub(crate) mod development_cards;
pub(crate) mod game;
pub(crate) mod player;
pub(crate) mod resources;
pub(crate) mod trade;

pub use game::Game;
pub use player::Player;

pub use development_cards::DevelopmentCard::*;
pub use resources::ResourceKind::*;
