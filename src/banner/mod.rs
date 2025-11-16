mod util;

use crate::cli::BannerContext;
use crate::util::human_size;
use colored::*;
use local_ip_address::local_ip;
use qrcode::{render::unicode, QrCode};
use util::{canon, label};

pub fn show_banner(banner_context: BannerContext) {
    let addr = banner_context.addr;
    let base_dir = canon(&banner_context.base_dir);
    let ignore = &banner_context.ignore;
    let files_count = banner_context.files_count;
    let total_size = human_size(banner_context.total_size);
    let show_qr = banner_context.show_qrcode;
    let local_only = banner_context.local_only;
    let lan_ip = local_ip().unwrap_or(addr.ip());

    println!();
    println!("{}", "🚀 LanJet Service Started".bright_yellow().bold());
    println!(
        "{}",
        "──────────────────────────────────────────────".dimmed()
    );

    println!("{} {}", label("📂 Directory:"), base_dir.display());
    println!(
        "{} {}",
        label("🕶️ Ignoring:"),
        base_dir.join(ignore).display()
    );
    println!(
        "{} {} files ({})",
        label("🧮 Statistic:"),
        files_count,
        total_size
    );
    println!(
        "{} http://{}:{}",
        label("🏠 Address:"),
        addr.ip(),
        addr.port()
    );
    if !local_only {
        println!("{} http://{}:{}", label("🌐 Address:"), lan_ip, addr.port());
    }

    if show_qr && !local_only {
        println!();
        println!("{}", "📱 Quick Scan:".bright_magenta().bold());
        if let Ok(code) = QrCode::new(format!("http://{}:{}/", lan_ip, addr.port())) {
            let qr = code.render::<unicode::Dense1x2>().build();
            println!("{}", qr);
        }
    }
    println!();
}
