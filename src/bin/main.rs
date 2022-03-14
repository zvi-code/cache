// use std::arch::x86_64::{_mm256_cmpeq_epi16, _mm256_shuffle_epi8, _mm_crc32_u64, _mm_sha1msg1_epu32};
use cache_proj::cache::bucket::{Bucket, FindRes, InsertRes};
use cache_proj::cache::cache::Cache;
use cache_proj::cache::cl::CacheLine;
use cache_proj::cache::cl_store::ClStore;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::borrow::Borrow;
// use std::fs::OpenOptions;
// use std::io::BufWriter;
use byte_slice_cast::AsByteSlice;
use std::mem::size_of_val;
// use std::collections::{HashMap, HashSet, VecDeque};
// use std::error::Error;
// use std::intrinsics::offset;

// use std::ptr::hash;
// use modular_bitfield::prelude::*;

//Coding: local cache, for files, read\write, cache policy, assume no concarency
//Coding: given array of positive numbers, find the largest sum, such that you don’t add sum to adjustend nodes
//Code: function that prints all numbers that are sum of 2 qubes in 2 different way
//Coding: Given an array of numbers,
// using a moving window of size 3,
// report the maximum element within the window for each position.
//Coding: giver binary tree, transform it in place to dll left mid right order
//Coding: implement a fifo data structure
//Coding: local cache, for files, read\write, cache policy, assume no concarency
//Coding: given array of positive numbers, find the largest sum, such that you don’t add sum to adjustend nodes
//Coding: stream of int, find first non-repeating int
//Coding: you have event stream and start and end to each event, you need to print all the completed events…

