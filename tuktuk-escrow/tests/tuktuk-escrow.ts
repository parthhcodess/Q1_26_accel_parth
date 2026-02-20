import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import {
    LAMPORTS_PER_SOL,
    PublicKey,
    Keypair,
    SystemProgram,
} from "@solana/web3.js";
import { TuktukEscrow } from "../target/types/tuktuk_escrow";
import {
    createMint,
    createAssociatedTokenAccount,
    mintTo,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("tuktuk-escrow", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.TuktukEscrow as Program<TuktukEscrow>;
    const maker = anchor.Wallet.local().payer;

    let mintA: PublicKey;
    let mintB: PublicKey;
    let makerAtaA: PublicKey;

    const seed = new anchor.BN(1);

    const escrowPda = web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("escrow"),
            maker.publicKey.toBuffer(),
            seed.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
    )[0];

    before(async () => {
        const balance = await provider.connection.getBalance(maker.publicKey);
        console.log("Balance:", balance / LAMPORTS_PER_SOL, "SOL");

        // Create mints
        mintA = await createMint(
            provider.connection,
            maker,
            maker.publicKey,
            null,
            6
        );
        mintB = await createMint(
            provider.connection,
            maker,
            maker.publicKey,
            null,
            6
        );

        // Create maker's ATAs
        makerAtaA = await createAssociatedTokenAccount(
            provider.connection,
            maker,
            mintA,
            maker.publicKey
        );

        // Mint tokens to maker
        await mintTo(
            provider.connection,
            maker,
            mintA,
            makerAtaA,
            maker,
            1_000_000_000 // 1000 tokens
        );

        console.log("Mint A:", mintA.toBase58());
        console.log("Mint B:", mintB.toBase58());
        console.log("Escrow PDA:", escrowPda.toBase58());
    });

    it("Creates an escrow (make)", async () => {
        const tx = await program.methods
            .make(seed, new anchor.BN(500_000_000), new anchor.BN(100_000_000))
            .accountsPartial({
                maker: maker.publicKey,
                mintA,
                mintB,
                makerAtaA,
            })
            .rpc();

        console.log("Escrow created:", tx);

        const escrow = await program.account.escrow.fetch(escrowPda);
        console.log("Escrow data:", {
            seed: escrow.seed.toString(),
            maker: escrow.maker.toBase58(),
            refundDeadline: new Date(escrow.refundDeadline.toNumber() * 1000).toISOString(),
        });
    });

    it("Manual refund works", async () => {
        const tx = await program.methods
            .refund()
            .accountsPartial({
                maker: maker.publicKey,
                mintA,
                makerAtaA,
                escrow: escrowPda,
            })
            .rpc();

        console.log("Escrow refunded:", tx);
    });
});