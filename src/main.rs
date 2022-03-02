mod bucket;
mod cache;
mod cl;
mod cl_store;

// use std::arch::x86_64::{_mm256_cmpeq_epi16, _mm256_shuffle_epi8, _mm_crc32_u64, _mm_sha1msg1_epu32};
use std::borrow::Borrow;
use std::mem::size_of_val;
// use std::collections::{HashMap, HashSet, VecDeque};
// use std::error::Error;
// use std::intrinsics::offset;
use crate::bucket::{Bucket, FindRes, InsertRes};
use crate::cl::CacheLine;
use crate::cl_store::ClStore;
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
fn main() {
    //insert Key, Value
    //get Key
    //Update Key
    //delete Keyc
    println!("Hello, world!{}", size_of_val(&CacheLine::new()));
    let mut cl_store = ClStore::new(7);
    let mut bucket = Bucket::new();

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
            let res = bucket.put(
                &mut cl_store,
                ((*k as usize) + 0xffff66677).to_string().as_bytes(),
                None,
                *k,
                &Some(*((*k as usize) + 0xffff66677).to_string().as_bytes()),
                *v,
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
            &Some(*((*k as usize) + 0xffff66677).to_string().as_bytes()),
        ) {
            FindRes::Found(d) => {
                println!(
                    "Get: key={} value={}: cl {} slot {} data {}",
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
            &Some(*((*k as usize) + 0xffff66677).to_string().as_bytes()),
        ) {
            FindRes::Found(d) => {
                println!(
                    "Delete: key={} value={}: cl {} slot {} data {}",
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
            &Some(*((*k as usize) + 0xffff66677).to_string().as_bytes()),
        ) {
            FindRes::Found(d) => {
                println!(
                    "key={} value={}: cl {} slot {} data {}",
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
            bucket.put(
                &mut cl_store,
                ((*k as usize) + 0xffff66677).to_string().as_bytes(),
                None,
                *k,
                &Some(*((*k as usize) + 0xffff66677).to_string().as_bytes()),
                *v,
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
            &Some(*((*k as usize) + 0xffff66677).to_string().as_bytes()),
        ) {
            FindRes::Found(d) => {
                println!(
                    "key={} value={}: cl {} slot {} data {}",
                    k, v, d.1, d.0, d.2.value
                );
            }
            FindRes::NotFound => {
                println!("key={} value={}: didn't find entry", k, v);
            }
        }
    });
}
