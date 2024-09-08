use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    /// Адрес, на котором будет запущен сервис.
    #[clap(long, default_value_t = false)]
    pub dev_tools: bool,

    #[clap(long, default_value = "info")]
    pub log_level: String,
}
