//fn main() {
//    println!("Hello, world!");
//}

use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");

}

fn main() {
    let future = hello_world();
    block_on(future);
}

//Async is about processes , for example jump , clap , sing . These are,
//simeoustenous procesess therefore consider jumping clapin and singing ,
//@ the same time if not do it one after the other , so jump , then claping ,
//and singing are aiting or sleeping, when jump is finshed then singing and clapping 
// starts 


async fn learn_and_sing() {
    //wait untill the song......
    //We use .await here rather than block_on'
    //thread, which makes it possible to ....

    let song = learn_song().await;
    sing_song(song).await;
}


//fn main() {
//    let song = block_on(learn_song());
//    block_on(sing_song(song));
//    block_on(dance());
//}

async fn learn_and_sing(){
    // Wait  until the song...........
    //We use .aawait here rather than .....
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main(){
    let f1 = learn_and_sing();
    let f2 = dance(); 
    // join! is like awit but can ait for mutiple futures concurrently
    //if we are temporarily blocked in the lern and sing future 
    //learn n sing can take back over 
    //async_main is blocked and will yield to the executor
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}


trait SimpleFuture {
    type Output; 
    fn poll(&mut self, wake:fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}