use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8,
}

#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    pub fn increment(&mut self) {
        self.val += 1;
        let log_message = format!("Increased number to, {}", self.val);

        env::log(log_message.as_bytes());
        after_counter_change();
    }

    pub fn decrement(&mut self) {
        self.val -= 1;
        let log_message = format!("Decreased number to {}", self.val);

        env::log(log_message.as_bytes());
        after_counter_change();
    }

    pub fn reset(&mut self) {
        self.val = 0;

        // Cast a string (`b`)
        env::log(b"Reset counter to zero");
    }
}

fn after_counter_change() {
    env::log("Make sure to avoid overflow".as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, MockedBlockchain, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            epoch_height: 19,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(10),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }
    #[test]
    fn increment() {
        // Set up mock context into the testing environment
        let context = get_context(vec![], false);

        testing_env!(context);

        // Instantiate a contract variable with counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment();

        println!("Value after increment: {}", contract.get_num());
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let context = get_context(vec![], false);

        testing_env!(context);

        let mut contract = Counter { val: 2 };
        contract.decrement();

        println!("Value after decrement: {}", contract.get_num());
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn rest() {
        let context = get_context(vec![], false);

        testing_env!(context);

        let mut contract = Counter { val: 2 };
        contract.increment();
        contract.reset();

        println!("Value after reset: {}", contract.get_num());
        assert_eq!(0, contract.get_num());
    }
}
