/*  *****************************************************************
    ===========================TEST==================================
    *****************************************************************
*/
use std::collections::HashMap;
use std::ptr::null;
use std::io::{Bytes, Write, Error, Read};
use std::fs::File;
use std::io::ErrorKind;
use std::fmt::Display;
use std::cmp::Eq;

pub trait Miners {
    fn get_address(&self) -> String;
}

impl Miners for Miner<Vec<u32>> {
    fn get_address(&self) -> String {
        format!("{}.{}.{}.{}",
                self.address[0],
                self.address[1],
                self.address[2],
                self.address[3], )
    }
}

impl Miners for Miner<String> {
    fn get_address(&self) -> String {
        format!("{}", self.address)
    }
}

impl<T> Miners for NewMiner<'_, T> {
    fn get_address(&self) -> String {
        format!("Miner don't activate yet, request new miner's address {} from {} department", self.name, self.department)
    }
}

#[derive(Debug)]
pub struct Miner<T> {
    pub address: T,
    pub signature: u32,
    pub name: String,
}

pub struct NewMiner<'a, T> {
    name: String,
    department: String,
    rate: &'a T,
}

#[derive(Debug)]
pub struct MinersTable {
    pub table: HashMap<String, Vec<String>>,
}


// impl NewMiner
impl<T> NewMiner<'_, T>
    where
        T: Display,
{
    // add is set new miner to new job department
    fn add(&self, mut base: HashMap<String, Vec<String>>) -> MinersTable {
        // CLONE base to dist (dist and base equal now)
        let mut dist = base.clone();
        // CLONE len base to len_base (is not ownership here)
        let mut len_base = base.clone().len();
        // MOVE base to table.table to MinersTable (ownership WORKING here)
        let mut table = MinersTable {
            table: base, // OWNERSHIP occasioning here, base NOT available now
        };
        // MOVE dist to spare_table.table to MinersTable (ownership WORKING here)
        let mut spare_table = MinersTable {
            table: dist // OWNERSHIP occasioning here, dist NOT available now
        };
        if len_base == 0 {
            // table OWNERSHIP
            table.table.insert(self.department.clone(), vec![self.name.clone()]);
        } else {
            let mut counter: usize = 0;
            for (key, value) in &mut table.table { // table OWNERSHIP (conditional 'if len_base == 0'
                // placed out of this SCOPE, therefore we can use table here
                counter += 1;
                if *key == self.department {
                    value.push(self.name.clone());
                } else if counter == len_base {
                    // spare_table OWNERSHIP. It in this SCOPE
                    spare_table.table.insert(self.department.clone(), vec![self.name.clone()]).clone();
                    // return spare_table rather table in THIS CONDITIONAL
                    return spare_table;
                }
            }
        }
        // return table instead spare_table since scope not get into 'else if counter == len_base'
        table
    }

    pub fn write_miner(&self, path: &String) {
        let mut f = File::open(&path);
        // example with verbose error handle
        let mut f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(&path) {
                    Ok(create_file) => create_file,
                    Err(err) => panic!("Cannot create file: {:?}", err),
                },
                other_error => {
                    panic!("Cannot open file: {:?}", other_error)
                }
            },
        };
        let write_miner = format!("{}:{}", &self.name, &self.department);
        f.write_all(write_miner.as_ref()).expect("Cannot write new miner")
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn department(&self) -> &String {
        &self.department
    }
}

impl<'a> NewMiner<'a, i32> {
    pub fn win_miner_rate(&self, miner: &'a NewMiner<i32>) -> i32 {
        let difference: i32;
        if self.rate > miner.rate {
            difference = self.rate - miner.rate;
            println!("Miner {} win this rate completition", self.name);
        } else {
            difference = miner.rate - self.rate;
            println!("Miner {} loose this rate completition to {} miner", self.name, miner.name);
        }
        difference
    }
}

pub fn read_miners_from_file(path: &String) -> Result<String, Error> {
    let f = File::open(&path);
    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut reader = String::new();

    f.read_to_string(&mut reader)?;
    Ok(reader)
}

