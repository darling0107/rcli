use clap::Parser;
use rcli::{Opts, SubCommand, process_csv, process_genpass};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // println!("{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }

        SubCommand::GenPass(opts) => {
            println!("Generate password: {:?}", opts);
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
                opts.verify,
            )?;
        }
    }

    Ok(())
}
