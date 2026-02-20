# Comandos Básicos

## Stellar CLI (recetas y autocompletado)
- Tareas típicas: enviar pagos, gestionar ciclo de vida de contratos, extender instancia/almacenamiento/wasm, y más (ver “Cookbook” del Stellar CLI).
- Autocompletado:
```bash
stellar completion --shell bash
source <(stellar completion --shell bash)
```

## Redes y llaves (Testnet)
```bash
# Usar Testnet
stellar network use testnet

# Generar y FONDEAR identidad (usa Friendbot internamente)
stellar keys generate --fund alice --network testnet

# Generar otra identidad sin fondear
stellar keys generate bob

# Añadir una clave pública existente con alias
stellar keys add --public-key G... charlie
```

## Fundear cuentas y pagos (tx)
```bash
# Crear y fundear cuenta (bob recibe 10 XLM)
stellar tx new create-account \
  --source alice \
  --destination bob \
  --starting-balance 100_000_000

# Enviar pago nativo (XLM) de bob a charlie: 4 XLM
stellar tx new payment \
  --source bob \
  --destination charlie \
  --asset native \
  --amount 40_000_000
```

## Contratos con Stellar CLI
```bash
# Inicializar proyecto de contrato
stellar contract init --name counter .

# Compilar
stellar contract build --manifest-path contracts/counter/Cargo.toml

# Desplegar (WASM ya compilado)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/counter.wasm \
  --network testnet --source alice

# Invocar función (pasando args tras --)
stellar contract invoke --id <CONTRACT_ID> \
  --source alice --network testnet -- \
  increment
```

## Consulta rápida (opcional)
Si deseas consultar balances/estado de una cuenta con Horizon:
```bash
curl "https://horizon-testnet.stellar.org/accounts/<PUBLIC_KEY>"
```

## SDK JS (snippet mínimo)
```js
import { Asset, Keypair, Server, Networks, TransactionBuilder, Operation } from "@stellar/stellar-sdk";

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
