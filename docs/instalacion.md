# Instalación del Entorno

## Requisitos
- Node.js LTS (incluye npm)
- Rust y cargo (via rustup)
- Stellar CLI (herramienta principal)
- soroban-cli (alternativa/legacy para contratos)
- Opcional: Python 3 + pip para ejemplos alternativos

## Node.js
- Descarga desde https://nodejs.org o usa nvm:
  ```bash
  # macOS/Linux con nvm instalado
  nvm install --lts
  nvm use --lts
  node -v
  npm -v
  ```

## Rust y cargo
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc -V
cargo -V
```

## Stellar CLI
Instala la última versión estable:
```bash
# Script (macOS, Linux)
curl -fsSL https://github.com/stellar/stellar-cli/raw/main/install.sh | sh

# Homebrew (macOS, Linux)
brew install stellar-cli

# Windows (winget)
winget install --id Stellar.StellarCLI

# Cargo desde fuente
cargo install --locked stellar-cli
```

Autocompletado:
```bash
# Bash/Zsh/Fish, etc.
stellar completion --shell <bash|zsh|fish|powershell|...>
# Activar temporalmente en bash
source <(stellar completion --shell bash)
```

## soroban-cli
```bash
cargo install --locked soroban-cli
soroban -V
```

## SDKs opcionales
- JavaScript:
  ```bash
  npm install @stellar/stellar-sdk
  ```
- Python:
  ```bash
  pip install stellar-sdk
  ```
