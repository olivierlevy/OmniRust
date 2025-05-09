// src/cli/arg_parser.rs

use clap::{Parser, Subcommand, Args};

/// OmniRust Command-Line Interface
/// 
/// A versatile toolkit built with the OmniRust framework, offering a range of
/// functionalities from simple utilities to complex operations.
/// Use `omnirust --help` for a brief overview or `omnirust <SUBCOMMAND> --help` for
/// detailed help on a specific subcommand.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct CliArgs {
    /// An optional name parameter for general use, its effect depends on the subcommand.
    #[arg(short, long, value_name = "NAME", global = true)]
    pub name: Option<String>,

    /// Increases verbosity of debugging information.
    /// Can be used multiple times (e.g., -d, -dd, -ddd).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manages and executes test-related operations within the OmniRust framework.
    /// Useful for developers and for verifying framework components.
    #[command(visible_alias = "tst")]
    Test(TestArgs),

    /// Provides access to various utility functions built into OmniRust.
    /// Includes string manipulation, file operations, etc.
    #[command(visible_alias = "utils")]
    Util(UtilArgs),
    
    /// Launches a simple TUI counter application.
    #[command(visible_alias = "tui")]
    TuiCounter,
    // Add more subcommands for different components like 'server', 'db', 'ml', etc.
    // Example:
    // /// Starts one of the OmniRust servers (e.g., GraphQL, REST, WebSocket).
    // Server(ServerArgs),
}

#[derive(Args, Debug)]
#[command(about = "Manages test operations.", long_about = "Use this subcommand to list available tests or run specific test cases.")]
pub struct TestArgs {
    /// Lists all available test categories or specific test values if applicable.
    #[arg(short, long, help = "Display a list of available test items.")]
    pub list: bool,

    /// Specifies a particular test case or scenario to execute.
    /// The exact format depends on the test harness.
    #[arg(value_name = "TEST_CASE", help = "The name or ID of the test case to run.")]
    pub case: Option<String>,
}

#[derive(Args, Debug)]
#[command(about = "Access utility functions.", long_about = "Provides a collection of general-purpose utility commands for common tasks.")]
pub struct UtilArgs {
    #[command(subcommand)]
    pub command: UtilCommands,
}

#[derive(Subcommand, Debug)]
pub enum UtilCommands {
    /// Reverses the characters in a given string.
    /// Example: omnirust util reverse "hello" -> "olleh"
    Reverse {
        /// The input string that will be reversed.
        #[arg(required = true, help = "The string to be reversed.")]
        input_string: String,
    },
    /// Checks if the given string is empty or consists only of whitespace.
    /// Example: omnirust util is-blank "  " -> true
    #[command(visible_alias = "blank")]
    IsBlank {
        /// The input string to check for blankness.
        #[arg(required = true, help = "The string to check if it's blank.")]
        input_string: String,
    },
    // Future utility subcommands:
    // /// Performs file operations.
    // File(FileUtilArgs),
    // /// Performs date/time operations.
    // DateTime(DateTimeUtilArgs),
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
