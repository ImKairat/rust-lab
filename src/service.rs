use std::thread;


pub fn run_client_server<S, C>(server: S, client: C) -> std::io::Result<()> 
where 
    S: Fn()->std::io::Result<()> + Send + 'static,
    C: Fn()->std::io::Result<()> + Send + 'static,
{
    let client_thread = thread::spawn( move || {
        client().unwrap();
    });
    let server_thread = thread::spawn( move || {
        server().unwrap();
    });
    
    client_thread.join().unwrap();
    server_thread.join().unwrap();
    
    Ok(())
}