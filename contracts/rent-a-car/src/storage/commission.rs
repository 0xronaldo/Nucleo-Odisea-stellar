use soroban_sdk::Env;

use super::types::storage::DataKey;

pub fn write_commission(env: &Env, amount: &i128) {
    let key = DataKey::Commission;
    env.storage().instance().set(&key, amount);
}

pub fn read_commission(env: &Env) -> i128 {
    let key = DataKey::Commission;
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or(0)
}

pub fn write_commission_balance(env: &Env, balance: &i128) {
    let key = DataKey::CommissionBalance;
    env.storage().instance().set(&key, balance);
}

pub fn read_commission_balance(env: &Env) -> i128 {
    let key = DataKey::CommissionBalance;
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or(0)
}

pub fn add_commission_balance(env: &Env, amount: &i128) {
    let current_balance = read_commission_balance(env);
    let new_balance = current_balance + amount;
    write_commission_balance(env, &new_balance);
}

pub fn subtract_commission_balance(env: &Env, amount: &i128) {
    let current_balance = read_commission_balance(env);
    let new_balance = current_balance - amount;
    write_commission_balance(env, &new_balance);
}
