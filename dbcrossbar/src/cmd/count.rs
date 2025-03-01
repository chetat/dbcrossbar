//! The `count` subcommand.

use anyhow::{format_err, Context as _, Result};
use dbcrossbarlib::{
    config::Configuration, Context, DriverArguments, SharedArguments, SourceArguments,
    TemporaryStorage, UnparsedLocator,
};
use structopt::{self, StructOpt};

/// Count arguments.
#[derive(Debug, StructOpt)]
pub(crate) struct Opt {
    /// The schema to use (defaults to input table schema).
    #[structopt(long = "schema")]
    schema: Option<UnparsedLocator>,

    /// Temporary directories, cloud storage buckets, datasets to use during
    /// transfer (can be repeated).
    #[structopt(long = "temporary")]
    temporaries: Vec<String>,

    /// Pass an extra argument of the form `key=value` to the source driver.
    #[structopt(long = "from-arg")]
    from_args: Vec<String>,

    /// SQL where clause specifying rows to use.
    #[structopt(long = "where")]
    where_clause: Option<String>,

    /// The locator specifying the records to count.
    locator: UnparsedLocator,
}

/// Count records.
pub(crate) async fn run(
    ctx: Context,
    config: Configuration,
    enable_unstable: bool,
    opt: Opt,
) -> Result<()> {
    let schema_opt = opt.schema.map(|s| s.parse(enable_unstable)).transpose()?;
    let locator = opt.locator.parse(enable_unstable)?;

    // Figure out what table schema to use.
    let schema = {
        let schema_locator = schema_opt.as_ref().unwrap_or(&locator);
        schema_locator
            .schema(ctx.clone())
            .await
            .with_context(|| format!("error reading schema from {}", schema_locator))?
            .ok_or_else(|| {
                format_err!("don't know how to read schema from {}", schema_locator)
            })
    }?;

    // Build our shared arguments. Specify 1 for `max_streams` until we actually
    // implement local counting.
    let temporaries = opt.temporaries.clone();
    let temporary_storage = TemporaryStorage::with_config(temporaries, &config)?;
    let shared_args = SharedArguments::new(schema, temporary_storage, 1);

    // Build our source arguments.
    let from_args = DriverArguments::from_cli_args(&opt.from_args)?;
    let source_args = SourceArguments::new(from_args, opt.where_clause.clone());

    let count = locator.count(ctx.clone(), shared_args, source_args).await?;
    println!("{}", count);
    Ok(())
}
