use std::io;
fn menu() {
    println!("1.Balance");
    println!("2.Withdraw");
    println!("3.Deposite");
    println!("4.Exit");
}

fn checkuser() -> bool {
    let hash = 1234;
    let mut pin = String::new();
    println!("Enter your PIN: ");
    io::stdin().read_line(&mut pin).expect("Err");
    let pin: u32 = pin.trim().parse().expect("Err");
    if pin == hash {
        return true;
    } else {
        return false;
    }

}
fn main() {
        let dash = "-".repeat(50);
        // balance 
        let mut balance = 1000;
        let result = checkuser();
        if result == true {
            loop {
                let mut choice = String::new();
                let mut deposite = String::new();
                menu();
                println!("Enter Your Choice");
                io::stdin().read_line(&mut choice).expect("Input failed");
                let choice:u32 = choice.trim().parse().expect("Convertion failed");
                if choice == 1 {
                    println!("{}",&dash[..]);
                    println!("User Balance {} 💲",balance);
                    println!("{}",&dash[..]);

                } else if choice == 2 {
                    let mut withdraw = String::new();
                    println!("Enter the amount to withdraw: ");
                    io::stdin().read_line(&mut withdraw).expect("amount not get");
                    let withdraw :u32 = withdraw.trim().parse().expect("Invalid convert");
                    if withdraw > balance {
                        println!("You don't have sufficient balance");
                    } else {
                        let balance = balance - withdraw;
                        println!("{}",&dash[..]);
                        println!("Amount Successfully withdraw");
                        println!("Balance is {}",balance);
                        println!("{}",&dash[..]);

                    }
                }
                
                else if choice == 3 {
                    println!("{}",&dash[..]);
                    println!("Enter the amount to deposite");
                    io::stdin().read_line(&mut deposite).expect("Input failed");
                    let deposite:u32 = deposite.trim().parse().expect("convertion failed");
                    if deposite > 0 {
                        println!("{}",&dash[..]);
                        balance = balance + deposite;
                        println!("User Deposited {} \n balance is {}",deposite,balance);
                    }
                    println!("{}",&dash[..]);
                }
                else if choice == 4 {
                    println!("{}",&dash[..]);
                    println!("Thankyou 🎉");
                    break;
                } else {
                    println!("Invalid Option");
                }
            }
        } else {
            println!("Invalid Pin Try Again");
    }
}