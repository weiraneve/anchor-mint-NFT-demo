use anchor_lang::prelude::*;

declare_id!("sEVXzgRND6bkGiWz2uV1j6NTm85egusBwkHUfQaJLMo");

#[program]
pub mod anchor_mint_nft_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
