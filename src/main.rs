// #![feature(new_uninit)]
// #![feature(vec_into_raw_parts)]
// #![feature(alloc_layout_extra)]
// #![feature(slice_from_raw_parts)]

#[macro_use]
extern crate lazy_static;
extern crate libc;

pub mod bin_format;
pub mod vm;
use std::env;
use bin_format::*;

use async_std::prelude::*;
#[async_std::main]
async fn main() {
    let constant_pool = [
        0x01,
        0x11,
        0x01,0x00,0x00,0x00,
        0x01,0x00,0x00,0x00,
    
        0x08, 0x00, 0x00, 0x00, 
        0x72, 0x00,  0x65, 0x00,  0x74, 0x00,  0x5F, 0x00,  0x6E, 0x00,  0x75, 0x00,  0x6C, 0x00,  0x6C, 0x00, 
        0x01,0x00,0x00,0x00,
        0x00,
        0x00,
        0x01,
        0x00,0x00,0x00,0x00,
        0x01,0x00,0x00,0x00,
        0x00,
        0x01,0x00,
        // start
        // label : 0
        0x00,0x00,
        0x07,0x00,0x00,0x00,

        0x00,0x02,0x00,0x01,0x00,
        0x00,0x4d,0x00,0x00,0x00,

        0x00,0x29,0x00,0x00,0x00,

        0x00,0x02,0x01,0x02,0x00,
        0x00,0x4d,0x00,0x00,0x00,
        0x00,0x4d,0x01,0x00,0x00,

        0x00,0x25,0x00,0x00,0x00,
        // end
    
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    
    let bytes = [
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x00,0x00,0x00,0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x01,0x00,
    // start
    // label : 0
        0x00,0x00,
        0x07,0x00,0x00,0x00,
    
        0x00,0x45,0x00,0x00,0x00,
        0x00,0x4e,0x00,0x00,0x00,
        0x00,0x4f,0x01,0x00,0x00,
        0x00,0x30,0x01,0x00,0x00,
        0x00,0x50,0x01,0x00,0x00,

        0x00,0x4d,0x02,0x00,0x00,
        0x00,0x4d,0x03,0x00,0x00,
    // end
        0x01,0x00,0x00,0x00,
        0x11,0x01,0x00,0x00,0x00,
    
        0x00,0x00,0x00,0x00,
    ];
    reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    //println!("{:?}",constant_and_pool::CONSTANT_POOL.read().unwrap());
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let func = reader.read_func();
    println!("{}",func);
    let mut stack= vm::executer::stack::Stack::new(Box::new(func));
    use vm::*;
    let h = new_thread(stack);
    let (s,r) = get_sender_receiver(h);
    println!("===== testing Async =====");
    s.send(VMMessage::Break).await;
    s.send(VMMessage::PrintStack).await;
    println!("currently  stack: {}",r.recv().await.unwrap());
    println!("UUID: {}",h);
    s.send(VMMessage::Continue).await;
    println!("{:?}",get_join_handle(h).await);
}