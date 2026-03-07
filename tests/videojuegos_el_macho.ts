import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VideojuegosElMacho } from "../target/types/videojuegos_el_macho";

describe("videojuegos el macho", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .VideojuegosElMacho as Program<VideojuegosElMacho>;
  const authority = provider.wallet.publicKey;

  it("inicializa tienda y hace crud completo", async () => {
    // pda de la tienda
    const [storePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("store"), authority.toBuffer()],
      program.programId
    );

    await program.methods
      .initialize()
      .accounts({ store: storePda, authority })
      .rpc();
    console.log("tienda 'videojuegos el macho' creada con 10% descuento");

    // juegos de la tienda
    const juegos = [
      { name: "super mario bros", price: 2000, isClassic: true },
      { name: "the legend of zelda", price: 3000, isClassic: true },
      { name: "elden ring", price: 6000, isClassic: false },
      { name: "god of war ragnarok", price: 5500, isClassic: false },
      { name: "cyberpunk 2077", price: 5000, isClassic: false },
    ];

    for (const juego of juegos) {
      const [gamePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("game"), authority.toBuffer(), Buffer.from(juego.name)],
        program.programId
      );

      await program.methods
        .createGame(juego.name, new anchor.BN(juego.price), juego.isClassic)
        .accounts({ game: gamePda, authority })
        .rpc();

      const precioConDescuento = juego.price * 0.9;
      console.log(
        `juego creado: ${juego.name} | precio original: ${juego.price} | con 10% descuento: ${precioConDescuento}`
      );
    }

    // parte del crud:
    // actualizar
    const [gamePdaUpdate] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("game"), authority.toBuffer(), Buffer.from("elden ring")],
      program.programId
    );
    await program.methods
      .updateGame(new anchor.BN(5800))
      .accounts({ game: gamePdaUpdate, authority })
      .rpc();
    console.log("precio de elden ring actualizado a 5800");

    // eliminar
    const [gamePdaDelete] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("game"),
        authority.toBuffer(),
        Buffer.from("cyberpunk 2077"),
      ],
      program.programId
    );
    await program.methods
      .deleteGame()
      .accounts({ game: gamePdaDelete, authority })
      .rpc();
    console.log("juego cyberpunk 2077 borrado");

    console.log("¡CRUD completo! Todo funciona en 'videojuegos el macho'");
  });
});
