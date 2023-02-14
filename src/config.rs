pub struct Config {
    pub maze_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments!")
        }

        let maze_path = args[1].clone();

        Ok(Config { maze_path })
    }
}
