# Ejercicio 01: Pago Simple en Testnet

## Objetivo
Enviar un pago de XLM desde tu cuenta a otra cuenta en Testnet y verificar el resultado en Horizon.

## Requisitos
- Clave pública y privada (Keypair)
- Cuenta fondeada con Friendbot
- Node.js o SDK de tu preferencia

## Pasos
1. Genera una cuenta (Keypair) y guarda su clave privada de forma segura.
2. Fondea con Friendbot:
   ```bash
   curl "https://friendbot.stellar.org/?addr=<PUBLIC_KEY>"
   ```
3. Crea otra cuenta destino y también fondea (o usa una pública existente).
4. Envía un pago simple de 5 XLM a la cuenta destino.
5. Verifica el resultado en Horizon:
   ```bash
   curl "https://horizon-testnet.stellar.org/accounts/<DEST_PUBLIC_KEY>"
   ```

## Criterios de Éxito
- La transacción devuelve un hash válido.
- El balance de la cuenta destino aumenta en la cantidad enviada.

## Siguientes pasos
- Repite con distintos montos.
- Implementa verificación automática con un script.
