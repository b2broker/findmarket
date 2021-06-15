# Findmarket

The utility to find instruments in different providers.

Examples: 
```bash
# find any matches with BTC/USDT in binance and coinbase
findmarket -p binance -p coinbase -s 'BTC/USDT' 

# You also can separate args with ','
findmarket -p 'binance,coinbase' -s 'USDC'

# And look for multiple instruments at the same time
findmarket -p 'poloniex,binanceus' -s BTC -s LTC 

```

## Supported markets
- Poloniex
- Binance
- Binance.US (doesn't work with Russian IP)
- Kraken
- Coinbase
- Bitstamp (doesn't work with Russian IP)

## Todo
- [x] Search in all markets if flag -m isn't set up
- [ ] Async parsing
- [ ] Refactor code to make it more Rust-way
- [ ] Handle errors
- [ ] Add support bitfinex
- [ ] Add support gemini
- [ ] Add support bittrex
- [ ] Look for symbols with partial match
- [ ] Look for forks (list of unique assets)
- [ ] Full export
- [ ] Export finded part
- [ ] Run as web-service
