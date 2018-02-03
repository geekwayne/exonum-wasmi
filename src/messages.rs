
pub const SERVICE_ID: u16 = 1;

const TX_DEPLOY: u16 = 1;
const TX_CALL: u16 = 2;

message! {
    struct TxDeploy {
        const TYPE = SERVICE_ID;
        const ID = TX_DEPLOY;

        name: &str,
        module: &[u8],
    }
}

message! {
    struct TxCall {
        const TYPE = SERVICE_ID;
        const ID = TX_CALL;

        name: &str,
        seed: u64,
    }
}