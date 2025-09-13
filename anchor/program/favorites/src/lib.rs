use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(_ctx: Context<SetFavorites>,
                         _color: String,
                        _number: u64,
                        _hobbies: Vec<String) -> Result<()> {
        msg!("Greetings! from {}", _ctx.account.program_id);
        let user_key = _ctx.accounts.user.key();

        msg!("The user {user_key}'s favorite")
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info > {

    #[account(mut)]
    pub user: Signer<'info >,

    #[account(
        init,
        payer = user,
        size = 8 + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]

    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub Struct Favorites{
    _number: u64,

    #[max_len(50)]
    _color: String,

    #[max_len(50)]
    _hobbies: Vec<String>,
}