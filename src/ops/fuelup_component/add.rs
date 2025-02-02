use std::str::FromStr;

use anyhow::{bail, Result};
use semver::Version;
use tracing::info;

use crate::{
    channel::PackageVersion, commands::component::AddCommand, download::DownloadCfg,
    target_triple::TargetTriple, toolchain::Toolchain,
};

pub fn add(command: AddCommand) -> Result<()> {
    let AddCommand {
        maybe_versioned_component,
    } = command;

    let toolchain = Toolchain::from_settings()?;
    if toolchain.is_official() {
        bail!(
            "Installing specific components is reserved for custom toolchains.
You are currently using '{}'.

You may create a custom toolchain using 'fuelup toolchain new <toolchain>'.",
            toolchain.name
        )
    };

    if toolchain.has_component(&maybe_versioned_component) {
        info!(
            "{} already exists in toolchain '{}'; replacing existing version with `latest` version",
            &maybe_versioned_component, toolchain.name
        );
    }

    let (component, version): (&str, Option<PackageVersion>) =
        match maybe_versioned_component.split_once('@') {
            Some(t) => {
                let v = match Version::from_str(t.1) {
                    Ok(v) => v,
                    Err(e) => bail!(
                        "Invalid version input '{}' while adding component: {}",
                        t.1,
                        e
                    ),
                };
                (
                    t.0,
                    Some(PackageVersion {
                        semver: v,
                        date: None,
                    }),
                )
            }
            None => (&maybe_versioned_component, None),
        };

    let download_cfg =
        DownloadCfg::new(component, TargetTriple::from_component(component)?, version)?;
    toolchain.add_component(download_cfg)?;

    Ok(())
}
