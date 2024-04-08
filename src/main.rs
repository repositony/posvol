//! Command line tool to inspect and convert posvol files
#![doc(hidden)]

// standard library
use std::path::PathBuf;

// internal modules
mod cli;
use cli::Cli;

// ntools modules
use ntools::format::f;
use ntools::posvol::{self, Posvol};

// external crates
use anyhow::Result;
use clap::Parser;
use log::{debug, info};

fn main() -> Result<()> {
    // set up the command line interface and logging
    let cli = Cli::parse();
    init_logging(&cli);

    // Try to read the posvol binary
    info!("Reading {}", &cli.file);
    let posvol = posvol::read_posvol_file(&cli.file)?;

    // Log a summary of the file parameters to the terminal for reference
    if !cli.quiet {
        print_summary(&posvol);
    }

    // set an output path
    let mut path = PathBuf::from(&cli.output);

    // Log a summary of the file parameters to the terminal for reference
    if cli.ascii {
        path.set_extension("txt");
        debug!("Writing {}", path.display());
        match &cli.pretty {
            true => posvol::write_ascii_pretty(&posvol, &path)?,
            false => posvol::write_ascii(&posvol, &path)?,
        };
    }

    if cli.json {
        path.set_extension("json");
        debug!("Writing {}", path.display());
        posvol::write_json(&posvol, &path)?;
    }

    Ok(())
}

/// Write summary to the terminal
fn print_summary(posvol: &Posvol) {
    let mut s = "Summary of binary file\n".to_string();
    s += &f!(
        "voxels  : {} ({}x{}x{})\n",
        posvol.number_of_voxels(),
        posvol.dimensions.n_x - 1,
        posvol.dimensions.n_y - 1,
        posvol.dimensions.n_z - 1
    );
    s += &f!(
        "samples : {} ({}x{}x{})\n",
        posvol.number_of_subvoxels(),
        posvol.dimensions.res_x,
        posvol.dimensions.res_y,
        posvol.dimensions.res_z
    );
    s += &f!(
        "cells   : {} ({}x{})",
        posvol.number_of_cells(),
        posvol.dimensions.number_of_voxels(),
        posvol.dimensions.number_of_subvoxels(),
    );
    println!("{s}")
}

/// Sets up logging at runtime to allow for multiple verbosity levels
fn init_logging(cli: &Cli) {
    stderrlog::new()
        .module(module_path!())
        .quiet(cli.quiet)
        .verbosity(cli.verbose as usize + 2)
        .show_level(false)
        .color(stderrlog::ColorChoice::Never)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .unwrap();
}
