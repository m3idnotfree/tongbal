use serde::{Deserialize, Serialize};

use crate::channel::{Channel, Follower, Moderator, Subscriber};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelResponse {
    pub data: Vec<Channel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModeratorResponse {
    pub data: Vec<Moderator>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FollowersResponse {
    pub data: Vec<Follower>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribersResponse {
    pub data: Vec<Subscriber>,
}
