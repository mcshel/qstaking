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
    
    const adminKeypair = Keypair.generate();
    const managerKeyepair = Keypair.generate();
    const falseManagerKeyepair = Keypair.generate();
    const creatorKeypair = Keypair.generate();
    const userKeypair1 = Keypair.generate();
    const userKeypair2 = Keypair.generate();
    
    let tokenMetadataProgram = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');
    let programDataAccount: PublicKey;
    let adminSettingsAccount: PublicKey;
    let mineAccount: PublicKey;
    let lootAccount: PublicKey;
    let lootProceedsAccount: PublicKey;
    let rewardMintAccount: PublicKey;
    
    let nft1 = null;
    let nft2 = null;
    
    const creatorWhitelist = false;
    const mintWhitelist = [];
    
    
    const mine_rate = 100;
    const mine_price = 1000000;
    const mine_cooldown = 3600;
    
    const loot_duration = 100;
    const loot_price = 1000000;
    const loot_cooldown = 3600;
    
    before( async () => {
        
        const airdropSignature0 = await provider.connection.requestAirdrop(adminKeypair.publicKey, 1e9);
        await provider.connection.confirmTransaction(airdropSignature0);
        
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
        [adminSettingsAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("admin")], program.programId);
        [mineAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("mine")], program.programId);
        [lootAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("loot")], program.programId);
        [lootProceedsAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("proceeds"), lootAccount.toBuffer()], program.programId);
        
        if (!await provider.connection.getAccountInfo(mineAccount)) {
            rewardMintAccount = await spl.createMint(provider.connection, authorityKeypair, mineAccount, authorityKeypair.publicKey, 6);
        } else {
            const mineAccountData = await program.account.mine.fetch(mineAccount);
            rewardMintAccount = mineAccountData.mint;
        }
        
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
    
    
    it('Set staking program admin!', async () => {
        
        const adminSettingsInfo = await provider.connection.getAccountInfo(adminSettingsAccount);
        if (adminSettingsInfo) {
            
            const tx = await program.transaction.setAdmin(adminKeypair.publicKey, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    program: program.programId,
                    programData: programDataAccount,
                    authority: authorityKeypair.publicKey,
                },
            });
            
            const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [authorityKeypair], {skipPreflight: true});
            console.log(`\tSet admin settings transaction: ${signature}`);
            
            const adminSettingsAccountData = await program.account.adminSettings.fetch(adminSettingsAccount);
            assert.equal(adminSettingsAccountData.adminKey.toString(), adminKeypair.publicKey.toString());
            
        } else {
            
            const tx = await program.transaction.initAdmin(adminKeypair.publicKey, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    program: program.programId,
                    programData: programDataAccount,
                    authority: authorityKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                },
            });
            const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [authorityKeypair], {skipPreflight: true});
            console.log(`\tInit admin settings transaction: ${signature}`);
            
            const adminSettingsAccountData = await program.account.adminSettings.fetch(adminSettingsAccount);
            assert.equal(adminSettingsAccountData.adminKey.toString(), adminKeypair.publicKey.toString());
        }
        
        const adminSettingsData = await program.account.adminSettings.fetch(adminSettingsAccount);
        assert.equal(adminSettingsData.adminKey.toString(), adminKeypair.publicKey.toString());
        
    });
    
    
    it("Whitelist account(s) created!", async () => {
        
        const tx = new anchor.web3.Transaction();
        
        if (creatorWhitelist) {
            const [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), creatorKeypair.publicKey.toBuffer()], program.programId);
            
            const ix = program.instruction.addWhitelist(creatorKeypair.publicKey, new anchor.BN(1), {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    whitelist: whitelistAccount,
                    authority: adminKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
//                     rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }
            });
            tx.add(ix);
        }
        
        for (let nft of mintWhitelist) {
            const [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), nft.mintAddress.toBuffer()], program.programId);
        
            const ix = program.instruction.addWhitelist(nft.mintAddress, new anchor.BN(0), {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    whitelist: whitelistAccount,
                    authority: adminKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
//                     rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                }
            });
            tx.add(ix);
        }
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [adminKeypair], {skipPreflight: true});
        console.log(`\tAdd creator whitelist account transaction: ${signature}`);
        
    });
    
    
    it("Character #1 account initialized!", async () => {
        
        const [characterAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("character"), nft1.mintAddress.toBuffer()], program.programId);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft1);
        const nftTokenAccount = await spl.getAssociatedTokenAddress(nft1.mintAddress, userKeypair1.publicKey);
        
        const tx = program.transaction.initCharacter({
            accounts: {
                character: characterAccount,
                whitelist: whitelistAccount,
                nftAta: nftTokenAccount,
                nftMetadata: nft1.metadataAddress,
                nftMint: nft1.mintAddress,
                user: userKeypair1.publicKey,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair1], {skipPreflight: true});
        console.log(`\tCharacter #1 account initialization transaction: ${signature}`);
        
    });
    
    
    it("Character #2 account initialized!", async () => {
        
        const [characterAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("character"), nft2.mintAddress.toBuffer()], program.programId);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft2);
        const nftTokenAccount = await spl.getAssociatedTokenAddress(nft2.mintAddress, userKeypair2.publicKey);
        
        const tx = program.transaction.initCharacter({
            accounts: {
                character: characterAccount,
                whitelist: whitelistAccount,
                nftAta: nftTokenAccount,
                nftMetadata: nft2.metadataAddress,
                nftMint: nft2.mintAddress,
                user: userKeypair2.publicKey,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair2], {skipPreflight: true});
        console.log(`\tCharacter #2 account initialization transaction: ${signature}`);
        
    });
    
    
    it("Staking pools initialized!", async () => {
        
        if (await provider.connection.getAccountInfo(mineAccount) && await provider.connection.getAccountInfo(lootAccount)) {
            
            const tx = new anchor.web3.Transaction();
            
            const ix1 = program.instruction.setMineManager(managerKeyepair.publicKey, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    mine: mineAccount,
                    authority: adminKeypair.publicKey,
                }
            });
            tx.add(ix1);
            
            const ix2 = program.instruction.setLootManager(managerKeyepair.publicKey, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    loot: lootAccount,
                    authority: adminKeypair.publicKey,
                }
            });
            tx.add(ix2);
            
            const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [adminKeypair], {skipPreflight: true});
            console.log(`\tSet pool manager transaction: ${signature}`);
            
            const mineAccountData = await program.account.mine.fetch(mineAccount);
            assert.equal(mineAccountData.manager.toString(), managerKeyepair.publicKey.toString());
            
            const lootAccountData = await program.account.loot.fetch(lootAccount);
            assert.equal(lootAccountData.manager.toString(), managerKeyepair.publicKey.toString());
            
            
        } else {
        
            const tx = program.transaction.initPools(managerKeyepair.publicKey, managerKeyepair.publicKey, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    mine: mineAccount,
                    loot: lootAccount,
                    lootProceeds: lootProceedsAccount,
                    mint: rewardMintAccount,
                    authority: adminKeypair.publicKey,
                    tokenProgram: spl.TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY
                },
            });
            
            const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [adminKeypair], {skipPreflight: true});
            console.log(`\tInitialize mine transaction: ${signature}`);
            
            const mineAccountData = await program.account.mine.fetch(mineAccount);
            assert.equal(mineAccountData.locked, true);
            assert.equal(mineAccountData.manager.toString(), managerKeyepair.publicKey.toString());
            assert.equal(mineAccountData.mint.toString(), rewardMintAccount.toString());
            assert.equal(mineAccountData.rate, 0);
            assert.equal(mineAccountData.price, 0);
            assert.equal(mineAccountData.cooldown, 0);
            assert.equal(mineAccountData.stakedCharacters, 0);
            assert.equal(mineAccountData.stakedPoints, 0);
            assert.equal(mineAccountData.accruedRewards, 0);
            assert.equal(mineAccountData.accruedTimestamp, 0);
            
            const lootAccountData = await program.account.loot.fetch(lootAccount);
            assert.equal(lootAccountData.locked, true);
            assert.equal(lootAccountData.manager.toString(), managerKeyepair.publicKey.toString());
            assert.equal(lootAccountData.fund, 0);
            assert.equal(lootAccountData.duration, 0);
            assert.equal(lootAccountData.rate, 0);
            assert.equal(lootAccountData.price, 0);
            assert.equal(lootAccountData.cooldown, 0);
            assert.equal(lootAccountData.stakedCharacters, 0);
            assert.equal(lootAccountData.stakedPoints, 0);
            assert.equal(lootAccountData.accruedRewards, 0);
            assert.equal(lootAccountData.accruedTimestamp, 0);
        }
    });
    
    
    /*
    it("Mine is initialized!", async () => {
        
        if (await provider.connection.getAccountInfo(mineAccount)) {
            
            const tx = program.transaction.setMineManager(managerKeyepair.publicKey, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    mine: mineAccount,
                    authority: adminKeypair.publicKey,
                }
            });
            
            const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [adminKeypair], {skipPreflight: true});
            console.log(`\tSet mine manager transaction: ${signature}`);
            
            const mineAccountData = await program.account.mine.fetch(mineAccount);
            assert.equal(mineAccountData.manager.toString(), managerKeyepair.publicKey.toString());
            
            
        } else {
        
            const tx = program.transaction.initMine(managerKeyepair.publicKey, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    mine: mineAccount,
                    mint: rewardMintAccount,
                    authority: adminKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                },
            });
            
            const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [adminKeypair], {skipPreflight: true});
            console.log(`\tInitialize mine transaction: ${signature}`);
            
            const mineAccountData = await program.account.mine.fetch(mineAccount);
            assert.equal(mineAccountData.locked, true);
            assert.equal(mineAccountData.manager.toString(), managerKeyepair.publicKey.toString());
            assert.equal(mineAccountData.mint.toString(), rewardMintAccount.toString());
            assert.equal(mineAccountData.rate, 0);
            assert.equal(mineAccountData.price, 0);
            assert.equal(mineAccountData.cooldown, 0);
            assert.equal(mineAccountData.stakedMiners, 0);
            assert.equal(mineAccountData.stakedPoints, 0);
            assert.equal(mineAccountData.accruedRewards, 0);
            //assert.equal(mineAccountData.accrued_timesstamp, 0);
        }
    });
    */
    
    
    it("Mine parameters set!", async () => {
        
        const tx = program.transaction.setMineParameters(new anchor.BN(mine_rate), new anchor.BN(mine_price), new anchor.BN(mine_cooldown), {
            accounts: {
                mine: mineAccount,
                manager: managerKeyepair.publicKey,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [managerKeyepair], {skipPreflight: true});
        console.log(`\tSet mine parameters transaction: ${signature}`);
        
        const mineAccountData = await program.account.mine.fetch(mineAccount);
        assert.equal(mineAccountData.rate, mine_rate);
        assert.equal(mineAccountData.price, mine_price);
        assert.equal(mineAccountData.cooldown, mine_cooldown);
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
        
        const expectedReward = mineAccountData1.accruedRewards.toNumber() + mineAccountData1.rate.toNumber() * (mineAccountData2.accruedTimestamp.toNumber() - mineAccountData1.accruedTimestamp.toNumber());
        assert.equal(mineAccountData2.accruedRewards, expectedReward);
    });
    
    
    it("Loot parameters set!", async () => {
        
        const tx = program.transaction.setLootParameters(new anchor.BN(loot_duration), new anchor.BN(loot_price), new anchor.BN(loot_cooldown), {
            accounts: {
                loot: lootAccount,
                manager: managerKeyepair.publicKey,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [managerKeyepair], {skipPreflight: true});
        console.log(`\tSet loot parameters transaction: ${signature}`);
        
        const lootAccountData = await program.account.loot.fetch(lootAccount);
        assert.equal(lootAccountData.duration, loot_duration);
        assert.equal(lootAccountData.price, loot_price);
        assert.equal(lootAccountData.cooldown, loot_cooldown);
    });
    
    
    it("Loot lock set!", async () => {
        
        const tx = program.transaction.setLootLock(false, {
            accounts: {
                loot: lootAccount,
                manager: managerKeyepair.publicKey,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [managerKeyepair], {skipPreflight: true});
        console.log(`\tSet loot lock transaction: ${signature}`);
        
        const lootAccountData = await program.account.loot.fetch(lootAccount);
        assert.equal(lootAccountData.locked, false);
    });
    
    
    it("Loot updated!", async () => {
        
        const keypair = Keypair.generate();
        const airdropSignature = await provider.connection.requestAirdrop(keypair.publicKey, 1e9);
        await provider.connection.confirmTransaction(airdropSignature);
        
        const tx = program.transaction.updateLoot({
            accounts: {
                loot: lootAccount,
            },
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [managerKeyepair], {skipPreflight: true});
        console.log(`\tUpdate loot transaction: ${signature}`);
        
        const lootAccountData = await program.account.loot.fetch(lootAccount);
        assert.equal(lootAccountData.accruedRewards, 0);
    });
    
    
    it("User #1 miner staked!", async () => {
        
        const [characterAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("character"), nft1.mintAddress.toBuffer()], program.programId);
        const [nftEditionAccount, bump2] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("metadata"), tokenMetadataProgram.toBuffer(), nft1.mintAddress.toBuffer(), Buffer.from("edition")], tokenMetadataProgram);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft1);
        const nftTokenAccount = await spl.getAssociatedTokenAddress(nft1.mintAddress, userKeypair1.publicKey);
        
        const tx = program.transaction.stakeMine({
           
            accounts: {
                mine: mineAccount,
                character: characterAccount,
                whitelist: whitelistAccount,
                nftAta: nftTokenAccount,
                nftEdition: nftEditionAccount,
                nftMetadata: nft1.metadataAddress,
                nftMint: nft1.mintAddress,
                user: userKeypair1.publicKey,
                tokenMetadataProgram: tokenMetadataProgram,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair1], {skipPreflight: true});
        console.log(`\tUser #1 stake miner transaction: ${signature}`);
        
        const nftTokenAccountData = await spl.getAccount(provider.connection, nftTokenAccount);
        assert.equal(nftTokenAccountData.delegate.toString(), mineAccount.toString());
        assert.equal(nftTokenAccountData.delegatedAmount, 1);
        assert.equal(nftTokenAccountData.isFrozen, true);
        
        const characterAccountData = await program.account.character.fetch(characterAccount);
        assert.equal(characterAccountData.staked, 1);
        assert.ok(characterAccountData.stakedTimestamp > 0);
        assert.ok(characterAccountData.stakedPeg > 0);
        
    });
    
    
    it("User #2 looter staked!", async () => {
        
        const [characterAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("character"), nft2.mintAddress.toBuffer()], program.programId);
        const [nftEditionAccount, bump2] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("metadata"), tokenMetadataProgram.toBuffer(), nft2.mintAddress.toBuffer(), Buffer.from("edition")], tokenMetadataProgram);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft2);
        const nftTokenAccount = await spl.getAssociatedTokenAddress(nft2.mintAddress, userKeypair2.publicKey);
        
        const tx = program.transaction.stakeLoot({
           
            accounts: {
                loot: lootAccount,
                character: characterAccount,
                whitelist: whitelistAccount,
                nftAta: nftTokenAccount,
                nftEdition: nftEditionAccount,
                nftMetadata: nft2.metadataAddress,
                nftMint: nft2.mintAddress,
                user: userKeypair2.publicKey,
                tokenMetadataProgram: tokenMetadataProgram,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair2], {skipPreflight: true});
        console.log(`\tUser #2 stake looter transaction: ${signature}`);
        
        const nftTokenAccountData = await spl.getAccount(provider.connection, nftTokenAccount);
        assert.equal(nftTokenAccountData.delegate.toString(), lootAccount.toString());
        assert.equal(nftTokenAccountData.delegatedAmount, 1);
        assert.equal(nftTokenAccountData.isFrozen, true);
        
        const characterAccountData = await program.account.character.fetch(characterAccount);
        assert.equal(characterAccountData.staked, 2);
        assert.ok(characterAccountData.stakedTimestamp > 0);
        assert.equal(characterAccountData.stakedPeg, 0);
        
    });
    
    
    it("User #1 miner unstaked!", async () => {
        
        await new Promise(f => setTimeout(f, 10000));
        
        const [characterAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("character"), nft1.mintAddress.toBuffer()], program.programId);
        const [nftEditionAccount, bump2] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("metadata"), tokenMetadataProgram.toBuffer(), nft1.mintAddress.toBuffer(), Buffer.from("edition")], tokenMetadataProgram);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft1);
        const nftTokenAccount = await spl.getAssociatedTokenAddress(nft1.mintAddress, userKeypair1.publicKey);
        const rewardAta = await spl.getOrCreateAssociatedTokenAccount(provider.connection, userKeypair1, rewardMintAccount, userKeypair1.publicKey);
        
        const tx = program.transaction.unstakeMine({
            accounts: {
                mine: mineAccount,
                loot: lootAccount,
                character: characterAccount,
                lootProceeds: lootProceedsAccount,
                nftAta: nftTokenAccount,
                nftEdition: nftEditionAccount,
                nftMint: nft1.mintAddress,
                rewardAta: rewardAta.address,
                rewardMint: rewardMintAccount,
                user: userKeypair1.publicKey,
                tokenMetadataProgram: tokenMetadataProgram,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair1], {skipPreflight: true});
        console.log(`\tUser #1 unstake miner transaction: ${signature}`);
        
        const nftTokenAccountData = await spl.getAccount(provider.connection, nftTokenAccount);
        assert.equal(nftTokenAccountData.delegate, null);
        assert.equal(nftTokenAccountData.delegatedAmount, 0);
        assert.equal(nftTokenAccountData.isFrozen, false);
        
        const characterAccountData = await program.account.character.fetch(characterAccount);
        assert.equal(characterAccountData.staked, 0);
        assert.equal(characterAccountData.stakedTimestamp, 0);
        assert.ok(characterAccountData.mineCooldownTimestamp > 0);
        assert.ok(characterAccountData.miningRewards > 0);
        assert.ok(characterAccountData.experience > 0);
        
        console.log(await program.account.loot.fetch(lootAccount));
        
    });
    
    
    it("User #2 looter unstaked!", async () => {
        
        await new Promise(f => setTimeout(f, 10000));
        
        const [characterAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("character"), nft2.mintAddress.toBuffer()], program.programId);
        const [nftEditionAccount, bump2] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("metadata"), tokenMetadataProgram.toBuffer(), nft2.mintAddress.toBuffer(), Buffer.from("edition")], tokenMetadataProgram);
        const whitelistAccount = await getWhitelistAccount(creatorKeypair, program.programId, creatorWhitelist, nft2);
        const nftTokenAccount = await spl.getAssociatedTokenAddress(nft2.mintAddress, userKeypair2.publicKey);
        const rewardAta = await spl.getOrCreateAssociatedTokenAccount(provider.connection, userKeypair2, rewardMintAccount, userKeypair2.publicKey);
        
        const tx = program.transaction.unstakeLoot({
            accounts: {
                loot: lootAccount,
                character: characterAccount,
                lootProceeds: lootProceedsAccount,
                nftAta: nftTokenAccount,
                nftEdition: nftEditionAccount,
                nftMint: nft2.mintAddress,
                rewardAta: rewardAta.address,
                user: userKeypair2.publicKey,
                tokenMetadataProgram: tokenMetadataProgram,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            
        });
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [userKeypair2], {skipPreflight: true});
        console.log(`\tUser #2 unstake looter transaction: ${signature}`);
        
        
        const nftTokenAccountData = await spl.getAccount(provider.connection, nftTokenAccount);
        assert.equal(nftTokenAccountData.delegate, null);
        assert.equal(nftTokenAccountData.delegatedAmount, 0);
        assert.equal(nftTokenAccountData.isFrozen, false);
        
        const characterAccountData = await program.account.character.fetch(characterAccount);
        assert.equal(characterAccountData.staked, 0);
        assert.equal(characterAccountData.stakedTimestamp, 0);
        assert.ok(characterAccountData.lootCooldownTimestamp > 0);
        assert.ok(characterAccountData.lootingRewards > 0);
        assert.ok(characterAccountData.experience > 0);
        
    });

    
    it("Whitelist account(s) removed!", async () => {
        
        const tx = new anchor.web3.Transaction();
        
        if (creatorWhitelist) {
            
            const [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), creatorKeypair.publicKey.toBuffer()], program.programId);
            
            const ix = program.instruction.removeWhitelist(creatorKeypair.publicKey, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    whitelist: whitelistAccount,
                    authority: adminKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }
            });
            tx.add(ix);
        }
        
        for (let nft of mintWhitelist) {
            
            const [whitelistAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("whitelist"), nft.mintAddress.toBuffer()], program.programId);
            
            const ix = program.instruction.removeWhitelist(nft.mintAddress, {
                accounts: {
                    adminSettings: adminSettingsAccount,
                    whitelist: whitelistAccount,
                    authority: adminKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }
            });
            tx.add(ix);
        }
        
        const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [adminKeypair], {skipPreflight: true});
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
