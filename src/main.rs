use state_type_pattern::Lockable;
use state_type_pattern::Vault;

fn main() {
    let tresor = Vault::new();
    let mut tresor = tresor.unlock();
    tresor.status();
    tresor.withdraw(100, 100);
    tresor.status();
    let tresor = tresor.lock();
    tresor.status();
    let mut tresor = tresor.unlock();
    tresor.deposit(1000, 1000);
    tresor.status();
    let tresor = tresor.lock();
    tresor.status();
}
