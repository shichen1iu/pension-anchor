use anchor_lang::prelude::*;

#[account]
pub struct Pension {
    pub amount: u64, // 存入金额 8byte
    pub expected_amount: u64, // 预期每月存入金额 2byte
    pub expected_year: u8,      // 预期存入年数 1byte
    pub cooldown: i64,          // 冷却时间 8byte
}
impl Pension {
    pub const LEN: usize = 8 + 8 + 2 + 1 + 8; 
}
