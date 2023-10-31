// Execution Layer hard forks https://ethereum.org/en/history/
pub const SHANGHAI_BLOCK_NUMBER: u64 = 17034870;
pub const MERGE_BLOCK_NUMBER: u64 = 15537394;
pub const LONDON_BLOCK_NUMBER: u64 = 12965000;
pub const BERLIN_BLOCK_NUMBER: u64 = 12244000;
pub const ISTANBUL_BLOCK_NUMBER: u64 = 9069000;
pub const CONSTANTINOPLE_BLOCK_NUMBER: u64 = 7280000;
pub const BYZANTIUM_BLOCK_NUMBER: u64 = 4370000;
pub const HOMESTEAD_BLOCK_NUMBER: u64 = 1150000;

// Tests can be written to be generate from this file.
// Tests data will be in a yaml key value format
// Tests consist of content key value pairs.
// For history tests the header for the respective block body/receipt must be place before
// so
// block header 1
// block body 1
// block receipt 1
// block header 2
// block receipt 2
// block header 3
// would be a valid test but
// block header 2
// block receipt 3
// block header 3
// this wouldn't be a valid test file as block receipt 3 doesn't have a header place before it
// related header, block body, receipts must be sequential
// We want this so block forks with different formats can be tested, by updating an external test repo
pub const TEST_DATA_FILE_PATH: &str = "./test-data/test_data_collection_of_forks_blocks.yaml";
