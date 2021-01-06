use clap::App;

pub fn create_app() -> App<'static, 'static> {
  App::new("timetracker-ruuvi-listener")
  .version("0.1")
  .about("Listens for ruuvitags and outputs acceleration values in various formats")
  .author("Samuli S.")
}
