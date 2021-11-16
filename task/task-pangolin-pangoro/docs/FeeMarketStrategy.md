Fee Market Strategy
===

## Relay strategy

In pangolin <> pangoro bridge. All relayers can decide whether to relay a transaction. Decide on your role (Assigned Relayer or Not Assigned Relayer).
Different roles will have different benefits.

You can refer to this picture:

![Fee market rewards strategy](./fee-market-rewards-strategy.png)

So if you have good ideas, you can modify it yourself.

- `pangolin`: [feemarket.rs](../../../components/client-pangolin-s2s/src/feemarket.rs)
- `pangoro`: [feemarket.rs](../../../components/client-pangoro-s2s/src/feemarket.rs)

## Fee strategy

In addition, All relayers can also modify fee strategy to automatically update your expected handling fees.
We provided two templates.

- crazy
- reasonable

### Crazy

In this mode, Every time you find that the first one in Assigned relayers is not yourself, you will quote the other party's price -1.

### Reasonable

Query the real fee use [Subscan](https://subscan.io) and query the exchange rate of both chains. Get an appropriate price based on this.


### Custom

If you want to use different strategy, follow these steps:

1. Write your strategy to [src/fee/strategy](../src/fee/strategy)
2. Expose your strategy in [mod.rs](../src/fee/strategy/mod.rs)
3. Add your strategy to config [config.rs@UpdateFeeStrategyType](../src/config.rs)
4. Call your strategy in [fee.rs](../src/service/fee.rs)
