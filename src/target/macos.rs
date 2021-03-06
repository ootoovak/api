// Copyright 2015-2017 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use command::{CommandResult, CommandTarget};
use directory::DirectoryTarget;
use error::{Error, Result};
use file::{FileTarget, FileOwner};
use host::Host;
use host::telemetry::{Cpu, Os, Telemetry, TelemetryTarget};
use package::PackageTarget;
use package::providers::Providers;
use serde_json;
use service::ServiceTarget;
use std::{env, process, str};
use std::path::Path;
use super::{default_base as default, Target, unix_base as unix};

// This implementation is legacy. More work is required to support
// modern launchd implementations.
//
// const LD_PATHS: [&'static str; 2] = [
//     "/Libarary/LaunchDaemons",
//     "/System/Library/LaunchDaemons"
// ];

//
// Command
//

impl CommandTarget for Target {
    #[allow(unused_variables)]
    fn exec(host: &mut Host, cmd: &str) -> Result<CommandResult> {
        default::command_exec(cmd)
    }
}

//
// Directory
//

impl<P: AsRef<Path>> DirectoryTarget<P> for Target {
    #[allow(unused_variables)]
    fn directory_is_directory(host: &mut Host, path: P) -> Result<bool> {
        default::directory_is_directory(path)
    }

    #[allow(unused_variables)]
    fn directory_exists(host: &mut Host, path: P) -> Result<bool> {
        default::file_exists(path)
    }

    #[allow(unused_variables)]
    fn directory_create(host: &mut Host, path: P, recursive: bool) -> Result<()> {
        default::directory_create(path, recursive)
    }

    #[allow(unused_variables)]
    fn directory_delete(host: &mut Host, path: P, recursive: bool) -> Result<()> {
        default::directory_delete(path, recursive)
    }

    #[allow(unused_variables)]
    fn directory_mv(host: &mut Host, path: P, new_path: P) -> Result<()> {
        default::file_mv(path, new_path)
    }

    #[allow(unused_variables)]
    fn directory_get_owner(host: &mut Host, path: P) -> Result<FileOwner> {
        unix::file_get_owner(path)
    }

    #[allow(unused_variables)]
    fn directory_set_owner(host: &mut Host, path: P, user: &str, group: &str) -> Result<()> {
        default::file_set_owner(path, user, group)
    }

    #[allow(unused_variables)]
    fn directory_get_mode(host: &mut Host, path: P) -> Result<u16> {
        unix::file_get_mode(path)
    }

    #[allow(unused_variables)]
    fn directory_set_mode(host: &mut Host, path: P, mode: u16) -> Result<()> {
        default::file_set_mode(path, mode)
    }
}

//
// File
//

impl<P: AsRef<Path>> FileTarget<P> for Target {
    #[allow(unused_variables)]
    fn file_is_file(host: &mut Host, path: P) -> Result<bool> {
        default::file_is_file(path)
    }

    #[allow(unused_variables)]
    fn file_exists(host: &mut Host, path: P) -> Result<bool> {
        default::file_exists(path)
    }

    #[allow(unused_variables)]
    fn file_delete(host: &mut Host, path: P) -> Result<()> {
        default::file_delete(path)
    }

    #[allow(unused_variables)]
    fn file_mv(host: &mut Host, path: P, new_path: P) -> Result<()> {
        default::file_mv(path, new_path)
    }

    #[allow(unused_variables)]
    fn file_copy(host: &mut Host, path: P, new_path: P) -> Result<()> {
        default::file_copy(path, new_path)
    }

    #[allow(unused_variables)]
    fn file_get_owner(host: &mut Host, path: P) -> Result<FileOwner> {
        unix::file_get_owner(path)
    }

    #[allow(unused_variables)]
    fn file_set_owner(host: &mut Host, path: P, user: &str, group: &str) -> Result<()> {
        default::file_set_owner(path, user, group)
    }

    #[allow(unused_variables)]
    fn file_get_mode(host: &mut Host, path: P) -> Result<u16> {
        unix::file_get_mode(path)
    }

    #[allow(unused_variables)]
    fn file_set_mode(host: &mut Host, path: P, mode: u16) -> Result<()> {
        default::file_set_mode(path, mode)
    }
}

