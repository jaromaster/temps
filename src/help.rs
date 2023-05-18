pub mod help {

    /// prints the help page (syntax, basic examples)
    pub fn print_help() {
        println!("---Usage---");
        println!("---syntax---");
        println!("temps [mode] [time]\n");

        println!("---timer---");
        println!("temps timer 2h10m30s");
        println!("temps timer 5m\n");

        println!("---alarm---");
        println!("temps alarm 21:15:00");
        println!("temps alarm 8:45:30\n");

        println!("---delayed commands---");
        println!("temps timer 1h && echo \"Hello world\"");
    }
}