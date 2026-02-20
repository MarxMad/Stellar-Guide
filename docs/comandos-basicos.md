# Comandos Básicos

## Friendbot (Testnet)
Fondea una cuenta en Testnet con su clave pública:
```bash
curl "https://friendbot.stellar.org/?addr=<PUBLIC_KEY>"
```

## Consultar cuenta (Horizon)
```bash
curl "https://horizon-testnet.stellar.org/accounts/<PUBLIC_KEY>"
```

## Soroban CLI (contratos)
Compilar, desplegar e invocar (ejemplo general):
```bash
# Compilar contrato (ajusta ruta a tu contrato)
soroban contract build --manifest-path contracts/<nombre>/Cargo.toml

# Configurar red Testnet
soroban config network add --global testnet \
  --rpc-url https://rpc.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Generar clave
soroban keys generate alice

# Desplegar
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/<nombre>.wasm \
  --network testnet --source alice

# Invocar función
soroban contract invoke --id <CONTRACT_ID> \
  --fn <funcion> --network testnet --source alice
```

## SDK JS (snippet mínimo)
```js
import { Keypair, Server, Networks, TransactionBuilder, Operation } from "@stellar/stellar-sdk";

const server = new Server("https://horizon-testnet.stellar.org");
const pair = Keypair.random();
// Fondea con Friendbot antes de usar

async function pagoSimple(destination, amount) {
  const account = await server.loadAccount(pair.publicKey());
  const tx = new TransactionBuilder(account, { fee: "100", networkPassphrase: Networks.TESTNET })
    .addOperation(Operation.payment({ destination, asset: Asset.native(), amount }))
    .setTimeout(30)
    .build();
  tx.sign(pair);
  const res = await server.submitTransaction(tx);
  console.log(res.hash);
}
```
