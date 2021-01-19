use clap::{App, Arg};

pub static OUTPUT: &str = "output";

pub fn create_app() -> App<'static, 'static> {
  App::new("timetracker-ruuvi-listener")
  .version("0.1")
  .about("Listens for ruuvitags and outputs acceleration values in various formats")
  .author("Samuli S.")
  .arg(Arg::with_name(OUTPUT).long("output").short("o").required(true).takes_value(true))
}
