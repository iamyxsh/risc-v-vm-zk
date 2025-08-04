use smart_contract_sdk::contract_module;

#[contract_module]
mod contract {
    use smart_contract_sdk::{entrypoint, tx, view};

    #[entrypoint]
    fn init() {}

    #[view]
    fn get_value(x: u32) -> u32 {
        x
    }

    #[tx]
    fn set_value(x: u32) {
        let _ = x;
    }
}

fn main() {}