impl MinersTable {
    fn get_employers_by_department(&self, department: &String) {
        let mut miners_list = self.table.clone();
        let miners_list_size = self.table.clone().len();
        let mut counter: usize = 0;
        for (key, value) in &mut miners_list {
            // match *key {
            //     Some(String::from("Sales"))=> println!("Employers miners in {} department:\n{:?}", &department, self.table.get(&department)),
            //     _ => println!("All miner employers:\n{:?}", self.table),
            // }
            counter += 1;
            if *key == *department {
                println!("Employers in {} department:\n{:?}", &department, self.table.get(department));
            } else if counter == miners_list_size {
                println!("All miner employers:\n{:?}", self.table);
            }
        }
    }
}

struct Cacher<T>
    where
        T: Fn(NewMiner<i32>) -> i32,
{
    calculation: T,
    value: Option<i32>,
}

// Cacher cache first calculation and avoid make calculation if it exist
impl<T> Cacher<T>
    where
        T: Fn(NewMiner<i32>) -> i32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, miner: NewMiner<i32>) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(miner);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn test_miner_action() {
    let test_string_vec = vec![String::from("Prada"), String::from("Gucci"), String::from("Tvo95uka")];
    let mut new_table = MinersTable {
        table: HashMap::new(),
    };
    let older_miner = Miner {
        name: "Older".to_string(),
        address: "antiprank.ru".to_string(),
        signature: 0,
    };
    let lost_dns_miner = Miner {
        name: "Loster".to_string(),
        address: vec![193, 145, 65, 78],
        signature: 0,
    };
    let jerrakka = NewMiner {
        name: "Jerrakka".to_string(),
        department: "Sales".to_string(),
        rate: &"administrator".to_string(),
    };
    let bubirija = NewMiner {
        name: "Bubirija".to_string(),
        department: "Sales".to_string(),
        rate: &224,
    };
    let zijujakk = NewMiner {
        name: "Zijujakk".to_string(),
        department: "IT".to_string(),
        rate: &213,
    };
    let fillerseek = NewMiner {
        name: "Fillerseek".to_string(),
        department: "IT".to_string(),
        rate: &93,
    };
    let groovie = NewMiner {
        name: "Groovie".to_string(),
        department: "Flower".to_string(),
        rate: &87,
    };

    // iterator NEXT method example
    iterators_next_example(test_string_vec);

    // closures example
    let mut miner_power = Cacher::new(|miner| -> i32 {
        if miner.department == "IT" {
           return miner.rate * 512;
        }
        miner.rate * 1024
    });
    println!("Groovie power: {}", miner_power.value(groovie));
    // cannot rewrite (store previous value)
    println!("Zijujakk power: {}", miner_power.value(zijujakk));

    println!("{}", older_miner.get_address());
    println!("{}", lost_dns_miner.get_address());
    println!("{}", bubirija.get_address());
    // println!("{}", fillerseek.win_miner_rate(&groovie));
    // println!("{:?}", read_miners_from_file(&"new_miner.txt".to_string()));
    // bubirija.write_miner(&"new_miner.txt".to_string());
    // let add_jerraka_job = jerrakka.add(new_miners.table);
    // let add_bubirija_job = bubirija.add(add_jerraka_job.table);
    // let add_zijujakk_job = zijujakk.add(add_bubirija_job.table);
    // let add_fillerseek_job = fillerseek.add(add_zijujakk_job.table);
    // let add_groovie_job = groovie.add(new_table.table);
    // add_groovie_job.get_employers_by_department(&String::from("Sales"));
    ()
}

// iterator using next method
fn iterators_next_example(vector: Vec<String>) -> Result<(), &'static str>{
    if vector.len() < 2 {
        return Err("Not enough elements of vector");
    }
    let mut vector_iter = vector.iter();
    println!("First value in vector using NEXT is {:?}", vector_iter.next());
    println!("Second value in vector using NEXT is {:?}", vector_iter.next());
    Ok(())

}