//
// Package
//

impl PackageTarget for Target {
    fn default_provider(host: &mut Host) -> Result<Providers> {
        default::default_provider(host, vec![Providers::Homebrew, Providers::Macports])
    }
}

//
// Service
//

impl ServiceTarget for Target {
    #[allow(unused_variables)]
    fn service_action(host: &mut Host, name: &str, action: &str) -> Result<Option<CommandResult>> {
        // This implementation is legacy. More work is required to
        // support modern launchd implementations.
        //
        // let action = match action {
        //     "start" => "load",
        //     "stop" => "unload",
        //     _ => action,
        // };
        //
        // // If name is relative path, search known LaunchDaemon paths
        // // for name's path.
        // let mut name = name;
        // let name_path = Path::new(name);
        // if name_path.is_relative() {
        //     for path in LD_PATHS.into_iter() {
        //         let mut buf = PathBuf::from(path);
        //
        //         if buf.is_dir() {
        //             buf.push(name);
        //
        //             if buf.is_file() {
        //                 name = buf.to_str().unwrap();
        //                 break;
        //             }
        //         }
        //     }
        // }
        //
        // command_exec(&format!("launchctl {} {}", action, name))
        unimplemented!()
    }
}

//
// Telemetry
//

impl TelemetryTarget for Target {
    #[allow(unused_variables)]
    fn telemetry_init(host: &mut Host) -> Result<serde_json::Value> {
        let cpu_vendor = try!(unix::get_sysctl_item("machdep\\.cpu\\.vendor"));
        let cpu_brand = try!(unix::get_sysctl_item("machdep\\.cpu\\.brand_string"));
        let hostname = try!(default::hostname());
        let (version_str, version_maj, version_min, version_patch) = try!(version());

        let telemetry = Telemetry::new(
            Cpu::new(
                &cpu_vendor,
                &cpu_brand,
                try!(try!(unix::get_sysctl_item("hw\\.physicalcpu")).parse::<u32>())
            ),
            try!(default::parse_fs(vec![
                default::FsFieldOrder::Filesystem,
                default::FsFieldOrder::Size,
                default::FsFieldOrder::Used,
                default::FsFieldOrder::Available,
                default::FsFieldOrder::Capacity,
                default::FsFieldOrder::Blank,
                default::FsFieldOrder::Blank,
                default::FsFieldOrder::Blank,
                default::FsFieldOrder::Mount,
            ])),
            &hostname,
            try!(try!(unix::get_sysctl_item("hw\\.memsize")).parse::<u64>()),
            default::net(),
            Os::new(env::consts::ARCH, "unix", "macos", &version_str, version_maj, version_min, version_patch),
        );

        Ok(serde_json::to_value(telemetry)?)
    }
}

fn version() -> Result<(String, u32, u32, u32)> {
    let out = process::Command::new("sw_vers").arg("-productVersion").output()?;
    let version_str = str::from_utf8(&out.stdout).or(Err(Error::Generic("Could not read OS version".into())))?.trim().to_owned();
    let (maj, min, patch) = {
        let mut parts = version_str.split('.');
        (parts.next().ok_or(Error::Generic(format!("Expected OS version format `u32.u32.u32`. Got: {}", version_str)))?.parse()?,
         parts.next().ok_or(Error::Generic(format!("Expected OS version format `u32.u32.u32`. Got: {}", version_str)))?.parse()?,
         parts.next().ok_or(Error::Generic(format!("Expected OS version format `u32.u32.u32`. Got: {}", version_str)))?.parse()?)
    };
    Ok((version_str, maj, min, patch))
}

#[cfg(test)]
mod tests {
    use Host;
    use package::PackageTarget;
    use target::Target;
    use host::telemetry::TelemetryTarget;

    #[test]
    fn test_package_default_provider() {
        let path: Option<String> = None;
        let mut host = Host::local(path).unwrap();
        let result = Target::default_provider(&mut host);
        assert!(result.is_ok());
    }

    #[test]
    fn test_telemetry_init() {
        let path: Option<String> = None;
        let mut host = Host::local(path).unwrap();
        let result = Target::telemetry_init(&mut host);
        assert!(result.is_ok());
    }
}
