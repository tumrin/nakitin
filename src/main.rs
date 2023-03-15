use poise::serenity_prelude::{self as serenity, Activity, Member, OnlineStatus};
use rand::Rng;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Nakittaa käyttäjän, nakkisuoja on ',' eroteltu lista @käyttäjiä
#[poise::command(slash_command, prefix_command)]
async fn nakki(
    ctx: Context<'_>,
    #[description = "Nakkisuoja"] users: Option<String>,
) -> Result<(), Error> {
    let members: Vec<Member> = ctx
        .serenity_context()
        .http
        .get_guild_members(ctx.guild_id().expect("to be on a server").0, None, None)
        .await?
        .into_iter()
        .filter(|member| !member.user.bot)
        .collect();
    let excluded: Vec<String> = users.map_or(vec![], |users| {
        users.split(',').map(|id| id.trim().to_string()).collect()
    });
    if excluded.len() >= members.len() {
        ctx.say("All members excluded").await?;
        return Ok(());
    }
    let allowed_members: Vec<String> = members
        .iter()
        .map(|member| format!("<@{}>", member.user.id))
        .filter(|member| !excluded.contains(&member))
        .collect();

    let gen_rng = || {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..allowed_members.len())
    };
    let random_num = gen_rng();
    let response = format!("{}", allowed_members[random_num]);
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![nakki()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_presence(Some(Activity::listening("/nakki")), OnlineStatus::Online)
                    .await;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