impl Miner<String> {
    pub fn get_miner_info(&self) -> String {
        format!("{}:{}", self.address, self.signature)
    }
    pub fn validator_test(&self, validate: &Miner<String>) -> bool {
        let for_validate: String = format!("{}{}", validate.address, validate.signature);
        self.get_miner_info() == for_validate
    }
}

impl Miner<Vec<u32>> {
    pub fn get_miner_info(&self) -> String {
        let address = format!("{}.{}.{}.{}", self.address[0], self.address[1],
                              self.address[2], self.address[3]);
        if self.address[3] > 254 {
            format!("Cannot set 255 bit at last in address {}", address);
            ()
        }
        format!("{}:{}", address, self.signature)
    }
}

pub fn start_miner_test_struct() -> String {
    let miner_lupa = Miner {
        address: 100,
        signature: 666,
        name: "lupa".to_string(),
    };
    format!("Miner Lupa has fields: {:#?}", miner_lupa)
}

pub fn miner_name_pig_latin(miner: &Miner<String>) -> String {
    let miner_name = miner.name.to_lowercase();
    let byte_name = miner_name.as_bytes();
    let isVowel = "aeiouAEIOU".to_string();
    let check_vowel = {
        let mut ret: bool = false;
        for vowel in isVowel.as_bytes() {
            if *vowel == *&byte_name[0] {
                ret = true;
                break;
            }
        }
        ret
    };

    if !check_vowel {
        return format!("{}-{}ay", &miner_name[1..miner_name.len()], &miner_name[0..1]);
    }
    format!("{}-hay", miner_name.as_str().to_string())
}

// train vectors manipulation
pub fn get_earnings(action: Option<&str>) -> i32 {
    let mut cost = vec![1, 6, 78, 34, 56, 12, 6];
    cost.sort();
    let summary = cost.len();
    let int_summary: i32 = summary as i32;
    println!("Possible earnings is {:?}", cost);
    match action {
        Some("mean") => {
            let mut mean: i32 = 0;
            for i in cost.into_iter() {
                mean += i;
            }
            mean / int_summary
        }
        Some("median") => {
            cost[summary / 2]
        }
        Some("mode") => {
            let mut cost_map = HashMap::new();
            let mut mode = 0;
            for value in cost.into_iter() {
                let value_entering = cost_map.entry(value).or_insert(0);
                *value_entering += 1;
            }
            let const_map_sibil: HashMap<i32, i32> = cost_map.clone();
            for (_, j) in cost_map.into_iter() {
                mode = find_max_from_vec_test(&const_map_sibil, j);
                if mode == 0 {
                    continue;
                } else {
                    break;
                }
            }
            mode
        }
        _ => 0
    }
}

fn find_max_from_vec_test(mapa: &HashMap<i32, i32>, j: i32) -> i32 {
    let mut big_counter: i32 = 0;
    let summary: i32 = mapa.len() as i32;
    let mut mode: i32 = 0;
    for (i, k) in mapa.into_iter() {
        if j >= *k {
            big_counter += 1;
        }
        if big_counter == summary {
            mode = *i;
        }
    }
    return mode;
}

// Unit test's examples
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn miner_name_pig_latin_not_vowel() -> Result<(), String> {
        let test_miner = Miner {
            address: String::from("test.org"),
            signature: 111,
            name: String::from("Test"),
        };
        let check_string = miner_name_pig_latin(&test_miner);
        if check_string == String::from("est-tay") {
            Ok(())
        } else {
            Err(String::from("'Test' must return 'est-tay'"))
        }
    }

    #[test]
    fn miner_name_pig_latin_vowel() {
        let test_miner = Miner {
            address: String::from("test.org"),
            signature: 111,
            name: String::from("Ebaat"),
        };
        let result = miner_name_pig_latin(&test_miner);
        assert!(result.eq(&String::from("ebaat-hay")), "Expected result 'ebaat-hay', obtained '{}'", result)
    }
}






























