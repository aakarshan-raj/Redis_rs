mod store;

use std::{net::TcpListener, sync::Arc};

use store::db::Cache;

fn main() {

    let connections = TcpListener::bind("127.0.0.1:8907").unwrap();

    let cache = Arc::new(Cache::new());

    for connection in connections.incoming(){
        println!("{:?}",connection);
    }

}
