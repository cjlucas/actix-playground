use actix::prelude::*;

#[derive(Default)]
struct State {
    i: u64
}


struct Agent {
    state: State
}

impl Actor for Agent {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct Update(fn(&mut State));

impl Handler<Update> for Agent {
    type Result = ();

    fn handle(&mut self, msg: Update, _ctx: &mut Self::Context) {
        msg.0(&mut self.state)
    }
}

#[actix::main]
async fn main() {
    let addr = Agent {state: State::default()}.start();
    let _ = addr.send(Update(|state| {
        state.i += 1;
    })).await;

    let _ = tokio::signal::ctrl_c().await;
}
