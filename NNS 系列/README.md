# NNS 系列讲解

**背景**
* 整个 Internet Computer 由很多相互独立子网组成。且子网数量可动态增长。
* 这么多子网怎么协同？除了子网之间直接通信之外，还有一些需要所有子网达成共识的信息，比如 Canister id 在所有子网里面应该是唯一。这些“全局”信息就保存在 NNS 子网中。
* 重要功能，比如 ICP 的管理，投票治理，ICP 和 Cycle 的转换，用户身份等，也在 NNS 子网中。

**NNS 构成**
* 37 个节点（可动态调整）
* 共 10 个 Canister：registry、governance、ledger、root、cycles-minting、lifeline、genesis-token、identity、nns-ui、archive-node

| **Name**       | **Principal/Canister id**    | **Controller** | **Functions**         |
| -------------- | ---------------------------- | -------------- | --------------------- |
| registry       | rwlgt-iiaaa-aaaaa-aaaaa-cai  | root           | 注册保存全局信息         |
| governance     | rrkah-fqaaa-aaaaa-aaaaq-cai  | root           | 神经元的管理、投票       ｜
| ledger         | ryjl3-tyaaa-aaaaa-aaaba-cai  | root           | 实现 ICP 的功能         ｜
| root           | r7inp-6aaaa-aaaaa-aaabq-cai  | lifeline       | 控制其他 NNS canisters ｜
| cycles-minting | rkp4c-7iaaa-aaaaa-aaaca-cai  | root           | 将 ICP 兑换成 Cycles    |
| lifeline       | rno2w-sqaaa-aaaaa-aaacq-cai  | root           | 控制 root canister     |
| genesis-token  | renrk-eyaaa-aaaaa-aaada-cai  | root           | 创世纪时的 token 分配    |
| identity       | rdmx6-jaaaa-aaaaa-aaadq-cai  | root           | 用户身份抽象             |
| nns-ui         | qoctq-giaaa-aaaaa-aaaea-cai  | root           | 方便用户使用 NNS 功能    |
| archive-node   | qjdve-lqaaa-aaaaa-aaaeq-cai  | root           | 存储 Ledger 的历史记录  |

Canister Id 的生成：
```rust
pub const REGISTRY_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 0;
pub const GOVERNANCE_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 1;
pub const LEDGER_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 2;
pub const ROOT_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 3;
pub const CYCLES_MINTING_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 4;
pub const LIFELINE_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 5;
pub const GENESIS_TOKEN_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 6;
pub const IDENTITY_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 7;
pub const NNS_UI_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 8;

pub const REGISTRY_CANISTER_ID: CanisterId =
		CanisterId::from_u64(REGISTRY_CANISTER_INDEX_IN_NNS_SUBNET);
```

## 目录

**[NNS 系列之 Registry](Registry.md)**
**[NNS 系列之 Governance](Governance.md)**
**[NNS 系列之 Ledger](Ledger.md)**
**[NNS 系列之 Root](Root.md)**
**[NNS 系列之 Cycles-minting](Cycles-minting.md)**
**[NNS 系列之 Lifeline](Lifeline.md)**
**[NNS 系列之 Genesis-token](Genesis-token.md)**
**[NNS 系列之 Identity](Identity.md)**
**[NNS 系列之 NNS-ui](NNS-ui.md)**
**[NNS 系列之 Archive-node](Archive-node.md)**

