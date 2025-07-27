//use std::mem;

struct Cash {
    total: u32,
}

impl Cash {
    fn new(amount: u32) -> Self {
        println!("Cash loaded: ₹{}", amount);
        Cash { total: amount }
    }

    fn withdraw(&mut self, amount: u32) {
        println!(
            "Withdrawal attempt: ₹{} from address: {:p} (Heap)",
            amount, self
        );
        if self.total >= amount {
            self.total -= amount;
            println!("Success! Dispensed ₹{}. Remaining ₹{}", amount, self.total);
        } else {
            println!("Failed! Not enough cash. Available ₹{}", self.total);
        }
    }
}

// // RAII: Drop is automatically called when ATM shuts down
// impl Drop for Cash {
//     fn drop(&mut self) {
//         println!("ATM shutting down. ₹{} cash returned to vault.", self.total);
//     }
// }

struct ATM {
    cash: Box<Cash>, // Heap-allocated cash
}

impl ATM {
    fn new(initial_amount: u32) -> Self {
        let boxed_cash = Box::new(Cash::new(initial_amount)); // moves to heap
        println!("Heap address of Cash: {:p}", boxed_cash);
        ATM { cash: boxed_cash }
    }

    fn withdraw_from_atm(&mut self, amount: u32) {
        self.cash.withdraw(amount);
    }
}

fn main() {
    // ATM is stored on stack, but owns heap-allocated cash
    let mut atm = ATM::new(10_000);
    println!("Stack address of ATM: {:p}", &atm);

    atm.withdraw_from_atm(3000);
    atm.withdraw_from_atm(5000);
    atm.withdraw_from_atm(3000); // should fail

    // ATM goes out of scope here -> Drop for Cash is automatically called
}
