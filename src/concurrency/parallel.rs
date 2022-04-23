extern crate crossbeam;
use crossbeam_channel::bounded;

use crate::thread;
use core::time::Duration;

pub fn parallel() {
    let arr = &[1, 25, -4, 15];
    let max = find_max(arr);
    assert_eq!(max, Some(25));

    //parallel-pipeline using bounded channels
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // close the channel, necessary to exit
            // the for-loop in the worker
            drop(snd1);
        });
        //Parallell processing by 2 threads
        for _ in 0..n_workers {
            //send to sink, recieve from source
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            //spawn workers in seperate threads
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                //recieve until channel closes
                for msg in recvr.iter() {
                    println!("Worker {:?} recieved {}.", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        drop(snd2);

        for msg in rcv2.iter() {
            println!("Sink recieved {}", msg);
        }
    })
    .unwrap();
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}
