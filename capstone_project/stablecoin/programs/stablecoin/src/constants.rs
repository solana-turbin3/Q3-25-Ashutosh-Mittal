use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const COLLATERAL_RATIO: u16 = 15000; 

#[constant]
pub const BASIS_POINTS: u16 = 10000;

#[constant]
pub const LIQUIDATION_THRESHOLD: u16 = 12500;

#[constant]
pub const PENALTY_REWARD_PERCENTAGE: u16 = 5;