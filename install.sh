#!/usr/bin/env bash
set -euo pipefail

REPO="you/llm-cli"
BINARY="llm-cli"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

info()  { printf "${GREEN}%s${NC}\n" "$*"; }
error() { printf "${RED}%s${NC}\n" "$*" >&2; }

detect_target() {
  local os arch

  case "$(uname -s)" in
    Linux)  os="unknown-linux-musl" ;;
    Darwin) os="apple-darwin" ;;
    *)      error "Unsupported OS: $(uname -s)"; exit 1 ;;
  esac

  case "$(uname -m)" in
    x86_64|amd64) arch="x86_64" ;;
    aarch64|arm64) arch="aarch64" ;;
    *)            error "Unsupported architecture: $(uname -m)"; exit 1 ;;
  esac

  echo "${arch}-${os}"
}

main() {
  local target
  target=$(detect_target)

  local archive="llm-cli-${target}.tar.gz"
  local url="https://github.com/${REPO}/releases/latest/download/${archive}"

  mkdir -p "$INSTALL_DIR"

  info "Downloading llm-cli (${target})..."
  local tmp_dir
  tmp_dir=$(mktemp -d)
  trap 'rm -rf "$tmp_dir"' EXIT

  if command -v curl &>/dev/null; then
    curl -sfL "$url" -o "${tmp_dir}/${archive}"
  elif command -v wget &>/dev/null; then
    wget -q "$url" -O "${tmp_dir}/${archive}"
  else
    error "Neither curl nor wget found. Install one of them and retry."
    exit 1
  fi

  tar xzf "${tmp_dir}/${archive}" -C "$tmp_dir"
  install -m 755 "${tmp_dir}/${BINARY}" "${INSTALL_DIR}/${BINARY}"

  info "llm-cli installed to ${INSTALL_DIR}/${BINARY}"

  case ":$PATH:" in
    *:"$INSTALL_DIR":*) ;;
    *) error "Warning: ${INSTALL_DIR} is not in PATH. Add it to your shell profile." ;;
  esac
}

main "$@"
