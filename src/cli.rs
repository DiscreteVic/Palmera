    use std::env;
    use std::path::Path;
    use std::process;

    pub fn print_initial_prompt()
    {
    println!(" _____      _                          \n|  __ \\    | |                         \n| |__) |_ _| |_ __ ___   ___ _ __ __ _ \n|  ___/ _` | | '_ ` _ \\ / _ \\ '__/ _` |\n| |  | (_| | | | | | | |  __/ | | (_| |\n|_|   \\__,_|_|_| |_| |_|\\___|_|  \\__,_|\n");
    }

    pub struct UserArgs
    {
        pub ifile: String,
        pub ofile: String,
    }

    impl UserArgs
    {
        pub fn new() -> UserArgs
        {
            UserArgs {
                ifile: "".to_string(),
                ofile: "".to_string(),
            }
        }
        pub fn parse(&mut self, args: env::ArgsOs)
        {
            if args.len() <= 1
            {
                println!("ERR: Missing arguments!");
                process::exit(1);
            }

            let mut args = args.into_iter();
            while let Some(arg) = args.next()
            {
                match arg.to_str().unwrap() 
                {
                    "-i" => self.ifile = args.next().unwrap().into_string().unwrap(),
                    "-o" => self.ofile = args.next().unwrap().into_string().unwrap(),
                    _ => {}
                }
            }
        }

        pub fn checks(&self)
        {
            let in_file_path = Path::new(&self.ifile);

            if !in_file_path.exists()
            {
                println!("ERR: Input file not found!");
                process::exit(1);
            }
        }
    }

