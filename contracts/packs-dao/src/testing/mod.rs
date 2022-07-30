use cosmwasm_std::testing::{mock_dependencies, MockApi, MockQuerier, MockStorage};
use cosmwasm_std::OwnedDeps;

const TEST_OWNER: &str = "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v";
const TEST_DAO_NAME: &str = "Packs DAO";
const TEST_MULTISIG_ADDRESS: &str = "gov000";
const TEST_GOV_CODE_ID: u64 = 2;
const TEST_GROUP_CODE_ID: u64 = 3;

type MockDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

fn mock_deps() -> MockDeps {
    mock_dependencies()
}
