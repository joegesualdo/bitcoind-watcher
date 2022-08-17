☠️⚠️ Work In Progress ⚠️☠️

# Bitcoind Watcher
> Be notified when important things happen on the Bitcoin blockchain

## Install
> Add package to Cargo.toml file
```rust
[dependencies]
bitcoind-watcher= "0.1.1"
```

## Usage:
```rust
use bitcoind_watcher::BitcoindWatcher;
use bitcoind_watcher::LogLine;

const PATH_TO_LOG_FILE: &str = "/Users/joe/Library/Application Support/Bitcoin/debug.log";

type Callback = Box<dyn Fn(LogLine) + 'static>;

let on_transaction_added_to_mempool: Callback = Box::new(|log_line| {
    println!(".....on_transaction_added_to_mempool.....");
    println!("{:?}", log_line)
});
let on_new_proof_of_work_valid_block: Callback = Box::new(|log_line| {
    println!(".....on_new_proof_of_work_valid_block.....");
    println!("{:?}", log_line)
});
let on_new_outbound_peer_connected: Callback = Box::new(|log_line| {
    println!(".....on_new_outbound_peer_connected.....");
    println!("{:?}", log_line)
});

BitcoindWatcher::new(PATH_TO_LOG_FILE)
    .on_transaction_added_to_mempool(on_transaction_added_to_mempool)
    .on_new_proof_of_work_valid_block(on_new_proof_of_work_valid_block)
    .on_new_outbound_peer_connected(on_new_outbound_peer_connected)
    .run();

```

## Related
- [bitcoind-log-parser](https://github.com/joegesualdo/bitcoind-log-parser) - Parse bitcoind log lones

## License
MIT © [Joe Gesualdo]()
