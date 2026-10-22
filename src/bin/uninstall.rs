#!/usr/bin/env rust
use std::fs;
use std::process::Command;

fn run_cmd(cmd: &str) {
    println!("[EXEC] {}", cmd);
    let _ = Command::new("sh").arg("-c").arg(cmd).status();
}

fn main() {
    println!("=== Desinstalador Astral Traffic Guard ===");
    run_cmd("systemctl stop astral-traffic-guard || true");
    run_cmd("systemctl disable astral-traffic-guard || true");
    let _ = fs::remove_file("/etc/systemd/system/astral-traffic-guard.service");
    let _ = fs::remove_file("/usr/local/bin/astral-traffic-guard");
    run_cmd("systemctl daemon-reload");
    println!("✓ Astral Traffic Guard desinstalado com sucesso.");
}
