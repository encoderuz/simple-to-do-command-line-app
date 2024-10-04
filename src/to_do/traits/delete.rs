pub trait Delete{
    fn delete(&self, title: &str){
        println!("DELETED: {} is being deleted", title);
    }
}