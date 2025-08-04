use smart_contract_sdk::contract;

#[contract]
mod my_contract {
    #[entrypoint]
    pub fn init() {}

    #[view]
    pub fn get_value(x: u32) -> u32 {
        x
    }

    #[tx]
    pub fn set_value(x: u32) {
        let _ = x;
    }
}

fn main() {}
