mod actors;
use actors::miner;

pub fn earnings() -> i32 {
    miner::get_earnings(Some("mode"))
}