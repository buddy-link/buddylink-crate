// @ts-check
"use strict";
const {tmpLedgerDir} = require("@metaplex-foundation/amman");

const programs = [];

const validator = {
    killRunningValidators: true,
    verifyFees: false,
    commitment: "confirmed",
    programs,
    jsonRpcUrl: "http://127.0.0.1:8899/",
    websocketUrl: "",
    resetLedger: true,
    ledgerDir: tmpLedgerDir(),
    matchFeatures: "devnet",
    accountsCluster: 'https://api.devnet.solana.com',
    accounts: [
        {
            label: 'Metadata Program',
            accountId: 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
            executable: true,
        },
        {
            label: 'BuddyLink',
            accountId: '9zE4EQ5tJbEeMYwtS2w8KrSHTtTW4UPqwfbBSEkUrNCA',
            executable: true,
        },
        {
            label: 'Master Organization',
            accountId: 'CRY1kbdXSDkK2fHP8aPMCH3dtwtWBpdc81tyt8XVVunH',
            executable: false,
        },
        {
            label: 'Organization',
            accountId: 'Vygga65LjTWs7kJR9Z7JbCpBpcuc7Tn8LkjXoxHomRQ',
            executable: false,
        },
        {
            label: 'Mint',
            accountId: '3Q6dz8cLd4BW1kyuGyUaS7qhTtFP7tGS55Y7fybCUfNy',
            executable: false,
        },
        {
            label: 'Referrer Authority',
            accountId: 'DK1FtDDy2RkydDuhprUNKmsyVv8JQb5YDrUZe3GB8ZFc',
            executable: false,
        },
        {
            label: 'Referrer GB',
            accountId: '4jHbHkwjJoZgDBsx774LAmmqxPuGwk65SVdV6yr5Xjsm',
            executable: false,
        },
        {
            label: 'Referrer Treasury',
            accountId: 'AsY9QzsVwu6KX9N5Yy85M5jaMYitPYrAC7vuCY6A3YKf',
            executable: false,
        },
        {
            label: 'Referrer ATA',
            accountId: 'C4yA9kJKohWhmGKAMGhJWRB827UdR6aVRUu82mGnmNwV',
            executable: false,
        },
        {
            label: 'Referrer Member',
            accountId: 'GZ3oVbxW1LY26LsbZKJEqbv7AXiJGsMtW9emm4wdexN9',
            executable: false,
        },
        {
            label: 'Referee Authority',
            accountId: 'HFnGHHTEKdggiHVFYEs1VAKKmjPvoD31HQsApkZqHqEx',
            executable: false,
        },
        {
            label: 'Referee GB',
            accountId: 'DLCAgJho2Fm3g2SEWHzfiuJdLMRhqUVDWtssqhCeCdYr',
            executable: false,
        },
        {
            label: 'Referee Treasury',
            accountId: 'CMLckMKGfa5MTeovcfr9Rhgg31rFcAGS9ZKMXigaHMWJ',
            executable: false,
        },
        {
            label: 'Referee ATA',
            accountId: 'C2LZp5DNf6JsjEWoiQiS1jXQPopi3y4K2H7zN6App7mH',
            executable: false,
        },
        {
            label: 'Referee Member',
            accountId: '9xNqfpwRUEyqpURcgNZWFUrurrSRtdQTYpUQGbpJXWpp',
            executable: false,
        },
    ]
};

module.exports = {
    programs,
    validator,
    storage: {
        storageId: "js-next-sdk",
        clearOnStart: true,
    },
    relay: {enabled: true},
};
