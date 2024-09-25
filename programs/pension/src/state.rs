use anchor_lang::prelude::*;

#[account]
pub struct Pension {
    pub expected_lamports: u16, // 预期每月存入金额
    pub expected_year: u8,      // 预期存入年数
}

#[account]
pub struct UserAccount {
    pub usdc_balance: u64,
    pub expected_usdc: u16,
    pub expected_year: u8,
}

impl UserAccount {
    pub const LEN: usize = 8 + 8 + 2 + 1; // 根据实际字段调整大小
}