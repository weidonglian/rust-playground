use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn thread_basic() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[test]
fn channel_demo() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[test]
fn mutex_basic() {
    use std::sync::Mutex;
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }
    println!("m is {:?}", m);
}

#[test]
fn spawn_and_join() {
    static NTHREADS: i32 = 10;
    let mut thread_handles = vec![];
    for i in 0..NTHREADS {
        thread_handles.push(std::thread::spawn(move || {
            println!("running thread:{}", i);
        }));
    }

    for h in thread_handles {
        h.join().expect("unknown to join thread");
    }
}

#[test]
fn spawn_compute_return() {
    use std::thread;

    // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);
        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        //
        // TODO: try removing the 'move' and see what happens
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                // iterate over the characters of our segment..
                .chars()
                // .. convert text-characters to their number value..
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // .. and sum the resulting iterator of numbers
                .sum();

            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result
        }));
    }

    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/

    // collect each thread's intermediate results into a new Vec
    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's return-value
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // combine all intermediate sums into a single final sum.
    //
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result);
}
