# Dexlize

This module is core module of [Dexlize Project](https://github.com/DexlizeProject). It provides the ability to create different types of dex and unified liquidation interface.

To be clear, Dexlize is not just a decentralize exchange, it's aimed to build a dex agreegator. It's more like dex.ag in the ethereum ecosystem and it have more functions.

## Arch
### APIs

- CreateMarket(symbol, base, type)  - Create type sepecifed dex market
- Trade(fromAssetId, toAssetId, fromAmount, exchanges, offsets, limitAmount, tradeType)

## In the future

Dexlize will combine with the [XCMP](https://wiki.polkadot.network/docs/en/learn-crosschain) to provide the ability to have cross-chain liquidity.


