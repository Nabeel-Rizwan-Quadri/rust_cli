use clap::{
    Args,
    Parser,
    Subcommand,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Mycli {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Manage mycli
    Mycli(MycliCommand),
}

#[derive(Debug, Args)]
pub struct MycliCommand {
    #[clap(subcommand)]
    pub command: MycliSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum MycliSubcommand {
    Server,
    Client(CreateClient),
    /// Create a new mycli user
    Create(CreateUser),
}

#[derive(Debug, Args)]
pub struct CreateClient {
    /// Address of the server to connect to
    pub arraydata: Vec<String>,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// Username for the new mycli user
    pub username: String,
    /// Email for the new mycli user
    pub email: String,
}
