#!/usr/bin/env rust
use std::process::Command;

fn check_service(name: &str) {
    let out = Command::new("systemctl")
        .args(["is-active", name])
        .output();
    match out {
        Ok(o) => {
            let status = String::from_utf8_lossy(&o.stdout).trim().to_string();
            println!("[STATUS] {} -> {}", name, status);
        },
        Err(e) => eprintln!("[ERRO] Não foi possível checar {}: {}", name, e),
    }
}

fn main() {
    println!("=== Verificação de Saúde do Astral Traffic Guard ===");
    check_service("astral-traffic-guard");
    println!("Testando endpoint local auth_request...");
    let _ = Command::new("curl").args(["-s", "-o", "/dev/null", "-w", "%{http_code}\n", "http://localhost:8080/actuator/health"]).status();
    println!("✓ Diagnóstico completo!");
}
