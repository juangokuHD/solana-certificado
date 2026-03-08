# Videojuegos El Macho - Tienda CRUD en Solana

Proyecto desarrollado como parte del Solana Latam Builders Program. Es una implementación sencilla de un CRUD (Create, Read, Update, Delete) para una tienda de videojuegos en la blockchain de Solana, usando el framework Anchor.

La idea es simular una tienda donde se guardan videojuegos con su nombre, precio y si son clásicos o modernos. Hay un descuento fijo del 10% que se calcula en el cliente (no en el programa on-chain para mantenerlo simple).

## ¿Qué hace el programa?

Es un smart contract básico que permite:
- Inicializar la tienda (solo una vez, guarda el authority y el descuento del 10%).
- Crear un videojuego nuevo (con nombre, precio y tipo: clásico o moderno).
- Actualizar el precio de un juego existente.
- Eliminar un juego.

Todo usa PDAs (Program Derived Addresses) para las cuentas:
- La tienda usa seeds ["store", authority].
- Cada juego usa seeds ["game", authority, nombre_del_juego].

Esto evita que cualquiera pueda modificar cualquier cuenta y mantiene el control en el authority (tu wallet).

## Cuenta principal: Store
| Campo       | Tipo    | Descripción                          |
|-------------|---------|--------------------------------------|
| authority   | Pubkey  | Wallet que controla la tienda        |
| discount    | u8      | Descuento fijo (10)                  |

## Cuenta por juego: Game
| Campo       | Tipo    | Descripción                          |
|-------------|---------|--------------------------------------|
| authority   | Pubkey  | Wallet que creó el juego             |
| name        | String  | Nombre del videojuego (ej: "elden ring") |
| price       | u64     | Precio original (en unidades simples)|
| is_classic  | bool    | true si es clásico, false si moderno |

## Instrucciones del programa
| Instrucción     | Quién la ejecuta | Qué hace                                      |
|-----------------|------------------|-----------------------------------------------|
| initialize      | Authority        | Crea la cuenta de la tienda con descuento 10% |
| create_game     | Authority        | Agrega un nuevo videojuego                    |
| update_game     | Authority        | Cambia el precio de un juego existente        |
| delete_game     | Authority        | Borra un juego (cierra la cuenta PDA)         |

## Cómo probarlo
El proyecto se desarrolló y probó en Solana Playground[](https://beta.solpg.io/).

Pasos para ejecutarlo:
1. Abre el proyecto desde el Share link (si te lo comparto) o crea uno nuevo.
2. Build (compila el programa).
3. Deploy a Devnet (sube el programa a la red de pruebas, cuesta algo de SOL de airdrop).
4. Run Test (ejecuta el archivo tests/videojuegos_el_macho.ts).

El test:
- Inicializa la tienda (si ya existe, comenta esa parte para evitar error 0x0).
- Crea 5 videojuegos de ejemplo (clásicos y modernos).
- Muestra el precio original y con 10% de descuento en consola.
- Actualiza el precio de uno (ej: Elden Ring).
- Borra otro (ej: Cyberpunk 2077).

Ejemplo de salida en consola:


## Stack tecnológico
- Blockchain: Solana (Devnet)
- Framework: Anchor (Rust)
- Lenguaje: Rust para el programa, TypeScript para los tests
- Entorno de desarrollo: Solana Playground (beta.solpg.io)
- Wallet: Phantom o Backpack (para deploy y tests)

## Estructura del proyecto

videojuegos_el_macho/
├── src/
│   └── lib.rs               # Código del programa en Rust/Anchor
├── tests/
│   └── videojuegos_el_macho.ts  # Tests con Anchor (Mocha-style)
├── Anchor.toml              # Configuración de Anchor
└── README.md                # Esta documentación


## Notas finales
Este es un proyecto básico para demostrar el uso de Anchor: cuentas PDA, instrucciones simples, validaciones de authority y tests locales/remotos. No incluye frontend porque el requerimiento era un programa sencillo CRUD.

El descuento del 10% se calcula en el test (price * 0.9), no en el programa, para no complicar la lógica on-chain.

Desplegado en Devnet. Program ID: [pega aquí tu Program ID real del explorer si lo tienes, ej: 7xYz...]

Cualquier duda o mejora, puedes clonar el repo y probarlo localmente con `anchor build`, `anchor deploy --provider.cluster devnet` y `anchor test`.

Gracias al equipo de Solana Latam por el bootcamp.

