use std::fs;
use std::path::Path;

use clap::{Parser, Subcommand};
use duct::cmd;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Clean,
    Ios {
        #[arg(long)]
        release: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Ios { release } => {
            ios_build(release)?;

            let library_paths = match release {
                true => (
                    "./target/aarch64-apple-ios/release/liblibellule.a",
                    "./target/aarch64-apple-ios-sim/release/liblibellule.a",
                ),
                false => (
                    "./target/aarch64-apple-ios/debug/liblibellule.a",
                    "./target/aarch64-apple-ios-sim/debug/liblibellule.a",
                ),
            };
            ios_bindings(library_paths)?;
            ios_xcframework(library_paths)?;
            ios_package()?;
        }
        Commands::Clean => clean()?,
    }

    Ok(())
}

fn ios_build(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let profile = match release {
        true => "release",
        false => "dev",
    };

    cmd!(
        "cargo",
        "build",
        "-p",
        "libellule-uniffi",
        "--profile",
        profile,
        "--target",
        "aarch64-apple-ios",
        "--target",
        "aarch64-apple-ios-sim",
    )
    .env("IPHONEOS_DEPLOYMENT_TARGET", "26.0")
    .run()?;

    Ok(())
}

fn ios_bindings(library_paths: (&str, &str)) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("./build/bindings")?;
    fs::create_dir_all("./build/headers")?;

    cmd!(
        "cargo",
        "run",
        "--bin",
        "uniffi-bindgen",
        "--release",
        "generate",
        "--library",
        library_paths.0,
        "--language",
        "swift",
        "--out-dir",
        "./build/bindings",
    )
    .run()?;

    fs::copy(
        "./build/bindings/libelluleFFI.h",
        "./build/headers/libelluleFFI.h",
    )?;

    fs::copy(
        "./build/bindings/libelluleFFI.modulemap",
        "./build/headers/module.modulemap",
    )?;

    Ok(())
}

fn ios_xcframework(library_paths: (&str, &str)) -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("./build/libellule.xcframework");

    cmd!(
        "xcodebuild",
        "-create-xcframework",
        "-library",
        library_paths.0,
        "-headers",
        "./build/headers",
        "-library",
        library_paths.1,
        "-headers",
        "./build/headers",
        "-output",
        "./build/libellule.xcframework",
    )
    .run()?;

    Ok(())
}

fn ios_package() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("./ios/LibelluleKit/libellule.xcframework");

    fs::create_dir_all("./ios/LibelluleKit/Sources/LibelluleKit")?;

    fs::rename(
        "./build/libellule.xcframework",
        "./ios/LibelluleKit/libellule.xcframework",
    )?;

    fs::copy(
        "./build/bindings/libellule.swift",
        "./ios/LibelluleKit/Sources/LibelluleKit/libellule.swift",
    )?;

    Ok(())
}

fn clean() -> Result<(), Box<dyn std::error::Error>> {
    cmd!("cargo", "clean").run()?;

    for path in [
        "./build",
        "./ios/LibelluleKit/.build",
        "./ios/LibelluleKit/libellule.xcframework",
        "./ios/LibelluleKit/Sources",
    ] {
        if Path::new(path).exists() {
            fs::remove_dir_all(path)?;
        }
    }

    Ok(())
}
