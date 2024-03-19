mod actors;

use clap::{App, AppSettings, SubCommand, Arg};
use actors::miner;
use actors::node;
use crate::actors::miner::{Miner, NewMiner};
use crate::actors::node::Host;
use std::collections::HashMap;

fn main() {
    let matches = App::new("filrust")
        .version("0.0")
        .author("Crimson Kissa")
        .about("Rust implementation of filecoin")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("miner")
            .about("Miner module")
            .arg(Arg::with_name("start")
                .help("Start miner"))
            .arg(Arg::with_name("earnings")
                .help("Show total mining earnings")
                .short("e")))
        .subcommand(SubCommand::with_name("node")
            .about("Node interaction module"))
        .get_matches();

    match matches.subcommand_name() {
        Some("miner") => {
            let pupa_miner = Miner {
                address: [10,22,10,145],
                signature: 666,
                name: "Pupa".to_string(),
            };
            let lupa_miner = Miner {
                address: vec![192,168,4,212],
                signature: 600,
                name: "Lupa".to_string(),
            };
            let enot_miner = Miner {
                address: "anti-prank.ru".to_string(),
                signature: 764,
                name: "Enot".to_string(),
            };
            // println!("{}", enot_miner.get_miner_info());
            // println!("{}", lupa_miner.get_miner_info());
            miner::test_miner_action();
            // // miner::create_miner("IT".to_string(), "Temper".to_string(),&new_miners_table);
            // // println!("{:?}", new_miners_table);
            // println!("{}", miner::miner_name_pig_latin(&enot_miner));
            // println!("{}", miner::miner_name_pig_latin(&lupa_miner));
            // let earnings_mean = miner::get_earnings_test(Some("mean"));
            // println!("Mean of miner earning variants equal {}", earnings_mean);
            // let earnings_median = miner::get_earnings_test(Some("median"));
            // println!("Median of miner earning variants equal {}", earnings_median);
            // let earnings_mode = miner::get_earnings_test(Some("mode"));
            // println!("Mode of miner earning variants equal {}", earnings_mode);
            // println!("{:#?}", miner::start_miner_test_struct());
            // println!("{}", pupa_miner.start_miner_test_method());
            // println!("{}", pupa_miner.validator_test(&lupa_miner))
        }
        Some("node") => {
            let node = Host {
                address: String::from("192.168.1.3"),
            };
            node::mutex_count_references();
            node::mutex_simplicity_test();
            node::channels_for_pass_msg_test();
            node::simple_concurrence_test();
            node::tree_no_leaks();
            node::interior_mutability_ref_cell_test();
            node::node_pointer_test();
            node::drop_test();
            node::deref_and_coercion_test();
            node.start_node_test();
        }
        None => println!("Something went wrong"),
        _ => println!("Unsupported module")
    }
}