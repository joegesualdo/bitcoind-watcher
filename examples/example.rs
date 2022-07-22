use bitcoind_watcher::BitcoindWatcher;
use bitcoind_watcher::LogLine;

const FILE_TO_WATCH: &str = "/Users/joe/Library/Application Support/Bitcoin/debug.log";

fn main() {
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
    BitcoindWatcher::new(FILE_TO_WATCH)
        .on_transaction_added_to_mempool(on_transaction_added_to_mempool)
        .on_new_proof_of_work_valid_block(on_new_proof_of_work_valid_block)
        .on_new_outbound_peer_connected(on_new_outbound_peer_connected)
        .run();
}
