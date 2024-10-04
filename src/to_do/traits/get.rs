pub trait Get {
    fn get(&self, title: &str){
        println!("Get: {} is being fetched", title);
    }
}