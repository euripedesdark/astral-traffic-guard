#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Astral Internet Security - Compilador e Publicador Limpo
Executa compilação do Frontend React/TypeScript (Vite),
compilação limpa em Rust (cargo clean + cargo build --release),
copia o executável e arquivos estáticos para o diretório global /opt/astral-traffic-guard
e reinicia o serviço no systemd.

Uso:
    ./deploy.py
ou
    python3 deploy.py
"""

import os
import sys
import subprocess
import time

SERVICE_NAME = "astral-traffic-guard"
PORT = 8085
AUTH_PORT = 8091
GLOBAL_DIR = "/opt/astral-traffic-guard"
APP_DIR = os.path.dirname(os.path.abspath(__file__))
FRONTEND_DIR = os.path.join(APP_DIR, "frontend")
STATIC_BUILD_DIR = os.path.join(APP_DIR, "src", "main", "resources", "static", "firewall")
TARGET_STATIC_DIR = os.path.join(GLOBAL_DIR, "static", "firewall")
SUDO_PASS = "Btf1_Mk3"

GREEN = "\033[0;32m"
BLUE = "\033[0;34m"
YELLOW = "\033[1;33m"
RED = "\033[0;31m"
BOLD = "\033[1m"
NC = "\033[0m"

def log(msg, color=BLUE):
    print(f"{color}{BOLD}[ASTRAL-DEPLOY]{NC} {msg}")

def run_sudo(cmd):
    if os.geteuid() == 0:
        return subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return subprocess.run(f"sudo -S {cmd}", shell=True, input=f"{SUDO_PASS}\n", capture_output=True, text=True)

def main():
    print(f"{BLUE}{BOLD}============================================================{NC}")
    print(f"{GREEN}{BOLD}   Astral Internet Security - Compilação & Publicação Limpa   {NC}")
    print(f"{BLUE}{BOLD}============================================================{NC}")

    os.chdir(APP_DIR)

    # 1. Compilação do Frontend React/TypeScript
    if os.path.exists(FRONTEND_DIR):
        log("Compilando Frontend React/TypeScript (Vite + PrimeReact)...", BLUE)
        start_front = time.time()
        front_proc = subprocess.run(["npm", "run", "build"], cwd=FRONTEND_DIR, capture_output=True, text=True)
        if front_proc.returncode != 0:
            log(f"ERRO na compilação do Frontend:\n{front_proc.stderr or front_proc.stdout}", RED)
            sys.exit(1)
        elapsed_front = time.time() - start_front
        log(f"Frontend compilado com sucesso em {elapsed_front:.2f} segundos!", GREEN)

    # 2. Limpeza Completa (Clean Build) do Rust
    log("Limpando cache de compilação anterior (cargo clean)...", YELLOW)
    clean_proc = subprocess.run(["cargo", "clean", "-p", SERVICE_NAME], capture_output=True, text=True)
    if clean_proc.returncode != 0:
        subprocess.run(["cargo", "clean"], capture_output=True, text=True)
    log("Cache limpo com sucesso.", GREEN)

    # 3. Compilação Otimizada de Release em Rust
    log("Iniciando compilação otimizada em Rust (cargo build --release)...", BLUE)
    start_time = time.time()
    build_proc = subprocess.run(["cargo", "build", "--release"])
    if build_proc.returncode != 0:
        log("ERRO: Falha na compilação do Rust!", RED)
        sys.exit(1)
    elapsed = time.time() - start_time
    log(f"Backend Rust compilado com sucesso em {elapsed:.2f} segundos!", GREEN)

    bin_path = os.path.join(APP_DIR, "target", "release", SERVICE_NAME)
    if not os.path.exists(bin_path):
        log(f"ERRO: Binário compilado não encontrado em {bin_path}", RED)
        sys.exit(1)

    # 4. Publicação no Diretório do Sistema (/opt/...)
    log(f"Instalando binário e arquivos no diretório global {GLOBAL_DIR}...", BLUE)
    run_sudo(f"mkdir -p {GLOBAL_DIR} {TARGET_STATIC_DIR} /etc/astral && rm -rf {GLOBAL_DIR}/static/app")
    target_bin = os.path.join(GLOBAL_DIR, SERVICE_NAME)
    
    cp_res = run_sudo(f"cp -f {bin_path} {target_bin}")
    if cp_res.returncode != 0:
        log(f"ERRO ao copiar binário: {cp_res.stderr}", RED)
        sys.exit(1)

    run_sudo(f"chmod 755 {target_bin}")

    # Copiar estáticos do frontend se compilados
    if os.path.exists(STATIC_BUILD_DIR):
        log(f"Copiando arquivos da UI para {TARGET_STATIC_DIR}...", BLUE)
        run_sudo(f"cp -rf {STATIC_BUILD_DIR}/* {TARGET_STATIC_DIR}/")
        run_sudo(f"chmod -R 755 {GLOBAL_DIR}/static")

    log(f"Binário e assets instalados em {GLOBAL_DIR} com sucesso.", GREEN)

    # 5. Reinicialização do Serviço no Systemd
    log(f"Reiniciando serviço systemd '{SERVICE_NAME}'...", YELLOW)
    run_sudo(f"systemctl restart {SERVICE_NAME}")
    time.sleep(2.0)

    # 6. Verificação de Saúde
    status_proc = run_sudo(f"systemctl is-active {SERVICE_NAME}")
    status = status_proc.stdout.strip()
    if status == "active":
        log(f"SUCESSO! O serviço '{SERVICE_NAME}' está ATIVO e rodando na porta {PORT}.", GREEN)
    else:
        log(f"AVISO: O serviço retornou status: {status}", RED)
        log("Verificando logs recentes do journalctl:", YELLOW)
        journal = run_sudo(f"journalctl -u {SERVICE_NAME} -n 15 --no-pager")
        print(journal.stdout)
        sys.exit(1)

    # 7. Teste de Endpoint Health e UI
    log("Validando endpoints REST e UI...", BLUE)
    curl_health = run_sudo(f"curl -s http://127.0.0.1:{PORT}/health || true")
    print(f"Health check: {curl_health.stdout.strip()}")

    # 8. Exibir porta em escuta
    ports = run_sudo(f"ss -tulpn | grep :{PORT} || true")
    if ports.stdout.strip():
        print(f"\n{GREEN}Porta em escuta:{NC}\n{ports.stdout.strip()}")

    print(f"\n{BLUE}{BOLD}============================================================{NC}")
    print(f"{GREEN}{BOLD}   Publicação e deploy concluídos com 100% de sucesso!       {NC}")
    print(f"{BLUE}{BOLD}   Acesse a UI em: https://192.168.2.10/proxy/ ou http://192.168.2.10:{PORT}/proxy/          {NC}")
    print(f"{BLUE}{BOLD}============================================================{NC}")

if __name__ == "__main__":
    main()
