use anchor_lang::prelude::*;

declare_id!("H1d1DkDnBFCrQmMEZJo1T2WVjZxsn28WpZ5caGd7V9nk"); // esto se actualiza solo al build

#[program]
pub mod videojuegos_el_macho {
    use super::*;

    // inicializando la tienda
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let store = &mut ctx.accounts.store;
        store.authority = ctx.accounts.authority.key();
        store.discount = 10; // descuento fijo 10%
        msg!("tienda videojuegos el macho inicializada con 10% descuento");
        Ok(())
    }

    // introducir videojuego nuevo
    pub fn create_game(ctx: Context<CreateGame>, name: String, price: u64, is_classic: bool) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.authority.key();
        game.name = name.clone();
        game.price = price;
        game.is_classic = is_classic;
        msg!("juego creado: {} precio {} clasico {}", name, price, is_classic);
        Ok(())
    }

    // actualiza precio
    pub fn update_game(ctx: Context<UpdateGame>, new_price: u64) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.price = new_price;
        msg!("precio actualizado a {}", new_price);
        Ok(())
    }

    // borra juego
    pub fn delete_game(ctx: Context<DeleteGame>) -> Result<()> {
        msg!("juego {} borrado", ctx.accounts.game.name);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1,
        seeds = [b"store", authority.key().as_ref()],
        bump
    )]
    pub store: Account<'info, Store>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGame<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 100 + 8 + 1, // nombre max ~100 chars para seguridad
        seeds = [b"game", authority.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGame<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub game: Account<'info, Game>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteGame<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority
    )]
    pub game: Account<'info, Game>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Store {
    pub authority: Pubkey,
    pub discount: u8,
}

#[account]
pub struct Game {
    pub authority: Pubkey,
    pub name: String,
    pub price: u64,
    pub is_classic: bool,
}