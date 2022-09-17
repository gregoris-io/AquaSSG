/** Implementation of the CLI tools
 * 
 */


use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]

//Macro 
#[clap(author, version, about)]

pub struct AquaSST{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]

// Help system menu
pub enum EntityType {
    /// Create your website - test 0.01
    Input(InputCommand),
    /// Finnished file location - test 0.01
    Output(OutputCommand),
}

#[derive(Debug, Args)]

pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

pub enum UserSubCommand {
    /// Create your website ->
    Input(ConvertFile),
    /// Determine finished file location ->
    Output(FileLocation),
}

pub struct ConvertFile {
    /// The name of the file
    pub file_name: String,
}

pub struct FileLocation {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    path_two: String,
}

