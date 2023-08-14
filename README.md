# NFT Renter Dapp

## Install
```
    npm install or yarn install
```

## Build
```
    anchor build
```

## Test
```
    anchor test    
```

## Notice: 
- Remember to change your network to local and change the keypair to your keypair file.

Change your network to localhost
```
    solana config set --url localhost
    solana config get
    solana airdrop 2
```

Modify your keypair
In Anchor.toml, at [provider], change wallet property to your identity, for ex: "home/[user]/.config/solana/jd.json", change [user] with you current user