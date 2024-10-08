use log::Level;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::process;
use tokio::sync;
use tokio::task;

async fn process_func() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = process::Command::new("sort");

    cmd.stdout(std::process::Stdio::piped());
    cmd.stdin(std::process::Stdio::piped());

    let mut child = cmd.spawn()?;

    let animals: &[&str] = &["dog", "bird", "frog", "cat", "fish"];

    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stdin");

    stdin
        .write(animals.join("\n").as_bytes())
        .await
        .expect("could not write to stdin");

    drop(stdin);

    let op = child.wait_with_output().await?;

    println!("Sorted:\n{}", std::str::from_utf8(&op.stdout)?);

    Ok(())
}

async fn io_func() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::open("foo.txt").await?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    println!("file contents: {}", contents);

    let mut outfile = fs::File::create("bar.txt").await?;
    outfile.write_all(contents.as_bytes()).await?;

    Ok(())
}

// green threads,
// threads that are managed by tokio runtime instead.
// like go gorutines
async fn task_func() {
    let a = task::spawn_blocking(|| {
        println!("Starting fib(40) computation...");
        let res = fib(40);
        println!("fib(40) = {}", res);
    });
    let b = task::spawn_blocking(|| {
        println!("Starting fib(39) computation...");
        let res = fib(39);
        println!("fib(39) = {}", res);
    });
    tokio::join!(a, b).0.unwrap();
}

struct MyStruct {
    field: i32,
}

async fn sync_func() {
    let lock = std::sync::Arc::new(sync::Mutex::new(MyStruct { field: 0 }));

    let lock_a = lock.clone();
    let lock_b = lock.clone();

    let a = tokio::spawn(async move {
        let mut val = lock_a.lock().await;
        val.field = 1;
    });

    let b = tokio::spawn(async move {
        let mut val = lock_b.lock().await;
        val.field = 2;
    });

    tokio::join!(a, b).0.unwrap();

    let val = lock.lock().await;
    println!("Value is: {}", val.field);

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(value) = rx.recv().await {
        println!("Got = {}", value);
    }
}

// tokio uses futures
// which are like promises
// but dont execute when you create them
// but when the tokio runtime runs them.
// only until we call the await function of the future

// futures are returned implicityly when ever
// we use the async keyword
async fn sleeper() {
    log::info!("Sleeping");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading some beeg data");
    let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read beeg {} bytes", contents.len());
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

async fn run() {
    // log::info!("Sleeping");
    // time::sleep(time::Duration::from_secs(1)).await;
    // log::info!("Awake!");
    sleeper().await;
    reader().await;

    tokio::join!(sleeper(), reader(), reader(), reader(), reader(), reader(),);

    tokio::task::spawn_blocking(move || {
        log::info!("Computing fib(40)");
        fib(40);
        log::info!("Done computing fib(40)");
    })
    .await
    .unwrap();

    sync_func().await;
    task_func().await;
    if let Err(e) = io_func().await {
        println!("Error: {}", e);
    }
    if let Ok(v) = process_func().await {
        println!("Success: {:?}", v);
    }
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let future = run();

    // rt.block_on(future);
    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("Took {:?} seconds", end - start);
}
