use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token};

declare_id!("YourProgramIDHere");

#[program]
pub mod rusty_collectibles {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, metadata_url: String) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;

        // Mint the token
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: mint.to_account_info(),
                    to: token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            1, // Minting exactly one token
        )?;

        msg!("NFT Minted with Metadata URL: {}", metadata_url);
        Ok(())
    }
}
