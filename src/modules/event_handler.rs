use serenity::{
    async_trait,
    model::{gateway::Ready, interactions::Interaction},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match &interaction {
            Interaction::ApplicationCommand(interaction) => {
                self.handle_slash_commands(&ctx, interaction).await;
            }
            Interaction::MessageComponent(_) => {}
            _ => {}
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Logged in as {}, ready to serve {} guilds, running {} v{}.",
            ready.user.tag(),
            ready.guilds.len(),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        self.register_slash_commands(&ctx, &ready).await;
    }
}
