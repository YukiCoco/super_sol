# super.exchange 交易量工具
小手一点，关注我的 [TWITTER](https://x.com/SteinAmour)  
## 说明
在 super.exchange 平台对刷交易，将买入和卖出交易放在同一个 TX 中。  
安装 [ rust 编译工具链](https://www.rust-lang.org/tools/install) 自行编译。  
供学习使用，平台跑路、修改合约风险自担，不要使用主要钱包的私钥。

## 使用
```bash
./super_sol -p  <对刷钱包的私钥> --rpc-url https://mainnet.helius-rpc.com/?api-key=b99e4ede-a932-4be1-b84a-fadc12d0302b --buy-amount <购买数量> --max-sol <最大 SOL 支付数量> --jito-url https://slc.mainnet.block-engine.jito.wtf
```
