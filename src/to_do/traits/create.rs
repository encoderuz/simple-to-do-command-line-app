pub trait Create{
    fn create(&self, title: &str){
        println!("CREATED: {} is being created", title);
    }
}