use anchor_lang::prelude::*;


#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient collateral for requested stablecoin amount")]
    InsufficientCollateral,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}

#[error_code]
pub enum LiquidationError {
    #[msg("Sufficient collateral, no need for liquidation")]
    SufficientCollateral,
    #[msg("Insufficient collateral for requested stablecoin amount")]
    InsufficientCollateral,
    #[msg("Liquidation amount must be greater than 0 and less than or equal to the debt amount")]
    LiquidationAmountInvalid,
}

#[error_code]
pub enum PositionDebtSettlementError {
    #[msg("Debt amount must be greater than 0")]
    DebtAmountZero,
    #[msg("Debt amount must be greater than 0 and less than or equal to the debt amount")]
    DebtAmountInvalid,
}