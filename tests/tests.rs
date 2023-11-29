use concordium_smart_contract_testing::*;
use voting::*;

/// A test account.
const ALICE: AccountAddress = AccountAddress([0u8; 32]);
const ALICE_ADDR: Address = Address::Account(ALICE);

const BOB: AccountAddress = AccountAddress([1u8; 32]);
const BOB_ADDR: Address = Address::Account(BOB);

const CARLY: AccountAddress = AccountAddress([2u8; 32]);
const CARLY_ADDR: Address = Address::Account(CARLY);

/// The initial balance of the ALICE test account.
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(10_000);

/// A [`Signer`] with one set of keys, used for signing transactions.
const SIGNER: Signer = Signer::with_one_key();

#[test]
fn test_voting() {

    let mut chain = Chain::new();

    chain.create_account(Account::new(ALICE, ACC_INITIAL_BALANCE));
    chain.create_account(Account::new(BOB, ACC_INITIAL_BALANCE));

    let module = module_load_v1("./concordium-out/module.wasm.v1").unwrap();

    let deployment = chain.module_deploy_v1(SIGNER, ALICE, module).unwrap();

    let init_parameter = InitParameter {
        description: "description".into(),
        options: vec!("DK".to_string(), "IT".to_string(), "SE".to_string()),
        end_time: Timestamp::from_timestamp_millis(1000),
    };

    let init = chain.contract_init(
        SIGNER, 
        ALICE, 
        Energy::from(10000), 
        InitContractPayload {
            amount: Amount::zero(), 
            mod_ref: deployment.module_reference, 
            init_name: OwnedContractName::new_unchecked("init_voting".to_string()), 
            param: OwnedParameter::from_serial(&init_parameter).unwrap(),
        },
    ).unwrap();

    let update_1 = chain.contract_update(
        SIGNER, 
        ALICE, 
        ALICE_ADDR, 
        Energy::from(10000), 
        UpdateContractPayload {
            amount: Amount::zero(),
            address: init.contract_address,
            receive_name: OwnedReceiveName::new_unchecked("voting.vote".to_string()),
            message: OwnedParameter::from_serial(&"DK".to_string()).unwrap(),
        },
    ).unwrap();

    let update_2 = chain.contract_update(
        SIGNER, 
        BOB, 
        BOB_ADDR, 
        Energy::from(10000), 
        UpdateContractPayload {
            amount: Amount::zero(),
            address: init.contract_address,
            receive_name: OwnedReceiveName::new_unchecked("voting.vote".to_string()),
            message: OwnedParameter::from_serial(&"IT".to_string()).unwrap(),
        },
    ).unwrap();

    let update_3 = chain.contract_update(
        SIGNER, 
        CARLY, 
        CARLY_ADDR, 
        Energy::from(10000), 
        UpdateContractPayload {
            amount: Amount::zero(),
            address: init.contract_address,
            receive_name: OwnedReceiveName::new_unchecked("voting.vote".to_string()),
            message: OwnedParameter::from_serial(&"SE".to_string()).unwrap(),
        },
    ).unwrap();

    let view = chain.contract_invoke(
        BOB, 
        BOB_ADDR, 
        Energy::from(10000), 
        UpdateContractPayload {
            amount: Amount::zero(),
            address: init.contract_address,
            receive_name: OwnedReceiveName::new_unchecked("voting.view".to_string()),
            message: OwnedParameter::empty(),
        },
    ).unwrap();


    let view_data: ViewData = view.parse_return_value().unwrap();
    assert_eq!(view_data == ["IT": 1], ["SE": 1], ["DK": 1],);

}
