use std::io;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    println!("I wonder if they love me...");
    println!("How many petals are on this flower?");

    let mut petal_number = String::new();

    io::stdin()
        .read_line(&mut petal_number)
        .expect("Failed to read line");

    let petal_number: u32 = match petal_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You can't have '{petal_number}' petals on a flower!");
            0;
            process::exit(exitcode::DATAERR);
        }
    };

    let mut petal_state = true;
    let mut i = 0;

    while i < petal_number {
        if petal_state == true {
            println!("Loves me...");
            i = i + 1;
            petal_state = false;
            thread::sleep(Duration::from_millis(900));
        } else {
            println!("Loves me not...");
            i = i + 1;
            petal_state = true;
            thread::sleep(Duration::from_millis(900));
        }
    }

    if petal_state == false {
        thread::sleep(Duration::from_secs(3));
        println!("They love me!!!");
        thread::sleep(Duration::from_secs(1));
        println!(":D");
    } else {
        thread::sleep(Duration::from_secs(3));
        println!("They don't love me...");
        thread::sleep(Duration::from_secs(3));
        println!("Deleting System32 in 5...");
        thread::sleep(Duration::from_secs(1));
        println!("Deleting System32 in 4...");
        thread::sleep(Duration::from_secs(1));
        println!("3...");
        thread::sleep(Duration::from_secs(1));
        println!("2...");
        thread::sleep(Duration::from_secs(1));
        println!("1...");
        thread::sleep(Duration::from_secs(3));

        println!("Deleting System32 [#-----------------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [##----------------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [###---------------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [####--------------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#####-------------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [######------------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#######-----------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [########----------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#########---------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [##########--------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [###########-------------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [###############---------------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [####################----------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#####################---------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [######################--------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#######################-------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [########################------------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#########################-----------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [##########################----------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [###########################---------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [############################--------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#############################-------------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [################################----------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#################################---------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [##################################--------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [###################################-------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [####################################------------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#####################################-----------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [########################################--------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [########################################--------------------------------------]");
        thread::sleep(Duration::from_millis(1000));
        println!("Deleting System32 [##########################################------------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [###########################################-----------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#############################################---------------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [###################################################---------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [######################################################------------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [#########################################################---------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [##########################################################--------------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [###############################################################---------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [################################################################--------------]");
        thread::sleep(Duration::from_millis(10));
        println!("Deleting System32 [######################################################################--------]");
        thread::sleep(Duration::from_millis(1000));
        println!("Deleting System32 [########################################################################------]");
        thread::sleep(Duration::from_millis(1000));
        println!("Deleting System32 [###########################################################################---]");
        thread::sleep(Duration::from_millis(300));
        println!("Deleting System32 [##############################################################################]");
        thread::sleep(Duration::from_secs(2));
        println!("Gotcha!");
        thread::sleep(Duration::from_secs(2));
        println!("stupid little bitch >:)");
        thread::sleep(Duration::from_millis(300));
        std::process::exit(exitcode::OK);
    }
}
