use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ChannelText(pub u8);

#[derive(Component)]
pub struct TempoText;

#[derive(Component)]
pub struct PlaybackStateText;
