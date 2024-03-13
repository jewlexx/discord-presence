use super::events::PartialUser;

builder! {SubscriptionArgs
    secret: String,     // Activity{Join,Spectate}
    user: PartialUser,  // ActivityJoinRequest
}

builder! {Subscription
    evt: String,
}
