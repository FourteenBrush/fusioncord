use twilight_model::guild::Guild;

#[derive(Debug)]
pub enum RenderMessage {
    InitialData {
        guilds: Vec<Guild>,
    }
}