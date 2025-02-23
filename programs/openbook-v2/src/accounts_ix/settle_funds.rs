use crate::error::OpenBookError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenInterface};

#[derive(Accounts)]
pub struct SettleFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub penalty_payer: Signer<'info>,

    #[account(
        mut,
        has_one = market,
        constraint = open_orders_account.load()?.is_owner_or_delegate(owner.key()) @ OpenBookError::NoOwnerOrDelegate
    )]
    pub open_orders_account: AccountLoader<'info, OpenOrdersAccount>,
    #[account(
        mut,
        has_one = market_base_vault,
        has_one = market_quote_vault,
        has_one = market_authority,
    )]
    pub market: AccountLoader<'info, Market>,
    /// CHECK: checked on has_one in market
    pub market_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_base_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(mut)]
    pub market_quote_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        token::mint = market_base_vault.mint,
        constraint = user_base_account.owner == open_orders_account.load()?.owner
    )]
    pub user_base_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        token::mint = market_quote_vault.mint,
        constraint = user_quote_account.owner == open_orders_account.load()?.owner
    )]
    pub user_quote_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        token::mint = market_quote_vault.mint
    )]
    pub referrer_account: Option<Box<InterfaceAccount<'info, token_interface::TokenAccount>>>,

    #[account(mut)]
    pub base_mint: Option<Box<InterfaceAccount<'info, token_interface::Mint>>>,
    #[account(mut)]
    pub quote_mint: Option<Box<InterfaceAccount<'info, token_interface::Mint>>>,

    pub base_token_program: Interface<'info, TokenInterface>,
    pub quote_token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
