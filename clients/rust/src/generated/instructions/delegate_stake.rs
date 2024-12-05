//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct DelegateStake {
    /// Initialized stake account to be delegated
    pub stake: solana_program::pubkey::Pubkey,
    /// Vote account to which this stake will be delegated
    pub vote: solana_program::pubkey::Pubkey,
    /// Clock sysvar
    pub clock_sysvar: solana_program::pubkey::Pubkey,
    /// Stake history sysvar
    pub stake_history: solana_program::pubkey::Pubkey,
    /// Unused account, formerly the stake config
    pub unused: solana_program::pubkey::Pubkey,
    /// Stake authority
    pub stake_authority: solana_program::pubkey::Pubkey,
}

impl DelegateStake {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vote, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.clock_sysvar,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_history,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.unused,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = DelegateStakeInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct DelegateStakeInstructionData {
    discriminator: u8,
}

impl DelegateStakeInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 2 }
    }
}

impl Default for DelegateStakeInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `DelegateStake`.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[]` vote
///   2. `[optional]` clock_sysvar (default to `SysvarC1ock11111111111111111111111111111111`)
///   3. `[]` stake_history
///   4. `[]` unused
///   5. `[signer]` stake_authority
#[derive(Clone, Debug, Default)]
pub struct DelegateStakeBuilder {
    stake: Option<solana_program::pubkey::Pubkey>,
    vote: Option<solana_program::pubkey::Pubkey>,
    clock_sysvar: Option<solana_program::pubkey::Pubkey>,
    stake_history: Option<solana_program::pubkey::Pubkey>,
    unused: Option<solana_program::pubkey::Pubkey>,
    stake_authority: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl DelegateStakeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Initialized stake account to be delegated
    #[inline(always)]
    pub fn stake(&mut self, stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake = Some(stake);
        self
    }
    /// Vote account to which this stake will be delegated
    #[inline(always)]
    pub fn vote(&mut self, vote: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vote = Some(vote);
        self
    }
    /// `[optional account, default to 'SysvarC1ock11111111111111111111111111111111']`
    /// Clock sysvar
    #[inline(always)]
    pub fn clock_sysvar(&mut self, clock_sysvar: solana_program::pubkey::Pubkey) -> &mut Self {
        self.clock_sysvar = Some(clock_sysvar);
        self
    }
    /// Stake history sysvar
    #[inline(always)]
    pub fn stake_history(&mut self, stake_history: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_history = Some(stake_history);
        self
    }
    /// Unused account, formerly the stake config
    #[inline(always)]
    pub fn unused(&mut self, unused: solana_program::pubkey::Pubkey) -> &mut Self {
        self.unused = Some(unused);
        self
    }
    /// Stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.stake_authority = Some(stake_authority);
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
        let accounts = DelegateStake {
            stake: self.stake.expect("stake is not set"),
            vote: self.vote.expect("vote is not set"),
            clock_sysvar: self.clock_sysvar.unwrap_or(solana_program::pubkey!(
                "SysvarC1ock11111111111111111111111111111111"
            )),
            stake_history: self.stake_history.expect("stake_history is not set"),
            unused: self.unused.expect("unused is not set"),
            stake_authority: self.stake_authority.expect("stake_authority is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `delegate_stake` CPI accounts.
pub struct DelegateStakeCpiAccounts<'a, 'b> {
    /// Initialized stake account to be delegated
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vote account to which this stake will be delegated
    pub vote: &'b solana_program::account_info::AccountInfo<'a>,
    /// Clock sysvar
    pub clock_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake history sysvar
    pub stake_history: &'b solana_program::account_info::AccountInfo<'a>,
    /// Unused account, formerly the stake config
    pub unused: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `delegate_stake` CPI instruction.
pub struct DelegateStakeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Initialized stake account to be delegated
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vote account to which this stake will be delegated
    pub vote: &'b solana_program::account_info::AccountInfo<'a>,
    /// Clock sysvar
    pub clock_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake history sysvar
    pub stake_history: &'b solana_program::account_info::AccountInfo<'a>,
    /// Unused account, formerly the stake config
    pub unused: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> DelegateStakeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: DelegateStakeCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            stake: accounts.stake,
            vote: accounts.vote,
            clock_sysvar: accounts.clock_sysvar,
            stake_history: accounts.stake_history,
            unused: accounts.unused,
            stake_authority: accounts.stake_authority,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vote.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.clock_sysvar.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_history.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.unused.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = DelegateStakeInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.vote.clone());
        account_infos.push(self.clock_sysvar.clone());
        account_infos.push(self.stake_history.clone());
        account_infos.push(self.unused.clone());
        account_infos.push(self.stake_authority.clone());
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

/// Instruction builder for `DelegateStake` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[]` vote
///   2. `[]` clock_sysvar
///   3. `[]` stake_history
///   4. `[]` unused
///   5. `[signer]` stake_authority
#[derive(Clone, Debug)]
pub struct DelegateStakeCpiBuilder<'a, 'b> {
    instruction: Box<DelegateStakeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> DelegateStakeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(DelegateStakeCpiBuilderInstruction {
            __program: program,
            stake: None,
            vote: None,
            clock_sysvar: None,
            stake_history: None,
            unused: None,
            stake_authority: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Initialized stake account to be delegated
    #[inline(always)]
    pub fn stake(&mut self, stake: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.stake = Some(stake);
        self
    }
    /// Vote account to which this stake will be delegated
    #[inline(always)]
    pub fn vote(&mut self, vote: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vote = Some(vote);
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
    /// Stake history sysvar
    #[inline(always)]
    pub fn stake_history(
        &mut self,
        stake_history: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_history = Some(stake_history);
        self
    }
    /// Unused account, formerly the stake config
    #[inline(always)]
    pub fn unused(
        &mut self,
        unused: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.unused = Some(unused);
        self
    }
    /// Stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_authority = Some(stake_authority);
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
        let instruction = DelegateStakeCpi {
            __program: self.instruction.__program,

            stake: self.instruction.stake.expect("stake is not set"),

            vote: self.instruction.vote.expect("vote is not set"),

            clock_sysvar: self
                .instruction
                .clock_sysvar
                .expect("clock_sysvar is not set"),

            stake_history: self
                .instruction
                .stake_history
                .expect("stake_history is not set"),

            unused: self.instruction.unused.expect("unused is not set"),

            stake_authority: self
                .instruction
                .stake_authority
                .expect("stake_authority is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct DelegateStakeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vote: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    clock_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_history: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    unused: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
