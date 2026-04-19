use anyhow::{Context, bail};
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::application::admin_users::{CreateAdminInput, create_admin};

pub enum Command {
    Serve,
    CreateAdmin(CreateAdminArgs),
    CreateAdminHelp,
    Help,
}

pub struct CreateAdminArgs {
    pub email: String,
    pub nickname: String,
    pub password: String,
}

pub fn parse_args() -> anyhow::Result<Command> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        None => Ok(Command::Serve),
        Some("create-admin") => parse_create_admin_args(args),
        Some("--help") | Some("-h") => {
            print_help();
            Ok(Command::Help)
        }
        Some(other) => bail!("unknown command: {other}"),
    }
}

pub async fn run_create_admin(
    db: &DatabaseConnection,
    args: CreateAdminArgs,
) -> anyhow::Result<()> {
    let admin = create_admin(
        db,
        CreateAdminInput {
            email: args.email,
            nickname: args.nickname,
            password: args.password,
            is_active: true,
        },
    )
    .await?;

    info!(id = %admin.id, email = %admin.email, "admin user created");
    println!("Admin created: {} ({})", admin.nickname, admin.email);

    Ok(())
}

fn parse_create_admin_args(mut args: impl Iterator<Item = String>) -> anyhow::Result<Command> {
    let mut email = None;
    let mut nickname = None;
    let mut password = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--email" => email = Some(next_value(&mut args, "--email")?),
            "--nickname" => nickname = Some(next_value(&mut args, "--nickname")?),
            "--password" => password = Some(next_value(&mut args, "--password")?),
            "--help" | "-h" => {
                print_create_admin_help();
                return Ok(Command::CreateAdminHelp);
            }
            other => bail!("unknown argument for create-admin: {other}"),
        }
    }

    if email.is_none() && nickname.is_none() && password.is_none() {
        bail!("missing required arguments for `create-admin`");
    }

    Ok(Command::CreateAdmin(CreateAdminArgs {
        email: email.context("missing required argument `--email`")?,
        nickname: nickname.context("missing required argument `--nickname`")?,
        password: password.context("missing required argument `--password`")?,
    }))
}

fn next_value(args: &mut impl Iterator<Item = String>, flag: &str) -> anyhow::Result<String> {
    args.next()
        .with_context(|| format!("missing value for `{flag}`"))
}

fn print_help() {
    eprintln!("Usage:");
    eprintln!("  cargo run");
    eprintln!(
        "  cargo run -- create-admin --email <email> --nickname <nickname> --password <password>"
    );
}

fn print_create_admin_help() {
    eprintln!("Usage:");
    eprintln!(
        "  cargo run -- create-admin --email <email> --nickname <nickname> --password <password>"
    );
}
