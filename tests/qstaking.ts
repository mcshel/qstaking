import assert from 'assert';
import * as spl from '@solana/spl-token';
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, Keypair } from '@solana/web3.js';
import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import { Qstaking } from "../target/types/qstaking";

describe("qstaking", () => {
    const program = anchor.workspace.Qstaking as Program<Qstaking>;

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    const authoritySecret = JSON.parse(require('fs').readFileSync('/home/mpetac/.config/solana/id.json', 'utf8'));
    const authorityKeypair = Keypair.fromSecretKey(Uint8Array.from(authoritySecret));
    
    const managerKeyepair = Keypair.generate();
    const falseManagerKeyepair = Keypair.generate();
    const creatorKeypair = Keypair.generate();
    const userKeypair1 = Keypair.generate();
    const userKeypair2 = Keypair.generate();
    
    let programDataAccount: PublicKey;
    let mineAccount: PublicKey;
    let rewardMintAccount: PublicKey;
    let creatorWhitelistAccount: PublicKey;
    
    let nft1 = null;
    let nft2 = null;
    
    const creatorWhitelist = false;
    const mintWhitelist = [];
    
    
    const mine_rate = 100;
    
    before( async () => {
        
        const airdropSignature1 = await provider.connection.requestAirdrop(managerKeyepair.publicKey, 1e9);
        await provider.connection.confirmTransaction(airdropSignature1);
        
        const airdropSignature2 = await provider.connection.requestAirdrop(falseManagerKeyepair.publicKey, 1e9);
        await provider.connection.confirmTransaction(airdropSignature2);
        
        const airdropSignature3 = await provider.connection.requestAirdrop(creatorKeypair.publicKey, 1e9);
        await provider.connection.confirmTransaction(airdropSignature3);
        
        const airdropSignature4 = await provider.connection.requestAirdrop(userKeypair1.publicKey, 1e9);
        await provider.connection.confirmTransaction(airdropSignature4);
        
        const airdropSignature5 = await provider.connection.requestAirdrop(userKeypair2.publicKey, 1e9);
        await provider.connection.confirmTransaction(airdropSignature5);
        
        let bump;
        [mineAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("mine")], program.programId);
        
        rewardMintAccount = await spl.createMint(provider.connection, authorityKeypair, mineAccount, authorityKeypair.publicKey, 6);
        
        const programAaccountInfo = await provider.connection.getAccountInfo(program.programId);
        try {
            programDataAccount = new PublicKey(programAaccountInfo.data.slice(4, 36));
        } catch (e) {
            console.log(`\tFailed getting the program's data account: ${e}`);
        }
        
        nft1 = await createNFT(provider.connection, creatorKeypair, userKeypair1, 'Bastard #1', 'https://raffles-test.s3.amazonaws.com/NFT1.jpg');
        nft2 = await createNFT(provider.connection, creatorKeypair, userKeypair2, 'Bastard #2', 'https://raffles-test.s3.amazonaws.com/NFT2.jpg');
        
        if (!creatorWhitelist) {
            mintWhitelist.push(nft1, nft2);
        }
        
    });
    
    
    it("Whitelist account(s) created!", async () => {
        
        const tx = new anchor.web3.Transaction();
        
        if (creatorWhitelist) {
            const [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), creatorKeypair.publicKey.toBuffer()], program.programId);
            
            const ix = program.instruction.addWhitelist(creatorKeypair.publicKey, new anchor.BN(1), {
                accounts: {
                    program: program.programId,
                    programData: programDataAccount,
                    whitelist: creatorWhitelistAccount,
                    authority: authorityKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }
            });
            tx.add(ix);
        }
        
        for (let nft of mintWhitelist) {
            const [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), nft.mintAddress.toBuffer()], program.programId);
        
            const ix = program.transaction.addWhitelist(nft.mintAddress, new anchor.BN(0), {
                accounts: {
                    program: program.programId,
                    programData: programDataAccount,
                    whitelist: whitelistAccount,
                    authority: authorityKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }
            });
            tx.add(ix);
        }
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [authorityKeypair], {skipPreflight: true});
        console.log(`\tAdd creator whitelist account transaction: ${signature}`);
        
    });
    
    
    it("Mine is initialized!", async () => {
        
        if (await provider.connection.getAccountInfo(mineAccount)) {
            
            const tx = program.transaction.setMineManager(managerKeyepair.publicKey, {
                accounts: {
                    program: program.programId,
                    programData: programDataAccount,
                    mine: mineAccount,
                    authority: authorityKeypair.publicKey,
                }
            });
            
            const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [authorityKeypair], {skipPreflight: true});
            console.log(`\tSet mine manager transaction: ${signature}`);
            
            const mineAccountData = await program.account.mine.fetch(mineAccount);
            assert.equal(mineAccountData.manager.toString(), managerKeyepair.publicKey.toString());
            
            
        } else {
        
            const tx = program.transaction.initMine(managerKeyepair.publicKey, {
                accounts: {
                    program: program.programId,
                    programData: programDataAccount,
                    mine: mineAccount,
                    mint: rewardMintAccount,
                    authority: authorityKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                },
            });
            
            const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [authorityKeypair], {skipPreflight: true});
            console.log(`\tInitialize mine transaction: ${signature}`);
            
            const mineAccountData = await program.account.mine.fetch(mineAccount);
            assert.equal(mineAccountData.locked, true);
            assert.equal(mineAccountData.manager.toString(), managerKeyepair.publicKey.toString());
            assert.equal(mineAccountData.mint.toString(), rewardMintAccount.toString());
            assert.equal(mineAccountData.rate, 0);
            assert.equal(mineAccountData.stakedMiners, 0);
            assert.equal(mineAccountData.stakedPoints, 0);
            assert.equal(mineAccountData.accruedRewards, 0);
            //assert.equal(mineAccountData.accrued_timesstamp, 0);
        }
    });
    
    
    it("Mine rate set!", async () => {
        
        const tx = program.transaction.setMineRate(new anchor.BN(mine_rate), {
            accounts: {
                mine: mineAccount,
                manager: managerKeyepair.publicKey,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [managerKeyepair], {skipPreflight: true});
        console.log(`\tSet mine rate transaction: ${signature}`);
        
        const mineAccountData = await program.account.mine.fetch(mineAccount);
        assert.equal(mineAccountData.rate, mine_rate);
    });
    
    
    it("Mine lock set!", async () => {
        
        const tx = program.transaction.setMineLock(false, {
            accounts: {
                mine: mineAccount,
                manager: managerKeyepair.publicKey,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [managerKeyepair], {skipPreflight: true});
        console.log(`\tSet mine rate transaction: ${signature}`);
        
        const mineAccountData = await program.account.mine.fetch(mineAccount);
        assert.equal(mineAccountData.locked, false);
    });
    
    
    it("Mine updated!", async () => {
        
        const mineAccountData1 = await program.account.mine.fetch(mineAccount);
        //console.log(`\t Initial accrued reward: ${mineAccountData1.accruedRewards.toNumber()}`);
        //console.log(`\t Initial accrued timestamp: ${mineAccountData1.accruedTimestamp.toNumber()}, current timestamp: ${Date.now() * 1e-3}`);
        
        await new Promise(f => setTimeout(f, 2000));
        
        const keypair = Keypair.generate();
        const airdropSignature = await provider.connection.requestAirdrop(keypair.publicKey, 1e9);
        await provider.connection.confirmTransaction(airdropSignature);
        
        const tx = program.transaction.updateMine({
            accounts: {
                mine: mineAccount,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [managerKeyepair], {skipPreflight: true});
        console.log(`\tUpdate mine transaction: ${signature}`);
        
        const mineAccountData2 = await program.account.mine.fetch(mineAccount);
        //console.log(`\t Updated accrued reward: ${mineAccountData2.accruedRewards.toNumber()}`);
        //console.log(`\t Updated accrued timestamp: ${mineAccountData2.accruedTimestamp.toNumber()}, current timestamp: ${Date.now() * 1e-3}`);
        
        const expectedReward = mineAccountData1.accruedRewards.toNumber() + mineAccountData1.rate.toNumber() * (mineAccountData2.accruedTimestamp.toNumber() - mineAccountData1.accruedTimestamp.toNumber());
        assert.equal(mineAccountData2.accruedRewards, expectedReward);
    });
    
    
    it("Miner #1 created!", async () => {
        
        const [minerAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("miner"), nft1.mintAddress.toBuffer()], program.programId);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft1);
        const nftTokenAccount = await spl.getAssociatedTokenAddress(nft1.mintAddress, userKeypair1.publicKey);
        
        const tx = program.transaction.createMiner({
            accounts: {
                miner: minerAccount,
                whitelist: whitelistAccount,
                nftAta: nftTokenAccount,
                nftMetadata: nft1.metadataAddress,
                nftMint: nft1.mintAddress,
                user: userKeypair1.publicKey,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair1], {skipPreflight: true});
        console.log(`\tMiner #1 account creation transaction: ${signature}`);
        
    });
    
    
    it("Miner #2 created!", async () => {
        
        const [minerAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("miner"), nft2.mintAddress.toBuffer()], program.programId);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft2);
        const nftTokenAccount = await spl.getAssociatedTokenAddress(nft2.mintAddress, userKeypair2.publicKey);
        
        const tx = program.transaction.createMiner({
            accounts: {
                miner: minerAccount,
                whitelist: whitelistAccount,
                nftAta: nftTokenAccount,
                nftMetadata: nft2.metadataAddress,
                nftMint: nft2.mintAddress,
                user: userKeypair2.publicKey,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair2], {skipPreflight: true});
        console.log(`\tMiner #2 account creation transaction: ${signature}`);
        
    });
    
    
    it("Miner #1 staked!", async () => {
        
        const [minerAccount, bump1] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("miner"), nft1.mintAddress.toBuffer()], program.programId);
        const [destinationTokenAccount, bump2] = await anchor.web3.PublicKey.findProgramAddress([nft1.mintAddress.toBuffer()], program.programId);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft1);
        const sourceTokenAccount = await spl.getAssociatedTokenAddress(nft1.mintAddress, userKeypair1.publicKey);
        
        const tx = program.transaction.stakeMiner({
           
            accounts: {
                mine: mineAccount,
                miner: minerAccount,
                whitelist: whitelistAccount,
                nftSourceAta: sourceTokenAccount,
                nftDestinationAta: destinationTokenAccount,
                nftMetadata: nft1.metadataAddress,
                nftMint: nft1.mintAddress,
                user: userKeypair1.publicKey,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                
            },
            
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair1], {skipPreflight: true});
        console.log(`\tMiner #1 stake transaction: ${signature}`);
        
    });
    
    
    it("Miner #1 unstaked!", async () => {
        
        await new Promise(f => setTimeout(f, 60000));
        
        const [minerAccount, bump1] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("miner"), nft1.mintAddress.toBuffer()], program.programId);
        const [sourceTokenAccount, bump2] = await anchor.web3.PublicKey.findProgramAddress([nft1.mintAddress.toBuffer()], program.programId);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft1);
        const destinationTokenAccount = await spl.getAssociatedTokenAddress(nft1.mintAddress, userKeypair1.publicKey);
        const rewardAta = await spl.getOrCreateAssociatedTokenAccount(provider.connection, userKeypair1, rewardMintAccount, userKeypair1.publicKey);
        
        const tx = program.transaction.unstakeMiner({
            accounts: {
                mine: mineAccount,
                miner: minerAccount,
                nftDestinationAta: destinationTokenAccount,
                nftSourceAta: sourceTokenAccount,
                nftMint: nft1.mintAddress,
                rewardAta: rewardAta.address,
                rewardMint: rewardMintAccount,
                user: userKeypair1.publicKey,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                
            },
            
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair1], {skipPreflight: true});
        console.log(`\tMiner #1 unstake transaction: ${signature}`);
        
    });
    
    
    it("Whitelist account(s) removed!", async () => {
        
        const tx = new anchor.web3.Transaction();
        
        if (creatorWhitelist) {
            
            const [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), creatorKeypair.publicKey.toBuffer()], program.programId);
            
            const ix = program.instruction.removeWhitelist(creatorKeypair.publicKey, {
                accounts: {
                    program: program.programId,
                    programData: programDataAccount,
                    whitelist: whitelistAccount,
                    authority: authorityKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }
            });
            tx.add(ix);
        }
        
        for (let nft of mintWhitelist) {
            
            const [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), nft.mintAddress.toBuffer()], program.programId);
            
            const ix = program.instruction.removeWhitelist(nft.mintAddress, {
                accounts: {
                    program: program.programId,
                    programData: programDataAccount,
                    whitelist: whitelistAccount,
                    authority: authorityKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }
            });
            tx.add(ix);
        }
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [authorityKeypair], {skipPreflight: true});
        console.log(`\tRemove creator whitelist account transaction: ${signature}`);
        
    });
    
});


async function createNFT(connection: anchor.web3.Connection, creatorKeypair: Keypair, ownerKeypair: Keypair, name: string, uri: string): Promise<PublicKey> {
    const metaplex = new Metaplex(connection);
    metaplex.use(keypairIdentity(creatorKeypair));
    
    const nftTask = metaplex.nfts().create({
        name: name,
        uri: uri,
        sellerFeeBasisPoints: 500, // Represents 5.00%.
        payer: ownerKeypair,
        updateAuthority: creatorKeypair,
        tokenOwner: ownerKeypair.publicKey,
        symbol: "BSTD",
    });
    
    const output = await nftTask.run();
    
    return output;
}


async function getWhitelistAccount(creatorKeypair, program_id, creatorWhitelist, nft): Promise<PublicKey> {
    let bump;
    let whitelistAccount: PublicKey;
    if (creatorWhitelist) {
        [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), creatorKeypair.publicKey.toBuffer()], program_id);
    } else {
        [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), nft.mintAddress.toBuffer()], program_id);
    }
    return whitelistAccount
}
