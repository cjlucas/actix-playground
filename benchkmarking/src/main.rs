use actix::prelude::*;

struct A;

impl Actor for A {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct Stop;

#[derive(Message)]
#[rtype(result = "i16")]
struct Ping;

impl Handler<Ping> for A {
    type Result = i16;

    fn handle(&mut self, _: Ping, _ctx: &mut Self::Context) -> Self::Result {
        2
    }
}

#[actix::main]
async fn main() {
    let addr = A {}.start();

    let mut i = 0i64;
    for _ in 0..1_000_000 {
        i += addr.send(Ping).await.unwrap() as i64;
    }

    dbg!(i);
}
