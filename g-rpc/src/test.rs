#![cfg(test)]

use std::println;
use std::format;

use super::{Symbol};
use alloc::string::ToString;
use soroban_sdk::{Env, xdr::ToXdr};

extern crate alloc;
extern crate std;

#[test]
#[ignore]
fn generate_pew_symbol() {
    let env = Env::default();
        env.budget().reset_unlimited();

    let mut iters: i32 = 0;
    let mut has_pew = false;
    let symbol2 = Symbol::new(&env,"dn");

    while !has_pew {
        let symbol = Symbol::new(&env, format!("{}{}", symbol2.to_string(), iters.to_string()).as_str());

        let bytes = symbol.clone().to_xdr(&env);
        let hash = env.crypto().sha256(&bytes);
        let mut i = 0;

        for v in hash.clone().into_iter() {
            if v == 112
                && hash.get(i + 1).unwrap_or(0) == 101
                && hash.get(i + 2).unwrap_or(0) == 119
            {
                has_pew = true;

                if has_pew {
                    println!("has_pew: {:?}", has_pew);
                    println!("{:?}", bytes);
                    println!("{:?}", hash);
                    println!("{:?}", symbol);
                }
            }
            i += 1;
        }

        if iters % 10000 == 0 {
            println!("{:?}", &iters);
            // println!("{:?}", &hash);
            // println!("{:?}", &symbol);
        }

        // if iters >= 100000 {
        //     break;
        // }

        iters += 1;
    }

    println!("has_pew: {:?}", has_pew);
}

#[test]
#[ignore]
fn generate_pew_symbol_2() {
    let env = Env::default();

    let symbol = Symbol::new(&env,"dn208513");

    let bytes = symbol.clone().to_xdr(&env);
    let hash = env.crypto().sha256(&bytes);
    let mut i = 0;
    let mut has_pew = false;
    for v in hash.clone().into_iter() {
        if v == 112
            && hash.get(i + 1).unwrap_or(0) == 101
            && hash.get(i + 2).unwrap_or(0) == 119
        {
            has_pew = true;
        }
        i += 1;
    }
    println!("{:?}", symbol);
    println!("has_pew: {:?}", has_pew);
}