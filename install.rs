#!/usr/bin/env rust
use std::fs;
use std::path::Path;
use std::process::Command;

fn run_cmd(cmd: &str) -> bool {
    println!("[EXEC] {}", cmd);
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    println!("============================================================");
    println!("   Instalador Astral Traffic Guard (Rust & Reverse Proxy)");
    println!("============================================================");

    let is_root = Command::new("id").arg("-u").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false);

    if !is_root {
        println!("[AVISO] Para instalar serviços no systemd, execute com sudo.");
    }

    println!("[1/4] Compilando backend em Rust (Release)...");
    if !run_cmd("cargo build --release") {
        eprintln!("[ERRO] Falha ao compilar backend em Rust!");
        std::process::exit(1);
    }
    println!("[OK] Binário Rust gerado com sucesso.");

    println!("[2/4] Verificando e instalando binário em /usr/local/bin...");
    let bin_src = "target/release/astral-traffic-guard";
    if Path::new(bin_src).exists() && is_root {
        let _ = fs::copy(bin_src, "/usr/local/bin/astral-traffic-guard");
        let _ = Command::new("chmod").args(["+x", "/usr/local/bin/astral-traffic-guard"]).status();
        println!("[OK] Instalado em /usr/local/bin/astral-traffic-guard.");
    }

    println!("[3/4] Configurando Systemd Service Unit...");
    let unit = r#"[Unit]
Description=Astral Traffic Guard - Reverse Proxy, ACL Engine & WAF (Rust)
After=network.target network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/astral-traffic-guard
Restart=always
RestartSec=5
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
"#;
    if is_root {
        let _ = fs::write("/etc/systemd/system/astral-traffic-guard.service", unit);
        run_cmd("systemctl daemon-reload");
        println!("[OK] Serviço astral-traffic-guard registrado no systemd.");
    }

    println!("[4/4] Instalação concluída com sucesso!");
    println!("Execute: sudo systemctl start astral-traffic-guard");
}
