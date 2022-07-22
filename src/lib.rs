// Helpful source: https://towardsdatascience.com/understand-your-comp&uter-system-using-logs-98139d0b5de1
extern crate logwatcher;
use bitcoind_log_parser;
use bitcoind_log_parser::LogMessage;
use logwatcher::LogWatcher;
use logwatcher::LogWatcherAction;

pub use bitcoind_log_parser::LogLine;

type Callback = Box<dyn Fn(LogLine) + 'static>;

pub struct Config {
    log_path: String,
}
pub struct BitcoindWatcher {
    config: Config,
    transaction_added_to_mempool_callback: Option<Callback>,
    new_proof_of_work_valid_block: Option<Callback>,
    new_outbound_peer_connected: Option<Callback>,
}

impl BitcoindWatcher {
    pub fn run(&self) {
        let transaction_added_to_mempool_callback_ref =
            self.transaction_added_to_mempool_callback.as_ref();
        let new_proof_of_work_valid_block_ref = self.new_proof_of_work_valid_block.as_ref();
        let new_outbound_peer_connected_ref = self.new_outbound_peer_connected.as_ref();

        let file_to_watch = &self.config.log_path;
        let mut log_watcher = LogWatcher::register(file_to_watch.to_string()).unwrap();
        log_watcher.watch(&mut move |line: String| {
            let log_line: LogLine = bitcoind_log_parser::parse(&line).unwrap();
            match &log_line.message {
                LogMessage::NewOutboundPeerConnected(_) => match new_outbound_peer_connected_ref {
                    Some(ref callback) => callback(log_line),
                    None => println!("no callback"),
                },
                LogMessage::TransactionAddedToMempool(tatmp) => {
                    match transaction_added_to_mempool_callback_ref {
                        Some(ref callback) => callback(log_line),
                        None => (),
                    }
                    //println!("{:#?}", &log_line);
                    //println!("{:#?}", tatmp.txid);
                }
                LogMessage::NewPoWValidBlock(npowvbm) => {
                    match transaction_added_to_mempool_callback_ref {
                        Some(ref callback) => callback(log_line),
                        None => (),
                    }
                    //println!("{:#?}", &log_line);
                }
                LogMessage::Unknown { raw: _raw } => {
                    //println!("{:#?}", &log_line);
                }
                _ => {
                    //println!("{}", line)
                }
            }
            LogWatcherAction::None
        });
    }
    pub fn new(log_path: &str) -> Self {
        let config = Config {
            log_path: log_path.to_string(),
        };
        BitcoindWatcher {
            config,
            transaction_added_to_mempool_callback: None,
            new_proof_of_work_valid_block: None,
            new_outbound_peer_connected: None,
        }
    }
    pub fn on_transaction_added_to_mempool(mut self, callback: Callback) -> Self {
        self.transaction_added_to_mempool_callback = Some(callback);
        self
    }
    pub fn on_new_proof_of_work_valid_block(mut self, callback: Callback) -> Self {
        self.new_proof_of_work_valid_block = Some(callback);
        self
    }
    pub fn on_new_outbound_peer_connected(mut self, callback: Callback) -> Self {
        self.new_outbound_peer_connected = Some(callback);
        self
    }
}
