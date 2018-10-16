use std::sync::{Arc, Mutex};
use std::thread;

static M: i32 = 4;
static N: i32 = 25000;

fn count(ctr:& Mutex<i32>){
    for i in 0..N{
        {
        //ctrのロックの獲得
        let mut ctr = ctr.lock().unwrap();
        *ctr += 1;
        //スコープから出るとctrを解放
        }
    }
}

fn main() {
    let ctr = Arc::new(Mutex::new(0));
    let mut children = vec![];
    for i in 0..M {
        //ctrの参照カウントを増やす
        let ctr = ctr.clone();
        children.push(thread::spawn(move || {
            count(&ctr);
        }))
    }

    //子スレッドが終了するのを待機
    for child in children{
        let _ = child.join();
    }

    //ctrの参照カウントを増やす
    let ctr = ctr.clone();
    //ctrのロックの獲得
    let ctr = ctr.lock().unwrap();
    println!("ctr={}",ctr );
}
