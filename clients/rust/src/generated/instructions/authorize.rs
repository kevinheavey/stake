//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use {
    crate::generated::types::StakeAuthorize,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::pubkey::Pubkey,
};

/// Accounts.
pub struct Authorize {
    /// Stake account to be updated
    pub stake: solana_program::pubkey::Pubkey,
    /// Clock sysvar
    pub clock_sysvar: solana_program::pubkey::Pubkey,
    /// Stake or withdraw authority
    pub authority: solana_program::pubkey::Pubkey,
    /// Lockup authority
    pub lockup_authority: Option<solana_program::pubkey::Pubkey>,
}

impl Authorize {
    pub fn instruction(
        &self,
        args: AuthorizeInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: AuthorizeInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.clock_sysvar,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        if let Some(lockup_authority) = self.lockup_authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                lockup_authority,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::STAKE_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = AuthorizeInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AuthorizeInstructionData {
    discriminator: u8,
}

impl AuthorizeInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 1 }
    }
}

impl Default for AuthorizeInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuthorizeInstructionArgs {
    pub arg0: Pubkey,
    pub arg1: StakeAuthorize,
}

/// Instruction builder for `Authorize`.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[optional]` clock_sysvar (default to `SysvarC1ock11111111111111111111111111111111`)
///   2. `[signer]` authority
///   3. `[signer, optional]` lockup_authority
#[derive(Clone, Debug, Default)]
pub struct AuthorizeBuilder {
    stake: Option<solana_program::pubkey::Pubkey>,
    clock_sysvar: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    lockup_authority: Option<solana_program::pubkey::Pubkey>,
    arg0: Option<Pubkey>,
    arg1: Option<StakeAuthorize>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AuthorizeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Stake account to be updated
    #[inline(always)]
    pub fn stake(&mut self, stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake = Some(stake);
        self
    }
    /// `[optional account, default to 'SysvarC1ock11111111111111111111111111111111']`
    /// Clock sysvar
    #[inline(always)]
    pub fn clock_sysvar(&mut self, clock_sysvar: solana_program::pubkey::Pubkey) -> &mut Self {
        self.clock_sysvar = Some(clock_sysvar);
        self
    }
    /// Stake or withdraw authority
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// `[optional account]`
    /// Lockup authority
    #[inline(always)]
    pub fn lockup_authority(
        &mut self,
        lockup_authority: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.lockup_authority = lockup_authority;
        self
    }
    #[inline(always)]
    pub fn arg0(&mut self, arg0: Pubkey) -> &mut Self {
        self.arg0 = Some(arg0);
        self
    }
    #[inline(always)]
    pub fn arg1(&mut self, arg1: StakeAuthorize) -> &mut Self {
        self.arg1 = Some(arg1);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Authorize {
            stake: self.stake.expect("stake is not set"),
            clock_sysvar: self.clock_sysvar.unwrap_or(solana_program::pubkey!(
                "SysvarC1ock11111111111111111111111111111111"
            )),
            authority: self.authority.expect("authority is not set"),
            lockup_authority: self.lockup_authority,
        };
        let args = AuthorizeInstructionArgs {
            arg0: self.arg0.clone().expect("arg0 is not set"),
            arg1: self.arg1.clone().expect("arg1 is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `authorize` CPI accounts.
pub struct AuthorizeCpiAccounts<'a, 'b> {
    /// Stake account to be updated
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Clock sysvar
    pub clock_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake or withdraw authority
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Lockup authority
    pub lockup_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `authorize` CPI instruction.
pub struct AuthorizeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake account to be updated
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Clock sysvar
    pub clock_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake or withdraw authority
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Lockup authority
    pub lockup_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: AuthorizeInstructionArgs,
}

impl<'a, 'b> AuthorizeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AuthorizeCpiAccounts<'a, 'b>,
        args: AuthorizeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            stake: accounts.stake,
            clock_sysvar: accounts.clock_sysvar,
            authority: accounts.authority,
            lockup_authority: accounts.lockup_authority,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.clock_sysvar.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        if let Some(lockup_authority) = self.lockup_authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *lockup_authority.key,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::STAKE_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = AuthorizeInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.clock_sysvar.clone());
        account_infos.push(self.authority.clone());
        if let Some(lockup_authority) = self.lockup_authority {
            account_infos.push(lockup_authority.clone());
        }
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Authorize` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[]` clock_sysvar
///   2. `[signer]` authority
///   3. `[signer, optional]` lockup_authority
#[derive(Clone, Debug)]
pub struct AuthorizeCpiBuilder<'a, 'b> {
    instruction: Box<AuthorizeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AuthorizeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AuthorizeCpiBuilderInstruction {
            __program: program,
            stake: None,
            clock_sysvar: None,
            authority: None,
            lockup_authority: None,
            arg0: None,
            arg1: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Stake account to be updated
    #[inline(always)]
    pub fn stake(&mut self, stake: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.stake = Some(stake);
        self
    }
    /// Clock sysvar
    #[inline(always)]
    pub fn clock_sysvar(
        &mut self,
        clock_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.clock_sysvar = Some(clock_sysvar);
        self
    }
    /// Stake or withdraw authority
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// `[optional account]`
    /// Lockup authority
    #[inline(always)]
    pub fn lockup_authority(
        &mut self,
        lockup_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.lockup_authority = lockup_authority;
        self
    }
    #[inline(always)]
    pub fn arg0(&mut self, arg0: Pubkey) -> &mut Self {
        self.instruction.arg0 = Some(arg0);
        self
    }
    #[inline(always)]
    pub fn arg1(&mut self, arg1: StakeAuthorize) -> &mut Self {
        self.instruction.arg1 = Some(arg1);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = AuthorizeInstructionArgs {
            arg0: self.instruction.arg0.clone().expect("arg0 is not set"),
            arg1: self.instruction.arg1.clone().expect("arg1 is not set"),
        };
        let instruction = AuthorizeCpi {
            __program: self.instruction.__program,

            stake: self.instruction.stake.expect("stake is not set"),

            clock_sysvar: self
                .instruction
                .clock_sysvar
                .expect("clock_sysvar is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            lockup_authority: self.instruction.lockup_authority,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct AuthorizeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    clock_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lockup_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    arg0: Option<Pubkey>,
    arg1: Option<StakeAuthorize>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
