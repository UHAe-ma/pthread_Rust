use std::thread;

static M: i32 = 2;
static N: i32 = 4;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..M {
        // Spin up another thread
        children.push(thread::spawn(move || {sub(i);}));
    }
    println!("I am main thread.");
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

fn sub(num: i32){
    for i in 1..N+1{
        println!("I am sub thread No.{} ({}).",num,i );
    }
}
