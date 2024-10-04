use anchor_lang::error_code;

#[error_code]
pub enum PensionError {
    #[msg("请30天后再存款")]
    CooldownNotExpired,
    #[msg("请传入自己的token账户")]
    InvalidTokenOwner,
    #[msg("请传入usdc/usdt的mint地址")]
    InvalidTokenMint,
    #[msg("请传入正确的token账户")]
    InvalidTokenAccount,
    #[msg("未达到自动关闭账户时间")]
    AccountClosureTimeNotYetReached,
    #[msg("请传入正确的预期月份")]
    InvalidExpectedMonth,
}
