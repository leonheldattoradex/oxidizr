use clap::{Arg, ArgAction, Command};

mod sqlstorage;
mod types;

use sqlstorage::SQLStorage;
use rusqlite::Result;

fn main() -> Result<()> {
    let _matches = Command::new("aktualizr-info")
        .version("0.0.1")
        .author("Leonardo Held <leonardo.held@toradex.com>")
        .about("aktualizr-info command line options")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .action(ArgAction::Set)
                .value_name("FILE")
                .help("Configuration file or directory")
                .num_args(1..),
        )
        .arg(
            Arg::new("loglevel")
                .long("loglevel")
                .action(ArgAction::Set)
                .value_name("LEVEL")
                .help("Set log level 0-5 (trace, debug, info, warning, error, fatal)"),
        )
        .arg(
            Arg::new("name-only")
                .long("name-only")
                .action(ArgAction::SetTrue)
                .help("Only output device name (intended for scripting). Cannot be used in combination with other arguments."),
        )
        .arg(
            Arg::new("tls-creds")
                .long("tls-creds")
                .action(ArgAction::SetTrue)
                .help("Outputs TLS credentials"),
        )
        .arg(
            Arg::new("tls-root-ca")
                .long("tls-root-ca")
                .action(ArgAction::SetTrue)
                .help("Outputs TLS Root CA"),
        )
        .arg(
            Arg::new("tls-cert")
                .long("tls-cert")
                .action(ArgAction::SetTrue)
                .help("Outputs TLS client certificate"),
        )
        .arg(
            Arg::new("tls-prv-key")
                .long("tls-prv-key")
                .action(ArgAction::SetTrue)
                .help("Output TLS client private key"),
        )
        .arg(
            Arg::new("ecu-keys")
                .long("ecu-keys")
                .action(ArgAction::SetTrue)
                .help("Outputs Primary's Uptane keys"),
        )
        .arg(
            Arg::new("ecu-keyid")
                .long("ecu-keyid")
                .action(ArgAction::SetTrue)
                .help("Outputs Primary's Uptane public key ID"),
        )
        .arg(
            Arg::new("ecu-pub-key")
                .long("ecu-pub-key")
                .action(ArgAction::SetTrue)
                .help("Outputs Primary's Uptane public key"),
        )
        .arg(
            Arg::new("ecu-prv-key")
                .long("ecu-prv-key")
                .action(ArgAction::SetTrue)
                .help("Outputs Primary's Uptane private key"),
        )
        .arg(
            Arg::new("secondary-keys")
                .long("secondary-keys")
                .action(ArgAction::SetTrue)
                .help("Outputs Secondaries' Uptane public keys"),
        )
        .arg(
            Arg::new("image-root")
                .long("image-root")
                .action(ArgAction::SetTrue)
                .help("Outputs root.json from Image repo, by default the latest"),
        )
        .arg(
            Arg::new("image-timestamp")
                .long("image-timestamp")
                .action(ArgAction::SetTrue)
                .help("Outputs timestamp.json from Image repo"),
        )
        .arg(
            Arg::new("image-snapshot")
                .long("image-snapshot")
                .action(ArgAction::SetTrue)
                .help("Outputs snapshot.json from Image repo"),
        )
        .arg(
            Arg::new("image-targets")
                .long("image-targets")
                .action(ArgAction::SetTrue)
                .help("Outputs targets.json from Image repo"),
        )
        .arg(
            Arg::new("delegation")
                .long("delegation")
                .action(ArgAction::SetTrue)
                .help("Outputs metadata of Image repo Targets' delegations"),
        )
        .arg(
            Arg::new("director-root")
                .long("director-root")
                .action(ArgAction::SetTrue)
                .help("Outputs root.json from Director repo, by default the latest"),
        )
        .arg(
            Arg::new("director-targets")
                .long("director-targets")
                .action(ArgAction::SetTrue)
                .help("Outputs targets.json from Director repo"),
        )
        .arg(
            Arg::new("root-version")
                .long("root-version")
                .action(ArgAction::Set)
                .value_name("VERSION")
                .help("Use with --image-root or --director-root to specify the version to output"),
        )
        .arg(
            Arg::new("allow-migrate")
                .long("allow-migrate")
                .action(ArgAction::SetTrue)
                .help("Opens database in read/write mode to make possible to migrate database if needed"),
        )
        .arg(
            Arg::new("wait-until-provisioned")
                .long("wait-until-provisioned")
                .action(ArgAction::SetTrue)
                .help("Outputs metadata when device already provisioned"),
        )

        .arg(
            Arg::new("images-root")
                .long("images-root")
                .action(ArgAction::SetTrue)
                .help("Outputs root.json from Image repo")
                .hide(true),
        )
        .arg(
            Arg::new("images-timestamp")
                .long("images-timestamp")
                .action(ArgAction::SetTrue)
                .help("Outputs timestamp.json from Image repo")
                .hide(true),
        )
        .arg(
            Arg::new("images-snapshot")
                .long("images-snapshot")
                .action(ArgAction::SetTrue)
                .help("Outputs snapshot.json from Image repo")
                .hide(true),
        )
        .arg(
            Arg::new("images-target")
                .long("images-target")
                .action(ArgAction::SetTrue)
                .help("Outputs targets.json from Image repo")
                .hide(true),
        )
        .arg(
            Arg::new("images-targets")
                .long("images-targets")
                .action(ArgAction::SetTrue)
                .help("Outputs targets.json from Image repo")
                .hide(true),
        )
        .arg(
            Arg::new("image-target")
                .long("image-target")
                .action(ArgAction::SetTrue)
                .help("Outputs targets.json from Image repo")
                .hide(true),
        )
        .arg(
            Arg::new("director-target")
                .long("director-target")
                .action(ArgAction::SetTrue)
                .help("Outputs targets.json from Director repo")
                .hide(true),
        )
        .get_matches();

    let storage = SQLStorage::new("sql.db")?;

    // Print general information if user does not provide any argument.
    match storage.load_device_id()? {
        Some(device_id) => {
            println!("Device ID: {}", device_id);
        }
        None => {
            println!("Device ID not found");
        }
    }

    let ecus = storage.load_ecus()?;
    let mut secondaries = Vec::new();

    for ecu in ecus {
        if ecu.is_primary {
            println!("Primary ECU serial ID: {}", ecu.serial);
            println!("Primary ECU hardware ID: {}", ecu.hardware_id);
        } else {
            secondaries.push(ecu);
        }
    }

    if !secondaries.is_empty() {
        println!("Secondaries:");
        for (index, secondary) in secondaries.iter().enumerate() {
            println!("{}) ID: {}, serial ID: {}", index + 1, secondary.id, secondary.serial);
            println!("   hardware ID: {}", secondary.hardware_id);
            println!("   no details about installed nor pending images");
        }
    }

    Ok(())

}
