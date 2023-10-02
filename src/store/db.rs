use std::{sync::{Arc, RwLock}, collections::HashMap, time::Instant};


#[derive(Debug,Clone)]
pub struct Cache{
   data:Arc<RwLock<HashMap<String,String>>>,
   expiration:Arc<RwLock<HashMap<String,Instant>>>

}


impl Cache{

   pub fn new()->Self{

      let data = Arc::new(RwLock::new(HashMap::new()));
      let expiration = Arc::new(RwLock::new(HashMap::new()));
      Cache { data , expiration }

   }

}