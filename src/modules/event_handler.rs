use serenity::{
    async_trait,
    model::{gateway::Ready, interactions::Interaction},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        self.handle_slash_commands(&ctx, &interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Logged in as {}, running {} v{}.",
            ready.user.tag(),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        self.register_slash_commands(&ctx).await;
    }
}
