/*  *****************************************************************
    ===========================TEST==================================
    *****************************************************************
*/
use crate::actors::node::Chain::Nil;
use crate::actors::node::Chain::Hosts;
use crate::actors::node::Loaded::Percents;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
enum NodeActionsTest {
    UpWorld { domain: String, port: String },
    Connect { address: String, port: String, path: String },
}

impl NodeActionsTest {
    fn init(&self) {
        if let NodeActionsTest::UpWorld { domain: dom, port: por } = self {
            println!("Run node at address:{}, port:{}", dom, por)
        } else {
            println!("Connect to node with address:{:#?}", self)
        }
    }
}

pub struct Host {
    pub address: String,
}

// mutual exclusion
pub fn mutex_simplicity_test() {
    // create mutex who point to i32 type stored 5
    let m = Mutex::new(5);
    {
        // lock mutex for this scope
        let mut num = m.lock().unwrap();
        // increase value m which stored mutex pointer to i32 type
        *num += 5;
    }
    // Now m evaluate from 5 to 10
    println!("m = {:?}", m);
    ()
}

// thanks to mechanism describe below Rust can transfer mutex value ownership into other thread.
// it is similar using Rc<T> pointer but in mutex cases
pub fn mutex_count_references() {
    // create Arc pointer (witch allow clone mutex pointer achieving transferring mutex ownership to
    // other thread) to mutex pointer to i32 type
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // call code below ten times
    for _ in 0..10 {
        // clone mutex for get ownership in this scope
        let counter = Arc::clone(&counter);
        // send actions to change value in mutex pointer to another thread
        let handle = thread::spawn(move || {
           // lock mutex and increase it to 2 poopoopoo grrrra
            let mut num = counter.lock().unwrap();
            *num += 2;
        });
        // push it in vector for prove that each value actual change
        handles.push(handle);
    }
    // wain each value to complete
    for handle in handles {
        handle.join().unwrap();
    }
    // not forget to deref counter
    println!("Result: {}", *counter.lock().unwrap());
    ()
}

