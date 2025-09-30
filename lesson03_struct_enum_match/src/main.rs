struct Packet {
    src: String,
    dst: String,
    port: String,
    payload: Vec<u8>,
}

impl Packet {
    fn print_info(&self) {
        let Self {
            src, dst, payload, ..
        } = self;
        let size = payload.len();
        println!("{} -> {} (size = {} bytes)", src, dst, size);
    }
    fn parse_port(&self) -> Option<u16> {
        let Ok(port) = self.port.parse::<u16>() else {
            return None;
        };

        Some(port)
    }
}

enum LogLevel {
    Info,
    Warning,
    Error(String),
}

fn main() {
    let pkt = Packet {
        src: "192.168.1.1".into(),
        dst: "192.168.1.2".into(),
        port: "8080".into(),
        payload: vec![1, 2, 3],
    };
    pkt.print_info();
    log(LogLevel::Info);
    log(LogLevel::Warning);
    log(LogLevel::Error("File not found!".into()));

    match pkt.parse_port() {
        Some(p) => println!("✅ Parsed port {}", p),
        None => println!("❌ Invalid port string {}", pkt.port),
    }
}

fn log(level: LogLevel) {
    match level {
        LogLevel::Info => println!("ℹ️ Info: Everything is fine."),
        LogLevel::Warning => println!("⚠️ Warning: Something looks suspicious."),
        LogLevel::Error(msg) => println!("❌ Error: {}", msg),
    }
}
