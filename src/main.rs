mod runtime;
mod types;
mod support;
mod balances;
mod proof_of_existence;
mod system;

use runtime::{Runtime, RuntimeCall};

fn main() {
    // Create a new instance of the Runtime.
    // It will instantiate with it all the modules it uses.
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    // Initialize the system with some initial balance.
    runtime.balances.set_balance(&alice, 100);

    // Here are the extrinsics in our block.
    // You can add or remove these based on the modules and calls you have set up.
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::Balances(balances::Call::BurnBalance { account: alice.clone() }),
            }
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "Hello, world!",
                }),
            },
        ],
    };

    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
                    claim: "Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: bob,
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "Hello, world!",
                }),
            },
        ],
    };

    // Execute the extrinsics which make up our blocks.
    // If there are any errors, our system panics, since we should not execute invalid blocks.
    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
    runtime.execute_block(block_3).expect("invalid block");

    // Simply print the debug format of our runtime state.
    println!("{:#?}", runtime);
}