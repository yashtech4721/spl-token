use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer};

declare_id!("2V6iSQwWKtxSXgNpmRe65tsB5HwJvHb6ijhY8EqQNP7J");

#[program]
pub mod token_program1 {

    use super::*;

    pub fn updatesol(ctx: Context<Solupdate>,amount:u64) -> ProgramResult{

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        );
        
        let result = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        );

        let amountf = amount as f64;
        let amountm = amountf*0.000000001;
        let tokenamount = amountm as u64;

        if result == Ok(()) {
            let cpi_program = ctx.accounts.token_program.clone();
                token::transfer(
                    CpiContext::new(
                        cpi_program,
                        anchor_spl::token::Transfer {
                            from:ctx.accounts.mintfrom.clone(),
                            to: ctx.accounts.mintto.clone(),
                            authority: ctx.accounts.authoritymint.clone(),
                        },
                    ),
                    tokenamount*100
                )?;
        }
        Ok(())

    }
    
    pub fn proxy_transfer<'a, 'b, 'c, 'info>(ctx: Context<ProxyTransfer>, amount: u64) -> ProgramResult {
        msg!("Transfer:{:?}",ctx.accounts.authority);
        // msg!("{:?}",cpi_program);
        let cpi_program = ctx.accounts.token_program.clone();
        token::transfer(
            CpiContext::new(
                cpi_program,
                anchor_spl::token::Transfer {
                    from:ctx.accounts.from.clone(),
                    to: ctx.accounts.to.clone(),
                    authority: ctx.accounts.authority.clone(),
                },
            ),
            amount
        )?;

        let amountf = amount as f64;
        let amountm = amountf*0.01;
        let amounti = amountm as u64;
        msg!("amount:{:?}",amounti);

        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.clone(),
                anchor_spl::token::Burn {
                    mint:ctx.accounts.mint.clone(),
                    to: ctx.accounts.from.clone(),
                    authority: ctx.accounts.authority.clone(),
                },
            ),
            amounti
        )?;
        
        Ok(())
        // token::transfer(ctx.accounts.into(), amount)
    }
    
    pub fn proxy_mint_to(ctx: Context<ProxyMintTo>, amount: u64) -> ProgramResult {
        token::mint_to(ctx.accounts.into(), amount)
    }
    
    pub fn proxy_burn(ctx: Context<ProxyBurn>, amount: u64) -> ProgramResult {
        token::burn(ctx.accounts.into(), amount)
    }

    pub fn proxy_set_authority(
        ctx: Context<ProxySetAuthority>,
        authority_type: AuthorityType,
        new_authority: Option<Pubkey>,
    ) -> ProgramResult {
        token::set_authority(ctx.accounts.into(), authority_type.into(), new_authority)
    }

}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount
}

#[derive(Accounts)]
pub struct ProxyTransfer<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyMintTo<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyBurn<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Solupdate<'info> {
    #[account(signer)]
    pub authoritymint: AccountInfo<'info>,
    #[account(mut,signer)]
    pub from:AccountInfo<'info>,
    #[account(mut)]
    pub to : AccountInfo<'info>,
    #[account(mut)]
    pub mintfrom: AccountInfo<'info>,
    #[account(mut)]
    pub mintto: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct ProxySetAuthority<'info> {
    #[account(signer)]
    pub current_authority: AccountInfo<'info>,
    #[account(mut)]
    pub account_or_mint: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyTransfer<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut ProxyTransfer<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.from.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };

        let cpi_program = accounts.token_program.clone();
        msg!("{:?}",cpi_program);
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyMintTo<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &mut ProxyMintTo<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.mint.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyBurn<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
    fn from(accounts: &mut ProxyBurn<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.mint.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxySetAuthority<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>>
{
    fn from(
        accounts: &mut ProxySetAuthority<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts.account_or_mint.clone(),
            current_authority: accounts.current_authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl From<AuthorityType> for spl_token::instruction::AuthorityType {
    fn from(authority_ty: AuthorityType) -> spl_token::instruction::AuthorityType {
        match authority_ty {
            AuthorityType::MintTokens => spl_token::instruction::AuthorityType::MintTokens,
            AuthorityType::FreezeAccount => spl_token::instruction::AuthorityType::FreezeAccount,
            AuthorityType::AccountOwner => spl_token::instruction::AuthorityType::AccountOwner,
            AuthorityType::CloseAccount => spl_token::instruction::AuthorityType::CloseAccount,
        }
    }
}
