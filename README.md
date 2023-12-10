![sunade](https://github.com/supernovahs/zk-sunade/assets/22412996/363788bb-36bb-4593-bc7a-683c17de46e3)

# zk-sunade

This is an optimized , hand written Groth16 implementation in Rust using Arbitrum's Stylus [SDK](https://docs.arbitrum.io/stylus/stylus-quickstart).


## Features
- Optimized the code size to 22 KB from 26 KB.(Unoptimized)
- Minimal use of std library. Removed use of parse() method , instead used bytes to reduce size.
- Verifier is deployed on Arbitrum Stylus testnet at this [address](https://stylus-testnet-explorer.arbitrum.io/address/0x921541EeE40927601E66DbF1cD20eFA2476A97D0/contracts#address-tabs)
- Proofs are working. We are demontrating it , by using tornado cash's code as a POC. Proof working [link](https://stylus-testnet-explorer.arbitrum.io/tx/0x7ca1690c7706983a6052175bc8955937880de8b7acfa0736558fd5d701f5d36f) on stylus testnet.

## Benchmarks
- Uncompressed Wasm - 72Kb
- Compressed Wasm - 22.7Kb
- Deployment Gas - 4969475
- Runtime Gas - 256334
  

## Safety
This is experimental software and is provided on an "as is" and "as available" basis.
We do not give any warranties and will not be liable for any loss incurred through any use of this codebase.
This code is strictly for educational purposes only. Not for production use.


## Made with ❤️ by 

- [Zemse.eth](https://github.com/zemse)
- [supernovahs.eth](https://github.com/supernovahs)
