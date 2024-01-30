#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"),symbol_short!("Soroban")]
    }
}
#[cfg(test)]
mod test;