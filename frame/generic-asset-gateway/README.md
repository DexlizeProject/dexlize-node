# generic-asset-gateway

This is a Substrate pallet which aimed to build a generic asset gateway based on bip32. It implements deposit/withdraw infrastructure for different kinds of assets.

With this generic asset gateway, it's possiable that the user could deposit/withdraw their assets to the substrate based blockchain just like the process in the centralied exchange(like Okex, Binance).

## Sub Modules

### Deposit Submodule
Deposit module will provide the ability to generate/get the deposit address only belong to this user. The address will be generated according to the bip32's path.  When the user apply for a new deposit address, it will derive from the pubkey of this asset type and record the index onchain. 

The deposit info will be provided by federated nodes. Currenlty it plants to use collective module to implement it. It will not solve the problems of fully decentralization of blockchain relay to make it more general.

### Withdraw Submodule
Withdraw module will provide the ability to withdraw assets from gateway with request -> approve/reject -> execute -> success process. 

## Deposit Process

### API

-- addRecord(origin, who: AccountId, assetType: AssetType, amount: Balance)
-- nextIndex(AssetType)
-- setXPubKey(origin, AssetType)

### Storage
-- XPubKey(Map(AssetType -> XPubKey))

## Withdraw Prcocess

### API
-- requestWithdraw(origin, assetType, toAddress, amount)

-- approveWithdraw(origin, withdrawId)   (admin requred)

-- rejectWithdraw(origin, withdrawId) (admin requried)

-- executeWithdraw(origin, withdrawId, txnInfo) (admin required)

### Storage
-- blackList(who: AccountId)
-- withdrawHistory(map(withdrawId->withdrawRecord))
