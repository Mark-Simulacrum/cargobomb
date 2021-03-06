#![recursion_limit = "1024"]

extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
#[macro_use(slog_info, slog_log,
            slog_record, slog_record_static, slog_b, slog_kv)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate slog_term;
extern crate result;

extern crate cargobomb;

mod model;

use cargobomb::{log, util};
use cargobomb::errors::*;
use clap::{App, AppSettings};
use std::panic;
use std::process;

fn main() {
    let _guard = log::init();
    let success = match panic::catch_unwind(main_) {
        Ok(Ok(())) => {
            true
        }
        Ok(Err(e)) => {
            util::report_error(&e);
            false
        }
        Err(e) => {
            util::report_panic(&*e);
            false
        }
    };
    info!("{}",
          if success {
              "command succeeded"
          } else {
              "command failed"
          });
    log::finish();
    process::exit(if success { 0 } else { 1 });
}

fn main_() -> Result<()> {
    let matches = cli().get_matches();
    let cmd = model::conv::clap_args_to_cmd(&matches)?;
    cmd.run()
}

fn cli() -> App<'static, 'static> {
    App::new("cargobomb")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Kaboom!")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(model::conv::clap_cmds())
}
