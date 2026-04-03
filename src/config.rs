use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: std::net::IpAddr,
    #[clap(long, env = "PORT", default_value = "3000")]
    pub port: u16,

    #[clap(long, env = "RUST_LOG", default_value = "info")]
    pub log_filter: String,

    #[clap(long, env = "WORKER_THREADS", default_value_t = num_cpus::get())]
    pub worker_threads: usize,

    #[clap(long, env = "REQUEST_TIMEOUT_SECS", default_value = "30", value_parser = parse_duration_secs)]
    pub request_timeout: std::time::Duration,
}

impl Config {
    pub fn socket_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::from((self.host, self.port))
    }
}

fn parse_duration_secs(s: &str) -> Result<std::time::Duration, String> {
    s.parse::<u64>()
        .map(std::time::Duration::from_secs)
        .map_err(|e| e.to_string())
}
