use std::thread;
use std::time::Duration;


pub fn run_client_server<S, C>(server: S, client: C, duration_ms: u64) -> std::io::Result<()> 
where 
    S: Fn()->std::io::Result<()> + Send + 'static,
    C: Fn()->std::io::Result<()> + Send + 'static,
{
    let server_thread = thread::spawn( move || {
        server().unwrap();
    });

    thread::sleep(Duration::from_millis(duration_ms));

    let client_thread = thread::spawn( move || {
        client().unwrap();
    });

    client_thread.join().unwrap();
    server_thread.join().unwrap();
    
    Ok(())
}