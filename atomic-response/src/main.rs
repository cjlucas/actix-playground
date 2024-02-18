use actix::prelude::*;

struct A;

impl Actor for A {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct Stop;

#[derive(Message)]
#[rtype(result = "()")]
struct Ping;

impl Handler<Ping> for A {
    type Result = ();
    
    fn handle(&mut self, _: Ping, _ctx: &mut Self::Context) {
	println!("PONG");
    }
}


impl Handler<Stop> for A {
    type Result = AtomicResponse<Self, ()>;
    
    fn handle(&mut self, _: Stop, _ctx: &mut Self::Context) -> Self::Result {
        AtomicResponse::new(Box::pin(async move {
	    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
	}.into_actor(self)))
    }
}

#[actix::main]
async fn main() {
    println!("here1");
    let addr = A{}.start();
    println!("here2");
    let _ = addr.send(Stop).await;
    println!("here3");
    let _ = addr.send(Stop).await;
    println!("here4");

    // let _ = tokio::signal::ctrl_c().await;
}