fn generate_key(k: usize) -> String {
    format!("{} is the key", k)
}
fn main() {
    let num_keys = 100000;
    //insert Key, Value
    //get Key
    //Update Key
    //delete Keyc
    // let f = OpenOptions::new()
    //     .append(true)
    //     .create(true)
    //     .open(format!("/tmp/cache/log"))
    //     .expect(&*format!("Unable to open file path /tmp/cache/log"));
    // let mut writer = BufWriter::new(f);
    println!("Hello, world!{}", size_of_val(&CacheLine::new()));
    let mut cl_store = ClStore::new(7);
    let mut bucket = Bucket::new();
    let mut cache = Cache::new(2, 1024);

    let mut ids = vec![0_usize; num_keys];
    let mut i = 0;
    ids.fill_with(|| {
        i += 1;
        i
    });
    let mut rng = thread_rng();
    ids.shuffle(&mut rng);
    let mut vector = vec![0_u32; num_keys];

    for i in 0..num_keys {
        vector[i] = rand::random::<u32>() + 0x1;
    }
    ids.iter().zip(vector.iter()).for_each(|(&k, &v)| {
        let res = cache.upsert(generate_key(k).as_bytes(), format!("{}", v).as_bytes());
        if !res {
            println!(
                "Failed write Key {:?}: Asked {:x?}",
                generate_key(k).as_bytes(),
                v
            );
        }
    });
    let mut had_failure = false;
    ids.iter().zip(vector.iter()).for_each(|(&k, &v)| {
        // cache.upsert(
        //     format!("the key {}", k).as_bytes(),
        //     format!("{}", v).as_bytes(),
        // );
        let res = cache.get(generate_key(k).as_bytes());
        match res {
            Some(resp) => {
                if resp.as_slice()[0..4] != format!("{}", v).as_bytes()[0..4] {
                    println!(
                        "Key {:x?}: Got {:x?} Asked {:x?}",
                        generate_key(k).as_bytes(),
                        resp,
                        format!("{}", v).as_bytes()
                    );
                }
            }
            None => {
                println!(
                    "Didn't find Key {:x?}: Asked {:x?} \n",
                    generate_key(k).as_bytes(),
                    format!("{}", v).as_bytes()
                );
                cache.print_bucket(generate_key(k).as_bytes());
                had_failure = true;
            }
        }
    });
    if !had_failure {
        cache.upsert("my paycheck".as_ref(), "a".as_ref());
        let res = cache.get("my paycheck".as_ref());
        match res {
            Some(resp) => println!("asked: my paycheck answered: {:x?}", resp),
            None => println!("asked: my paycheck No answer"),
        }
        bucket.head = cl_store.allocate_cl();

        let kv_pairs = [
            (20, 199982),
            (32, 9982221),
            (45, 889292),
            (44, 8827272),
            (89, 8872),
            (81, 88711),
            (11, 88711),
            (13, 88711),
            (20, 199982),
        ];
        let kv_pairs2 = [
            (20, 199982),
            (332, 9982221),
            (425, 889292),
            (542, 8827272),
            (89, 8872),
            (281, 88711),
            (888, 88711),
            (333, 88711),
            (266, 199982),
        ];
        let kv_pairs3 = [
            (1, 199982),
            (2, 9982221),
            (3, 889292),
            (4, 8827272),
            (5, 8872),
            (6, 88711),
            (7, 88711),
            (8, 88711),
            (9, 199982),
        ];
        let mut free_cl = cl_store.allocate_cl();

        let insert_res = kv_pairs
            .iter()
            .map(|(k, v)| {
                let val = v.to_string();
                let res = bucket.put(
                    &mut cl_store,
                    ((*k as usize) + 0xffff66677).to_string().as_bytes(),
                    None,
                    *k,
                    Some(((*k as usize) + 0xffff66677).to_string().as_bytes()),
                    val.as_byte_slice(),
                    false,
                    Some(free_cl),
                );
                match res.borrow() {
                    InsertRes::Success(cl) => {
                        if *cl == free_cl {
                            free_cl = cl_store.allocate_cl();
                        }
                    }
                    _ => (),
                };
                res
            })
            .collect::<Vec<InsertRes>>();
        insert_res.iter().for_each(|res| match res {
            InsertRes::Success(ix) => println!("Succ ix {}", ix),
            InsertRes::EntryExists(ix) => println!("Exist ix {}", ix),
            InsertRes::OutOfSpace => println!("OOS"),
        });
        kv_pairs.iter().for_each(|(k, v)| {
            match bucket.get(
                &mut cl_store,
                *k,
                Some(((*k as usize) + 0xffff66677).to_string().as_bytes()),
            ) {
                FindRes::Found(d) => {
                    println!(
                        "Get: key={} value={}: cl {} slot {} data {:?}",
                        k, v, d.1, d.0, d.2.value
                    );
                }
                FindRes::NotFound => {
                    println!("Get: key={} value={}: didn't find entry", k, v);
                }
            }
        });
        kv_pairs2.iter().for_each(|(k, v)| {
            match bucket.delete(
                &mut cl_store,
                *k,
                Some(((*k as usize) + 0xffff66677).to_string().as_bytes()),
            ) {
                FindRes::Found(d) => {
                    println!(
                        "Delete: key={} value={}: cl {} slot {} data {:?}",
                        k, v, d.1, d.0, d.2.value
                    );
                }
                FindRes::NotFound => {
                    println!("Delete: key={} value={}: didn't find entry", k, v);
                }
            }
        });
        kv_pairs.iter().for_each(|(k, v)| {
            match bucket.get(
                &mut cl_store,
                *k,
                Some(((*k as usize) + 0xffff66677).to_string().as_bytes()),
            ) {
                FindRes::Found(d) => {
                    println!(
                        "key={} value={}: cl {} slot {} data {:?}",
                        k, v, d.1, d.0, d.2.value
                    );
                }
                FindRes::NotFound => {
                    println!("key={} value={}: didn't find entry", k, v);
                }
            }
        });
        let insert_res3 = kv_pairs3
            .iter()
            .map(|(k, v)| {
                let val = v.to_string();
                bucket.put(
                    &mut cl_store,
                    ((*k as usize) + 0xffff666277).to_string().as_bytes(),
                    None,
                    *k,
                    Some(((*k as usize) + 0xffff666277).to_string().as_bytes()),
                    val.as_byte_slice(),
                    false,
                    None,
                )
            })
            .collect::<Vec<InsertRes>>();
        insert_res3.iter().for_each(|res| match res {
            InsertRes::Success(ix) => println!("Succ ix {}", ix),
            InsertRes::EntryExists(ix) => println!("Exist ix {}", ix),
            InsertRes::OutOfSpace => println!("OOS"),
        });
        kv_pairs3.iter().for_each(|(k, v)| {
            match bucket.get(
                &mut cl_store,
                *k,
                Some(((*k as usize) + 0xffff66677).to_string().as_bytes()),
            ) {
                FindRes::Found(d) => {
                    println!(
                        "key={} value={}: cl {} slot {} data {:?}",
                        k, v, d.1, d.0, d.2.value
                    );
                }
                FindRes::NotFound => {
                    println!("key={} value={}: didn't find entry", k, v);
                }
            }
        });
        kv_pairs3.iter().for_each(|(k, v)| {
            match bucket.get(
                &mut cl_store,
                *k,
                Some(((*k as usize) + 0xffff666277).to_string().as_bytes()),
            ) {
                FindRes::Found(d) => {
                    println!(
                        "key={} value={}: cl {} slot {} data {:?}",
                        k, v, d.1, d.0, d.2.value
                    );
                }
                FindRes::NotFound => {
                    println!("key={} value={}: didn't find entry", k, v);
                }
            }
        });
    }
}
