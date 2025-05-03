export const SAMPLE_CONTRACTS = {
	ink: `#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
`,
	solidity: `// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Flipper {
    bool private value;

    constructor(bool initValue) {
        value = initValue;
    }

    function flip() public {
        value = !value;
    }

    function get() public view returns (bool) {
        return value;
    }
}
`,
};
