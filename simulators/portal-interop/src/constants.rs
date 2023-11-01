// Execution Layer hard forks https://ethereum.org/en/history/
pub const SHANGHAI_BLOCK_NUMBER: u64 = 17034870;
pub const MERGE_BLOCK_NUMBER: u64 = 15537394;
pub const LONDON_BLOCK_NUMBER: u64 = 12965000;
pub const BERLIN_BLOCK_NUMBER: u64 = 12244000;
pub const ISTANBUL_BLOCK_NUMBER: u64 = 9069000;
pub const CONSTANTINOPLE_BLOCK_NUMBER: u64 = 7280000;
pub const BYZANTIUM_BLOCK_NUMBER: u64 = 4370000;
pub const HOMESTEAD_BLOCK_NUMBER: u64 = 1150000;

// Tests will be automatically generated from the data in this file.
// Test data is written in a yaml key/value format
// For history tests, the header MUST be placed before its respective block body/receipt, to ensure that the data necessary for validation is available.
pub const TEST_DATA_FILE_PATH: &str = "./test-data/test_data_collection_of_forks_blocks.yaml";
