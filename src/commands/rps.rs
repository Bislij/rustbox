use poise::{ChoiceParameter, CreateReply};
use rand::{seq::SliceRandom, thread_rng};
use serenity::all::{
    ButtonStyle, Color, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateEmbed,
    CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse, ReactionType,
};
use std::time::Duration;
use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{Context, Error};

#[derive(Debug, Eq, PartialEq, Copy, Clone, VariantArray, ChoiceParameter)]
enum ValidMove {
    Rock,
    Paper,
    Scissors,
}

impl ValidMove {
    pub fn get_emoji(&self) -> ReactionType {
        match self {
            ValidMove::Rock => ReactionType::from('🗿'),
            ValidMove::Paper => ReactionType::from('🧻'),
            ValidMove::Scissors => ReactionType::from('✂'),
        }
    }
    pub fn get_emoji_text(&self) -> &'static str {
        match self {
            ValidMove::Rock => "🗿",
            ValidMove::Paper => "🧻",
            ValidMove::Scissors => "✂",
        }
    }
}

#[poise::command(
    prefix_command,
    slash_command,
    required_bot_permissions = "MANAGE_MESSAGES"
)]
pub async fn rps(
    ctx: Context<'_>,
    #[description = "Your move to play."] mut choice: Option<ValidMove>,
) -> Result<(), Error> {
    let response = CreateReply::default().content("```Loading...```");
    let msg = ctx.send(response).await?;

    'game_loop: loop {
        match choice {
            Some(_) => {
                let comp_move = ValidMove::VARIANTS.choose(&mut thread_rng()).unwrap();
                let user_move = &choice.unwrap();

                let comp_move_num = ValidMove::VARIANTS
                    .iter()
                    .position(|&e| e == *comp_move)
                    .unwrap() as i32;
                let user_move_num = ValidMove::VARIANTS
                    .iter()
                    .position(|&e| e == *user_move)
                    .unwrap() as i32;

                let [res_desc, res_footer]: [String; 2];
                if user_move_num == comp_move_num {
                    res_desc = format!(
                        "You Tied! Computer played {:?}, and you played {:?}.",
                        comp_move, user_move
                    );
                    res_footer = "pretty mid game ngl".into();
                } else if ((user_move_num - comp_move_num) + 3) % 3 == 1 {
                    res_desc = format!(
                        "You Win! Computer played {:?}, and you played {:?}.",
                        comp_move, user_move
                    );
                    res_footer = "decent job chump".into();
                } else {
                    res_desc = format!(
                        "You Lose! Computer played {:?}, and you played {:?}.",
                        comp_move, user_move
                    );
                    res_footer = "massive L".into();
                }

                let rps_embed = CreateEmbed::new()
                    .color(Color::DARK_GOLD)
                    .author(CreateEmbedAuthor::new("rustbox")
                        .icon_url("https://cdn.discordapp.com/avatars/1223321217285492949/8e439c9c4fe0065e91aac6fe00f437ad?size=1024"))
                    .description(res_desc)
                    .field(
                        "You:",
                        user_move.get_emoji_text(),
                        true,
                    )
                    .field(
                        "Computer:",
                        comp_move.get_emoji_text(),
                        true,
                    )
                    .footer(CreateEmbedFooter::new(res_footer));

                let response = CreateReply::default()
                    .embed(rps_embed)
                    .content("")
                    .components(Vec::new());
                msg.edit(ctx, response).await?;
                break 'game_loop;
            }
            None => {
                let rps_embed = CreateEmbed::new()
                    .color(Color::DARK_GOLD)
                    .author(CreateEmbedAuthor::new("rustbox")
                        .icon_url("https://cdn.discordapp.com/avatars/1223321217285492949/8e439c9c4fe0065e91aac6fe00f437ad?size=1024"))
                    .description("Choose your move!")
                    .field(
                        "Possible Moves:",
                        "🗿 - Rock\n🧻 - Paper\n✂️ - Scissors",
                        false,
                    );
                let rps_components = vec![CreateActionRow::Buttons(vec![
                    CreateButton::new("rock")
                        .emoji(ValidMove::Rock.get_emoji())
                        .style(ButtonStyle::Primary),
                    CreateButton::new("paper")
                        .emoji(ValidMove::Paper.get_emoji())
                        .style(ButtonStyle::Primary),
                    CreateButton::new("scissors")
                        .emoji(ValidMove::Scissors.get_emoji())
                        .style(ButtonStyle::Primary),
                ])];

                let response = CreateReply::default()
                    .embed(rps_embed)
                    .content("")
                    .components(rps_components);
                msg.edit(ctx, response).await?;

                while let Some(mci) = ComponentInteractionCollector::new(ctx)
                    .author_id(ctx.author().id)
                    .channel_id(ctx.channel_id())
                    .timeout(Duration::from_secs(120))
                    .filter(move |mci| {
                        mci.data.custom_id == "rock"
                            || mci.data.custom_id == "paper"
                            || mci.data.custom_id == "scissors"
                    })
                    .await
                {
                    mci.create_response(ctx, CreateInteractionResponse::Acknowledge)
                        .await?;
                    match mci.data.custom_id.as_str() {
                        "rock" => choice = Some(ValidMove::Rock),
                        "paper" => choice = Some(ValidMove::Rock),
                        "scissors" => choice = Some(ValidMove::Rock),
                        _ => choice = None,
                    }
                    continue 'game_loop;
                }
            }
        }
    }

    Ok(())
}
