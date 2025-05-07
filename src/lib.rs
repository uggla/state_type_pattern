pub mod machin;

use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

pub struct Bidule {
    pub name: String,
    pub machine: String,
}

impl Bidule {
    pub fn new(name: &str, machine: &str) -> Bidule {
        Bidule {
            name: name.to_string(),
            machine: machine.to_string(),
        }
    }
}

impl Debug for Bidule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Bidule {{ name: {}, machine: {} }}",
            self.name, self.machine
        )
    }
}

pub trait Lockable {
    fn status(&self) -> String;
}

pub struct Vault<T> {
    gold: u32,
    silver: u32,
    status: PhantomData<T>,
}

pub struct Locked;
pub struct Unlocked;

impl Vault<Unlocked> {
    pub fn deposit(&mut self, gold: u32, silver: u32) {
        self.gold += gold;
        self.silver += silver;
    }

    pub fn withdraw(&mut self, gold: u32, silver: u32) {
        self.gold -= gold;
        self.silver -= silver;
    }

    pub fn lock(self) -> Vault<Locked> {
        Vault {
            gold: self.gold,
            silver: self.silver,
            status: PhantomData,
        }
    }
}

impl Lockable for Vault<Unlocked> {
    fn status(&self) -> String {
        let msg = format!(
            "Vault status: Unlocked\nVault contains gold: {:#?}, silver: {:#?}",
            self.gold, self.silver
        );
        println!("{}", msg);
        msg
    }
}

impl Vault<Locked> {
    pub fn new() -> Vault<Locked> {
        Self::default()
    }
    pub fn unlock(self) -> Vault<Unlocked> {
        Vault {
            gold: self.gold,
            silver: self.silver,
            status: PhantomData,
        }
    }
}

impl Default for Vault<Locked> {
    fn default() -> Self {
        Self {
            gold: 100,
            silver: 100,
            status: PhantomData,
        }
    }
}

impl Lockable for Vault<Locked> {
    fn status(&self) -> String {
        let msg = "Vault status: Locked\nYou cannot see the containt.".to_string();
        println!("{}", msg);
        msg
    }
}
