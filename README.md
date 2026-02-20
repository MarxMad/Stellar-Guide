# Stellar Guide (Talleres en Español)

Repositorio simple en GitHub para impartir talleres de Stellar: documentación en Markdown, diagramas Mermaid y ejercicios prácticos de 0→builder.

## Estructura
- docs/: documentación principal
- exercises/: ejercicios prácticos
- contracts/: (reservado) ejemplos Soroban
- examples/: (reservado) clientes JS/Python
- slides/: (reservado) presentaciones Marp
- assets/: imágenes y recursos

## Primeros pasos
- Lee [docs/introduccion.md](docs/introduccion.md)
- Configura tu entorno con [docs/instalacion.md](docs/instalacion.md)
- Practica comandos en [docs/comandos-basicos.md](docs/comandos-basicos.md)
- Revisa flujos en [docs/flujos-mermaid.md](docs/flujos-mermaid.md)
- Sigue la [docs/guia-0-a-builder.md](docs/guia-0-a-builder.md)
- Haz el ejercicio inicial: [exercises/01-pago-simple.md](exercises/01-pago-simple.md)

## Cómo publicar en GitHub
1. Crear repo en GitHub vacío (por ejemplo `stellar-guide`)
2. En tu máquina:
   ```bash
   git init
   git add .
   git commit -m "init: estructura mínima docs y ejercicios"
   git branch -M main
   git remote add origin https://github.com/<tu-usuario>/stellar-guide.git
   git push -u origin main
   ```

## Licencia
MIT
# Stellar-Guide
