# Rust Server Backend

这个项目中对外提供的 API endpoint 大概是这几个

然后，这些 API endpoint 所绑定的 service 名字对应的 CacheKey 对应是这些,
绑定的 service 大概是这些

现在是和 Glassnode 有关的几个模块是这几个

1.  GLASSNODE_API_KEY: 可能与以下模块绑定, 而这个 GLASSNODE_API_KEY 能为系统提供链上关键数据的支持,
    CoinGecko, CryptoCompare,

    record-eth-price: 获取 ETH 价格数据。
    sync-execution-supply-deltas: 获取 ETH 供应量数据。
    update-effective-balance-sum: 获取以太坊验证者的有效余额数据。
    update-issuance-breakdown 和 update-issuance-estimate: 获取以太坊的发行量和通胀数据。
    update-validator-rewards: 获取以太坊验证者的奖励数据。

2.  Cache Key 和 API Endpoint 绑定关系
    Cache Key API Endpoint 功能描述
    AverageEthPrice /api/v2/fees/average-eth-price 获取平均以太币价格
    BaseFeeOverTime /api/v2/fees/base-fee-over-time 获取基准费用变化
    BaseFeePerGas /api/v2/fees/base-fee-per-gas 获取当前的基准费用
    BaseFeePerGasBarrier /api/v2/fees/base-fee-per-gas-barrier 获取基准费用的门槛
    BaseFeePerGasStats /api/v2/fees/base-fee-per-gas-stats 获取基准费用统计数据
    BaseFeePerGasStatsTimeFrame /api/v2/fees/base-fee-per-gas-stats（时间维度） 获取不同时间框架的基准费用统计数据
    BlobFeePerGasStats /api/v2/fees/blob-fee-per-gas-stats 获取区块链扩展费用的统计数据
    BlockLag /api/v2/fees/block-lag 获取区块延迟信息
    BurnRates /api/v2/fees/burn-rates 获取燃烧率数据
    BurnSums /api/v2/fees/burn-sums 获取燃烧总量数据
    EffectiveBalanceSum /api/v2/fees/effective-balance-sum 获取有效余额总和
    EthPrice /api/v2/fees/eth-price-stats 获取以太坊价格统计数据
    FlippeningData /api/v2/fees/flippening-data 获取翻转数据（指以太坊与其他资产间的市场份额变化）
    GaugeRates /api/v2/fees/gauge-rates 获取衡量费率数据
    IssuanceBreakdown /api/v2/fees/issuance-breakdown 获取以太坊发行分解数据
    IssuanceEstimate /api/v2/fees/issuance-estimate 获取以太坊发行估算数据
    SupplyChanges /api/v2/fees/supply-changes 获取供应变化数据
    SupplyDashboardAnalysis /api/v2/fees/supply-dashboard-analysis 获取供应分析仪表板数据
    SupplyOverTime /api/v2/fees/supply-over-time 获取供应随时间变化的情况
    SupplyParts /api/v2/fees/supply-parts 获取供应部分数据
    SupplyProjectionInputs /api/v2/fees/supply-projection-inputs 获取供应预测输入数据
    SupplySinceMerge /api/v2/fees/supply-since-merge 获取自合并以来的供应数据
    TotalDifficultyProgress /api/v2/fees/total-difficulty-progress 获取总难度进度数据
    ValidatorRewards /api/v2/fees/validator-rewards 获取验证者奖励数据

3.  BEACON_URL: Beacon 链的 URL，用于从以太坊共识层获取数据。
4.  ETHERSCAN_API_KEY: 用于与 Etherscan API 进行交互，查询以太坊网络相关数据.
5.  OPSGENIE_API_KEY: 用于 Opsgenie 服务，可能与报警和通知功能相关.
6.  GETH_URL: 与以太坊执行层的 Geth 节点连接，用于获取区块链的数据.

### GlassNode 上能够提供的数据指标:

    • BurnRates: 可能与 Glassnode 提供的链上燃烧数据相关。
    • BurnSums: 同样，可能是通过 Glassnode 提供的链上数据计算的。
    • FlippeningData: Glassnode 也可能提供有关以太坊与其他资产之间市场份额变动的数据。

计划-1: 我计划先将现在配置无法获取的 GlassNode & OPSGENIE_API_KEY 这个功能偏弱的 module 从代码中进行剥离出来,
只保留 Geth 本地服务 & Enthereum API 可用 /BEACON_URL 本地服务 对外提供的 API

代码先不用删除, 先确保这个服务对外提供的 API 都是可用的
