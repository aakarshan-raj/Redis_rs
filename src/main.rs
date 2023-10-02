mod store;

use store::db::Cache;

fn main() {

    let x = Cache::new();
    println!("{:?}",x);

}
