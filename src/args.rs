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
    /// Manage users
    User(UserCommand),
    /// Manage projects
    Project(ProjectCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    Run,
    /// Create a new user
    Create(CreateUser),
    /// Delete an existing user
    Delete(DeleteUser),
    /// List all users
    List,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// Username for the new user
    pub username: String,
    /// Email for the new user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    /// Username of the user to delete
    pub username: String,
}

#[derive(Debug, Args)]
pub struct ProjectCommand {
    #[clap(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubcommand {
    /// Create a new project
    Create(CreateProject),
    /// Delete an existing project
    Delete(DeleteProject),
}

#[derive(Debug, Args)]
pub struct CreateProject {
    /// Name for the new project
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DeleteProject {
    /// Name of the project to delete
    pub name: String,
}