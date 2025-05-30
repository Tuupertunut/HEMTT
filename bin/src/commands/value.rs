use std::fmt::Write as _;

use hemtt_common::version::Version;

use crate::{context::Context, error::Error, report::Report};

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
/// Print a value from the project
///
/// Print a value from the project, use `list` to see all available values
pub struct Command {
    #[clap(name = "name")]
    name: String,
}

#[allow(clippy::too_many_lines)]
/// Execute the script command
///
/// # Errors
/// [`Error`] depending on the modules
pub fn execute(cmd: &Command) -> Result<Report, Error> {
    let default = String::new();
    let ctx = Context::new(None, crate::context::PreservePrevious::Remove, false)?;
    match cmd.name.as_str() {
        "project.name" => {
            println!("{}", ctx.config().name());
        }
        "project.author" => {
            println!("{}", ctx.config().author().unwrap_or(&default));
        }
        "project.prefix" => {
            println!("{}", ctx.config().prefix());
        }
        "project.mainprefix" => {
            println!("{}", ctx.config().mainprefix().unwrap_or(&default));
        }
        "project.properties" => {
            println!("{}", {
                let mut props = String::new();
                for (key, value) in ctx.config().properties() {
                    writeln!(&mut props, "{key}: {value}").unwrap();
                }
                props
            });
        }
        "project.version" => {
            println!("{}", version(&ctx));
        }
        "project.version.short" => {
            let version = version(&ctx);
            println!(
                "{}.{}.{}",
                version.major(),
                version.minor(),
                version.patch()
            );
        }
        "project.version.major" => {
            println!("{}", version(&ctx).major());
        }
        "project.version.minor" => {
            println!("{}", version(&ctx).minor());
        }
        "project.version.patch" => {
            println!("{}", version(&ctx).patch());
        }
        "project.version.build" => {
            println!(
                "{}",
                version(&ctx)
                    .build()
                    .map(|b| b.to_string())
                    .unwrap_or_default()
            );
        }
        "project.version.path" => {
            println!("{}", ctx.config().version().path());
        }
        "project.version.git_hash" => {
            println!("{}", ctx.config().version().git_hash().unwrap_or(0));
        }
        "project.signing.authority" => {
            println!("{}", crate::modules::sign::get_authority(&ctx, None)?);
        }
        "project.signing.version" => {
            println!("{}", ctx.config().signing().version());
        }
        "project.files.include" => {
            println!("{}", ctx.config().files().include().join("\n"));
        }
        "project.files.exclude" => {
            println!("{}", ctx.config().files().exclude().join("\n"));
        }
        "list" => {
            println!("project.name");
            println!("project.prefix");
            println!("project.mainprefix");
            println!("project.properties");
            println!("project.version");
            println!("project.version.short");
            println!("project.version.major");
            println!("project.version.minor");
            println!("project.version.patch");
            println!("project.version.build");
            println!("project.version.path");
            println!("project.version.git_hash");
            println!("project.signing.authority");
            println!("project.signing.version");
            println!("project.files.include");
            println!("project.files.exclude");
        }
        _ => {
            std::process::exit(1);
        }
    }

    Ok(Report::new())
}

fn version(ctx: &Context) -> Version {
    ctx.config()
        .version()
        .get(ctx.workspace_path().vfs())
        .unwrap_or_else(|_| {
            println!("Unable to find version");
            std::process::exit(1);
        })
}
