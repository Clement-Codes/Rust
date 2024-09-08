// Struct: a data structure that allows you to 
// group multiple fields together under one name.
// Demonstration on one mutable reference or many
// immutable references
fn main(){
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(45.5);

    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}


impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}