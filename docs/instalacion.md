# Instalación del Entorno

## Requisitos
- Node.js LTS (incluye npm)
- Rust y cargo (via rustup)
- soroban-cli (herramienta para contratos)
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
