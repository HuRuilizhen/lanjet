use crate::cli::BannerContext;
use local_ip_address::local_ip;
use qrcode::{render::unicode, QrCode};

pub fn show_banner(banner_context: BannerContext) {
    let addr = banner_context.addr;
    let base_dir = banner_context.base_dir.canonicalize().unwrap();
    let ignore = banner_context.ignore;
    let files_count = banner_context.files_count;
    let total_size = banner_context.total_size;
    let show_qrcode = banner_context.show_qrcode;

    println!("🚀 LanJet Service Started");
    println!("📂 serving on {}", base_dir.display());
    println!("🕶️ skipping {}", base_dir.join(ignore).display());
    println!(
        "🧮 serving {} files ({:.2} KB total)",
        files_count,
        total_size as f64 / 1024.0
    );
    println!("🌐 serving at http://{}:{}", addr.ip(), addr.port());
    println!(
        "🌐 servering at http://{}:{}",
        local_ip().unwrap(),
        addr.port()
    );

    if show_qrcode {
        println!("📱 quick scan:");
        let code = QrCode::new(format!(
            "http://{}:{}/files",
            local_ip().unwrap(),
            addr.port()
        ))
        .unwrap();
        let qrcode = code.render::<unicode::Dense1x2>().build();
        println!("{}", qrcode);
    }
}
