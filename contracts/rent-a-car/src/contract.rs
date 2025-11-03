use crate::{
    interfaces::contract::RentACarContractTrait,
    storage::{
        admin::{read_admin, write_admin}, 
        car::{read_car, remove_car, write_car}, 
        commission::{add_commission_balance, read_commission, read_commission_balance, subtract_commission_balance, write_commission},
        rental::write_rental, 
        structs::{car::Car, rental::Rental}, 
        token::write_token, 
        types::car_status::CarStatus
    },
};
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct RentACarContract;

#[contractimpl]
impl RentACarContractTrait for RentACarContract {
    fn __constructor(env: &Env, admin: Address, token: Address) {
        write_admin(env, &admin);
        write_token(env, &token);
    }

    fn get_admin(env: &Env) -> Address {
        read_admin(env)
    }

    fn add_car(env: &Env, owner: Address, price_per_day: i128) {
        let car = Car {
            price_per_day,
            car_status: CarStatus::Available,
        };

        write_car(env, &owner, &car);
    }

    fn get_car_status(env: &Env, owner: Address) -> CarStatus {
        let car = read_car(env, &owner);

        car.car_status
    }

    fn rental(env: &Env, renter: Address, owner: Address, total_days_to_rent: u32, amount: i128) {
        let mut car = read_car(env, &owner);

        car.car_status = CarStatus::Rented;

        let commission = read_commission(env);
        add_commission_balance(env, &commission);

        let rental = Rental {
            total_days_to_rent,
            amount,
        };

        write_car(env, &owner, &car);
        write_rental(env, &renter, &owner, &rental);
    }

    fn remove_car(env: &Env, owner: Address) {
        remove_car(env, &owner);
    }

    fn set_commission(env: &Env, admin: Address, commission: i128) {
        let current_admin = read_admin(env);
        assert!(admin == current_admin, "Only admin can set commission");
        write_commission(env, &commission);
    }

    fn get_commission(env: &Env) -> i128 {
        read_commission(env)
    }

    fn get_commission_balance(env: &Env) -> i128 {
        read_commission_balance(env)
    }

    fn withdraw_commission(env: &Env, admin: Address, amount: i128) {
        let current_admin = read_admin(env);
        assert!(admin == current_admin, "Only admin can withdraw commission");
        
        let commission_balance = read_commission_balance(env);
        assert!(amount <= commission_balance, "Insufficient commission balance");
        
        subtract_commission_balance(env, &amount);
    }

    fn withdraw_owner(env: &Env, owner: Address, _amount: i128) {
        let car = read_car(env, &owner);
        assert!(car.car_status == CarStatus::Available, "Car must be available to withdraw");
    }

    fn return_car(env: &Env, owner: Address) {
        let mut car = read_car(env, &owner);
        car.car_status = CarStatus::Available;
        write_car(env, &owner, &car);
    }
}