pub fn channels_for_pass_msg_test() {
    // make new channel
    // tx and rx has named transmitter and receiver respectively
    let (tx, rx) = mpsc::channel();
    // cloning tx value to show implementation multiple senders -- single receiver concept
    let tx1 = tx.clone();
    // call first (after calling code) thread
    thread::spawn(move || {
        let first_thread_msgs = vec![
            String::from("Flip"),
            String::from("Phone"),
            String::from("Twerk"),
        ];

        for msg in first_thread_msgs {
            // msg will send to same receiver that send of further thread
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    })
        .join()// let's wait completing thread for right order in last println
        .unwrap();

    // call second thread who will send data to some receiver from first thread
    thread::spawn(move || {
        let second_thread_msgs = vec![
            String::from("Flextovaya"),
            String::from("Tina"),
            String::from("Kandelaki"),
        ];

        for msg in second_thread_msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // iterate through rx (receiver) who got values from two separate thread
    for receiver in rx {
        println!("Capture from channel: {}", receiver);
    }
    // we can also use this for extract value in not multiple sending:
    // let received = rx.recv().unwrap();
    ()
}

pub fn simple_concurrence_test() {
    let increasing_rewards = vec![0, 5, 10, 30, 50];
    let marker_msgs = vec![String::from("Call"), String::from("Me"), String::from("last")];
    // create and call new thread
    let new_thread = thread::spawn(move || { // move ownership from main fn
        for reward in increasing_rewards {
            println!("Rewards is {} now", reward);
            thread::sleep(Duration::from_millis(50));
        }
    });
    new_thread.join().unwrap(); // wait completing new_tread at this code point
    // validate output for prove that below output actual complete after new_tread
    for msg in marker_msgs {
        println!("This should print after displayed rewards: {}", &msg);
        thread::sleep(Duration::from_secs(1));
    }
    ()
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// create tree with parents, children, and keeping value
// while demonstrate useful strong and weak links
pub fn tree_no_leaks() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    ()
}

#[derive(Debug)]
enum Loaded {
    Percents(Rc<RefCell<i32>>, Rc<Loaded>),
    Nil,
}

pub fn interior_mutability_ref_cell_test() { // The ability to borrow mutate reference from immutable value name INTERIOR MUTABILITY
    // create a Rc pointer to RefCell pointer whom specify some value
    let alice = Rc::new(RefCell::new(10));
    // create Rc<Loaded> and clone first value from alice variable
    let cooper = Rc::new(Percents(Rc::clone(&alice), Rc::new(Loaded::Nil)));
    // add to cooper some value aid by cloning
    let similar = Percents(Rc::new(RefCell::new(20)), Rc::clone(&cooper));
    // and repeat with other value
    let pugna = Percents(Rc::new(RefCell::new(15)), Rc::clone(&cooper));
    // we can increase values from alice, because RefCell use
    // thus we can get mutate borrow from value and make some changes
    *alice.borrow_mut() += 25;
    println!("cooper after = {:?}", cooper);
    println!("similar after = {:?}", similar);
    println!("pugna after = {:?}", pugna);
}

pub fn drop_test() {
    struct CustomSmartPointer {
        data: String,
    }
    // Drop is set action witch code make when out of scope
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data {}", self.data);
        }
    }
    let c = CustomSmartPointer {
        data: String::from("-> and second"),
    };
    let d = CustomSmartPointer {
        data: String::from("-> reallocate first"),
    };
    println!("CustomSmartPointers created.");
    let z = CustomSmartPointer {
        data: String::from("-> dropping data"),
    };
    drop(z);
    println!("Force drop was done");
    ()
}

pub fn deref_and_coercion_test() {
    // custom Box pointer
    #[derive(Debug)]
    struct CustomBox<T>(T);
    // implementation for custom pointer creation
    impl<T> CustomBox<T> {
        fn new(x: T) -> CustomBox<T> {
            CustomBox(x)
        }
    }
    // implementation for release the dereference property for custom box pointer so we can deref
    // it's value for use in code
    impl<T> Deref for CustomBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let point_me = [192, 167, 2, 3];
    // create NEW pointer to type
    let new_pointer = Box::new(point_me);
    let custom_new_pointer = CustomBox(point_me);
    println!("New pointer store value {}", *&new_pointer[0]);
    println!("Custom new pointer store value {:?}", *custom_new_pointer);


    ()
}

#[derive(Debug)]
enum Chain {
    Hosts([u32; 4], Box<Chain>),
    Nil,
}

#[derive(Debug)]
enum RcChain {
    Hosts([u32; 4], Rc<RcChain>),
    Nil,
}

pub fn node_pointer_test() {
    // push value to heap
    let push = Box::new(3);
    println!("push {} at heap.", push);
    // containing elements of sequence by Box pointer
    let container = Hosts([192, 145, 23, 1], Box::new(Hosts([192, 153, 27, 87], Box::new(Hosts([195, 32, 134, 75], Box::new(Nil))))));
    println!("Container was create to use Box pointer: {:?}", container);

    // variant using Rc<> smart pointer
    let original = Rc::new(RcChain::Hosts([195, 12, 45, 126], Rc::new(RcChain::Hosts([194, 231, 45, 3], Rc::new(RcChain::Nil)))));
    let first_copy = RcChain::Hosts([0, 0, 0, 0], Rc::clone(&original));
    // Rc<> pointer able to count quantity of references to value
    println!("count after creating first_copy = {}", Rc::strong_count(&original));
    let second_copy = RcChain::Hosts([255, 255, 255, 255], Rc::clone(&original));
    println!("count after creating second_copy = {}", Rc::strong_count(&original));
    ()
}

impl Host {
    pub fn start_node_test(self) {
        // let switch_node = NodeActionsTest::UpWorld {
        //     domain: self.address,
        //     port: String::from("3325"),
        // };
        let connect_to_node = NodeActionsTest::Connect {
            address: self.address,
            port: String::from("1252"),
            path: String::from("welcome"),
        };
        connect_to_node.init();
        // switch_node.init();
    }
}
