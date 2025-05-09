// src/cli/arg_parser.rs

use clap::{Parser, Subcommand, Args};

/// OmniRust Command-Line Interface
/// A versatile toolkit for various operations.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Optional name to operate on
    #[arg(short, long, value_name = "NAME")]
    pub name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manages test operations
    Test(TestArgs),
    /// Utility commands
    Util(UtilArgs),
    // Add more subcommands for different components like 'server', 'db', 'ml', etc.
}

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Lists test values
    #[arg(short, long)]
    pub list: bool,

    /// Specific test case to run
    #[arg(value_name = "TEST_CASE")]
    pub case: Option<String>,
}

#[derive(Args, Debug)]
pub struct UtilArgs {
    #[command(subcommand)]
    pub command: UtilCommands,
}

#[derive(Subcommand, Debug)]
pub enum UtilCommands {
    /// Reverses a string
    Reverse {
        /// The string to reverse
        #[arg(required = true)]
        input_string: String,
    },
    /// Checks if a string is blank
    IsBlank {
        /// The string to check
        #[arg(required = true)]
        input_string: String,
    },
    // Could add file utils, datetime utils etc. as subcommands here
}

/// Parses command line arguments using clap.
pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

// Example of how this might be used in a main.rs or a CLI-specific binary:
// fn main() {
//     let cli = parse_args();
//
//     // You can see how many times a particular flag or argument occurred
//     // Note, only flags can have multiple occurrences
//     match cli.debug {
//         0 => println!("Debug mode is off"),
//         1 => println!("Debug mode is kind of on"),
//         2 => println!("Debug mode is on"),
//         _ => println!("Don't be crazy"),
//     }
//
//     if let Some(name) = cli.name.as_deref() {
//         println!("Value for name: {}", name);
//     }
//
//     match &cli.command {
//         Some(Commands::Test(test_args)) => {
//             if test_args.list {
//                 println!("Listing all tests...");
//             } else if let Some(case) = &test_args.case {
//                 println!("Running test case: {}...", case);
//             } else {
//                 println!("Running all tests...");
//             }
//         }
//         Some(Commands::Util(util_args)) => {
//             match &util_args.command {
//                 UtilCommands::Reverse { input_string } => {
//                     // Assuming omnirust::utils::string_utils is available
//                     // let reversed = omnirust::utils::string_utils::reverse(input_string);
//                     // println!("Reversed string: {}", reversed);
//                     println!("Input to reverse: {}", input_string); // Placeholder
//                 }
//                 UtilCommands::IsBlank { input_string } => {
//                     // let is_blank = omnirust::utils::string_utils::is_blank(input_string);
//                     // println!("Is blank: {}", is_blank);
//                     println!("Input to check for blank: {}", input_string); // Placeholder
//                 }
//             }
//         }
//         None => {
//             println!("No subcommand was used. Use --help for more info.");
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_basic() {
        // Simulate command line: "my_app --name TestApp -d"
        let cli = CliArgs::try_parse_from(&["my_app", "--name", "TestApp", "-d"]).unwrap();
        assert_eq!(cli.name, Some("TestApp".to_string()));
        assert_eq!(cli.debug, 1);
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_parse_test_subcommand() {
        // Simulate: "my_app test --list"
        let cli = CliArgs::try_parse_from(&["my_app", "test", "--list"]).unwrap();
        match cli.command {
            Some(Commands::Test(test_args)) => {
                assert!(test_args.list);
                assert!(test_args.case.is_none());
            }
            _ => panic!("Expected Test subcommand"),
        }
    }
    
    #[test]
    fn test_parse_util_reverse_subcommand() {
        // Simulate: "my_app util reverse 'hello world'"
        let cli = CliArgs::try_parse_from(&["my_app", "util", "reverse", "hello world"]).unwrap();
        match cli.command {
            Some(Commands::Util(util_args)) => {
                match util_args.command {
                    UtilCommands::Reverse { input_string } => {
                        assert_eq!(input_string, "hello world");
                    }
                    _ => panic!("Expected Util Reverse subcommand"),
                }
            }
            _ => panic!("Expected Util subcommand"),
        }
    }

    #[test]
    fn test_help_message_generation() {
        // This doesn't run the app, just checks if help can be generated.
        // Useful for catching basic clap config errors.
        use clap::CommandFactory;
        CliArgs::command().debug_assert(); // Checks for common misconfigurations
    }
}
