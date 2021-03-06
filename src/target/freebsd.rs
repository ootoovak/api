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
use regex::Regex;
use serde_json;
use service::ServiceTarget;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use super::{default_base as default, Target, unix_base as unix};

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
        default::default_provider(host, vec![Providers::Pkg, Providers::Ports])
    }
}

//
// Service
//

impl ServiceTarget for Target {
    #[allow(unused_variables)]
    fn service_action(host: &mut Host, name: &str, action: &str) -> Result<Option<CommandResult>> {
        let mut rc_conf = try!(OpenOptions::new().read(true).write(true).open("/etc/rc.conf"));
        let mut rc = String::new();
        try!(rc_conf.read_to_string(&mut rc));

        let match_daemon = Regex::new(&format!("(?m)^\\s*{}_enable\\s*=\\s*[\"']{{0,1}}(?:YES|yes)[\"']{{0,1}}\n?", name)).unwrap();

        match action {
            "enable" => {
                if ! match_daemon.is_match(&rc) {
                    let newline = if rc.ends_with("\n") { "" } else { "\n" };
                    try!(rc_conf.write_all(&format!("{}{}_enable=\"YES\"\n", newline, name).into_bytes()));
                    try!(rc_conf.sync_data());

                    Ok(Some(CommandResult {
                        exit_code: 0,
                        stdout: String::new(),
                        stderr: String::new(),
                    }))
                } else {
                    Ok(None)
                }
            },
            "disable" => {
                if match_daemon.is_match(&rc) {
                    let replace = match_daemon.replace(&rc, "").trim().to_string();
                    try!(rc_conf.seek(SeekFrom::Start(0)));
                    try!(rc_conf.set_len(replace.len() as u64));
                    try!(rc_conf.write_all(replace.as_bytes()));
                    try!(rc_conf.sync_data());

                    Ok(Some(CommandResult {
                        exit_code: 0,
                        stdout: String::new(),
                        stderr: String::new(),
                    }))
                } else {
                    Ok(None)
                }
            },
            "start" | "stop" | "restart" if ! match_daemon.is_match(&rc) => {
                default::service_action(name, &format!("one{}", action))
            },
            _ => default::service_action(name, action),
        }
    }
}

//
// Telemetry
//

impl TelemetryTarget for Target {
    #[allow(unused_variables)]
    fn telemetry_init(host: &mut Host) -> Result<serde_json::Value> {
        let cpu_vendor = try!(telemetry_cpu_vendor());
        let cpu_brand = try!(unix::get_sysctl_item("hw\\.model"));
        let hostname = try!(default::hostname());
        let (version_str, version_maj, version_min) = try!(unix::version());

        let telemetry = Telemetry::new(
            Cpu::new(
                &cpu_vendor,
                &cpu_brand,
                try!(try!(unix::get_sysctl_item("hw\\.ncpu")).parse::<u32>()),
            ),
            try!(default::fs()),
            &hostname,
            try!(try!(unix::get_sysctl_item("hw\\.physmem")).parse::<u64>()),
            default::net(),
            Os::new(env::consts::ARCH, "unix", "freebsd", &version_str, version_maj, version_min, 0),
        );

        Ok(serde_json::to_value(telemetry)?)
    }
}

fn telemetry_cpu_vendor() -> Result<String> {
    let mut fh = try!(File::open("/var/run/dmesg.boot"));
    let mut fc = String::new();
    fh.read_to_string(&mut fc).unwrap();

    let regex = Regex::new(r#"(?m)^CPU:.+$\n\s+Origin="([A-Za-z]+)""#).unwrap();
    if let Some(cap) = regex.captures(&fc) {
        Ok(cap.get(1).unwrap().as_str().into())
    } else {
        Err(Error::Generic("Could not match CPU vendor".to_string()))
    }
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
