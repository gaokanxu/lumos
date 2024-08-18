use lumos_sdk::{
    account::{Account, AccountSharedData},
    bpf_loader_upgradeable::UpgradeableLoaderState,
    pubkey::Pubkey,
    rent::Rent,
};

mod lpl_token {
    lumos_sdk::declare_id!("unknown111111111111111111111111111111111111");
}
mod lpl_token_2022 {
    lumos_sdk::declare_id!("unknown111111111111111111111111111111111111");
}
mod lpl_memo_1_0 {
    lumos_sdk::declare_id!("unknown111111111111111111111111111111111111");
}
mod lpl_memo_3_0 {
    lumos_sdk::declare_id!("unknown111111111111111111111111111111111111");
}
mod lpl_associated_token_account {
    lumos_sdk::declare_id!("unknown111111111111111111111111111111111111");
}

static LPL_PROGRAMS: &[(Pubkey, Pubkey, &[u8])] = &[
    (
        lpl_token::ID,
        lumos_sdk::bpf_loader::ID,
        include_bytes!("programs/lpl_token-3.5.0.so"),
    ),
    (
        lpl_token_2022::ID,
        lumos_sdk::bpf_loader_upgradeable::ID,
        include_bytes!("programs/lpl_token_2022-1.0.0.so"),
    ),
    (
        lpl_memo_1_0::ID,
        lumos_sdk::bpf_loader::ID,
        include_bytes!("programs/lpl_memo-1.0.0.so"),
    ),
    (
        lpl_memo_3_0::ID,
        lumos_sdk::bpf_loader::ID,
        include_bytes!("programs/lpl_memo-3.0.0.so"),
    ),
    (
        lpl_associated_token_account::ID,
        lumos_sdk::bpf_loader::ID,
        include_bytes!("programs/lpl_associated_token_account-1.1.1.so"),
    ),
];

pub fn lpl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    LPL_PROGRAMS
        .iter()
        .flat_map(|(program_id, loader_id, elf)| {
            let mut accounts = vec![];
            let data = if *loader_id == lumos_sdk::bpf_loader_upgradeable::ID {
                let (programdata_address, _) =
                    Pubkey::find_program_address(&[program_id.as_ref()], loader_id);
                let mut program_data = bincode::serialize(&UpgradeableLoaderState::ProgramData {
                    slot: 0,
                    upgrade_authority_address: Some(Pubkey::default()),
                })
                .unwrap();
                program_data.extend_from_slice(elf);
                accounts.push((
                    programdata_address,
                    AccountSharedData::from(Account {
                        lamports: rent.minimum_balance(program_data.len()).max(1),
                        data: program_data,
                        owner: *loader_id,
                        executable: false,
                        rent_epoch: 0,
                    }),
                ));
                bincode::serialize(&UpgradeableLoaderState::Program {
                    programdata_address,
                })
                .unwrap()
            } else {
                elf.to_vec()
            };
            accounts.push((
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(data.len()).max(1),
                    data,
                    owner: *loader_id,
                    executable: true,
                    rent_epoch: 0,
                }),
            ));
            accounts.into_iter()
        })
        .collect()
}
