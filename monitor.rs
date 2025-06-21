use crate::pnl;

pub fn print_status(net_delta: f64) {
    let pnl = pnl::get_total_pnl();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    println!();
    println!("=========================");
    println!("📈 Samco MM Bot Status");
    println!("🕒 Time: {}", now);
    println!("💰 PnL: ₹{:.2}", pnl);
    println!("📉 Net Delta: {:.2}", net_delta);
    println!("=========================");
}
