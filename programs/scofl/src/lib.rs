use anchor_lang::prelude::*;
//id sistemini düşün
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
//id sistemini bi düşün
//recieve de doğru şekilde kontrata para aldım mı
//get taskta clonelayınca doğru çalışır mı


#[program]
pub mod scofl {

    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>,name:String,description:String,target_balance:u64,tasks:Vec<task>) -> Result<()> {
        let project = &mut ctx.accounts.project; 
        project.name=name;
        project.description= description;
        project.tasks=tasks;
        project.raised_balance=0;
        project.target_balance=target_balance;

        Ok(())
    }

    pub fn recieve_balance(ctx:Context<RecieveBalance>,amount:u64)-> Result<()>{
        let project = &mut ctx.accounts.project; 
        let from = &ctx.accounts.from;
        let to = &ctx.accounts.to;
        let amount_of_lamports = amount;

        msg!("new invesment coming");
        **from.try_borrow_mut_lamports()? -= amount_of_lamports;
        **to.try_borrow_mut_lamports()? += amount_of_lamports;
        project.raised_balance+=amount_of_lamports;
        msg!("balance increased by {} and now has {}",amount,project.raised_balance);
        
        Ok(())
    }

    pub fn get_task(ctx: Context<GetTask>,task_id:u8)->Result<()>{
        let project = &mut ctx.accounts.project;
        let dev = ctx.accounts.dev.owner;

        for mut task in project.tasks.clone(){
            if task_id==task.id && task.contributer !=None{
                task.contributer=Some(*dev);
            }
        }
        msg!("task assigned to {}",dev);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,seeds=[signer.key.as_ref()],bump,payer=signer,space=16+256)]
    pub project: Account<'info,project>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecieveBalance<'info>{
    #[account(mut,seeds=[signer.key.as_ref()],bump)]
    pub project: Account<'info,project>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from:AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to:AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetTask<'info>{
    #[account(mut,seeds=[signer.key.as_ref()],bump)]
    pub project: Account<'info,project>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub dev:AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct project {
    pub name: String,
    pub description: String,
    pub tasks: Vec<task>,
    pub raised_balance: u64,
    pub target_balance: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct task{
    pub id: u8,
    pub name:String,
    pub description: String,
    pub contributer: Option<Pubkey>,
    pub price:u8,
    pub state: bool,

}
