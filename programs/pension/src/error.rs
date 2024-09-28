use anchor_lang::error_code;

#[error_code]
pub enum PensionError {
    #[msg("Cooldown period has not expired yet")]
    CooldownNotExpired,
    #[msg("Invalid expected year")]
    InvalidExpectedYear,
    #[msg("Invalid expected amount")]
    InvalidExpectedAmount,
    #[msg("Insufficient balance")]
    InsufficientBalance,
}
