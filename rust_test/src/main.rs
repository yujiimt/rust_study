async fn add(letf: i32, right: i32) -> i32{
    left + rignt
}

#[async_std::main]
async fn main(){
    let ans = add(2,3).await;
    println!("{}". ans);
}