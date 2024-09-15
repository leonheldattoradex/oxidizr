use clap::{Arg, ArgAction, Command};
use env_logger::Env;
use log::debug;
use rusqlite::Result;
use sqlstorage::SQLStorage;

use std::thread::sleep;
use std::time::Duration;

mod crypto;
mod ecu_serial;
mod hardware_identifier;
mod public_key;
mod secondary_info;
mod sqlstorage;
mod tuf_repository_type;
mod tuf_roles;
mod tuf_version;
mod types;

fn main() -> Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    let matches = Command::new("aktualizr-info")
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
                .help("Use with --image-root or --director-root to specify the version to output")
                .value_parser(clap::value_parser!(i32)),
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
                .help("Busy wait until the device is registered and has metadata, ie, is provisioned"),
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

    let mut print_default_information = true;

    let allow_migrate = matches.get_flag("allow-migrate");
    let wait_until_provisioned = matches.get_flag("wait-until-provisioned");
    let mut storage = SQLStorage::new("sql.db", allow_migrate)?;

    if wait_until_provisioned {
        let mut registered = false;
        let mut has_metadata = false;
        while !registered || !has_metadata {
            match SQLStorage::new("sql.db", allow_migrate) {
                Ok(new_storage) => {
                    storage = new_storage;
                    registered = storage.load_ecu_registered()?;
                    has_metadata = storage.load_director_root()?.is_some();
                }
                Err(e) => {
                    log::debug!("Storage exception: {}", e);
                }
            }

            // Busy wait
            sleep(Duration::from_secs(1));
        }
    }
    let tls_required = matches.get_flag("tls-creds")
        || matches.get_flag("tls-root-ca")
        || matches.get_flag("tls-cert")
        || matches.get_flag("tls-prv-key");

    if tls_required {
        print_default_information = false;

        let mut ca: Vec<u8> = Vec::new();
        let mut cert: Vec<u8> = Vec::new();
        let mut pkey: Vec<u8> = Vec::new();

        let tls_loaded = storage.load_tls_credentials(&mut ca, &mut cert, &mut pkey)?;

        if tls_loaded {
            if matches.get_flag("tls-creds") {
                let ca_str = String::from_utf8(ca.clone())
                    .unwrap_or_else(|_| "Invalid UTF-8 in CA certificate".to_string());
                let cert_str = String::from_utf8(cert.clone())
                    .unwrap_or_else(|_| "Invalid UTF-8 in Client certificate".to_string());
                let pkey_str = String::from_utf8(pkey.clone())
                    .unwrap_or_else(|_| "Invalid UTF-8 in Client private key".to_string());

                println!("Root CA Certificate:");
                println!("{}", ca_str);
                println!("Client Certificate:");
                println!("{}", cert_str);
                println!("Client Private Key:");
                println!("{}", pkey_str);
            } else {
                if matches.get_flag("tls-root-ca") {
                    let ca_str = String::from_utf8(ca.clone())
                        .unwrap_or_else(|_| "Invalid UTF-8 in CA certificate".to_string());
                    println!("CA Certificate:");
                    println!("{}", ca_str);
                }

                if matches.get_flag("tls-cert") {
                    let cert_str = String::from_utf8(cert.clone())
                        .unwrap_or_else(|_| "Invalid UTF-8 in Client certificate".to_string());
                    println!("Client Certificate:");
                    println!("{}", cert_str);
                }

                if matches.get_flag("tls-prv-key") {
                    let pkey_str = String::from_utf8(pkey.clone())
                        .unwrap_or_else(|_| "Invalid UTF-8 in Client private key".to_string());
                    println!("Client Private Key:");
                    println!("{}", pkey_str);
                }
            }
        } else {
            println!("TLS credentials not found or are incomplete.");
        }
    }

    if matches.get_flag("ecu-keys")
        || matches.get_flag("ecu-keyid")
        || matches.get_flag("ecu-pub-key")
        || matches.get_flag("ecu-prv-key")
    {
        print_default_information = false;

        if let Some((pubkey, privkey)) = storage.load_primary_keys()? {
            if matches.get_flag("ecu-keys") {
                println!("Public Key:");
                println!("{}", pubkey);
                println!("Private Key:");
                println!("{}", privkey);
            }

            if matches.get_flag("ecu-keyid") {
                println!("Public Key ID: {}", pubkey.key_id());
            }

            if matches.get_flag("ecu-pub-key") {
                println!("Public Key:");
                println!("{}", pubkey);
            }

            if matches.get_flag("ecu-prv-key") {
                println!("Private Key:");
                println!("{}", privkey);
            }
        } else {
            println!("Failed to load primary keys or keys are empty.");
        }
    }
    if matches.get_flag("name-only") {
        print_default_information = false;
        match storage.load_device_id()? {
            Some(device_id) => {
                println!("{}", device_id);
            }
            None => {
                println!("Device ID not found");
            }
        }
    }

    let mut secondaries = Vec::new();
    if matches.get_flag("secondary-keys") {
        print_default_information = false;
        if storage.load_secondaries_info(&mut secondaries)? {
            debug!("Secondaries loaded successfully:");
            for secondary in secondaries {
                println!("{}", secondary);
            }
        } else {
            println!("No secondary info found.");
        }
    }
    if matches.get_flag("image-root") || matches.get_flag("images-root") {
        print_default_information = false;
        // Get the root version, if provided
        let version = matches.get_one::<i32>("root-version").copied();

        match storage.load_image_root_with_version(version)? {
            Some(root_metadata) => println!("{}", root_metadata),
            None => println!("Failed to load image root metadata."),
        }
    }

    if matches.get_flag("director-root") {
        print_default_information = false;
        // Get the root version, if provided
        let version = matches.get_one::<i32>("root-version").copied();

        match storage.load_director_root_with_version(version)? {
            Some(root_metadata) => println!("{}", root_metadata),
            None => println!("Failed to load director root metadata."),
        }
    }

    if matches.get_flag("director-targets") || matches.get_flag("director-target") {
        match storage.load_director_targets()? {
            Some(director_targets) => println!("{}", director_targets),
            None => println!("Director targets metadata not found."),
        }
    }

    if matches.get_flag("image-snapshot") || matches.get_flag("images-snapshot") {
        match storage.load_image_snapshot()? {
            Some(snapshot) => println!("{}", snapshot),
            None => println!("Image snapshot metadata not found."),
        }
    }

    if matches.get_flag("image-timestamp") || matches.get_flag("images-timestamp") {
        match storage.load_image_timestamp()? {
            Some(timestamp) => println!("{}", timestamp),
            None => println!("Image timestamp metadata not found."),
        }
    }

    if matches.get_flag("image-targets")
        || matches.get_flag("image-target")
        || matches.get_flag("images-targets")
        || matches.get_flag("images-target")
    {
        match storage.load_image_targets()? {
            Some(images_targets) => println!("{}", images_targets),
            None => println!("Image targets metadata not found."),
        }
    }

    // Print general information if user does not provide any argument.
    if print_default_information {
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
                println!(
                    "{}) ID: {}, serial ID: {}",
                    index + 1,
                    secondary.id,
                    secondary.serial
                );
                println!("   hardware ID: {}", secondary.hardware_id);
                println!("   no details about installed nor pending images");
            }
        }
    }

    Ok(())
}
