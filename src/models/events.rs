builder! {ReadyEvent
    v:      u32,
    config: RpcServerConfiguration,
    user:   PartialUser,
}

builder! {ErrorEvent
    code: u32,
    message: String,
}

builder! {RpcServerConfiguration
    cdn_host: String,
    api_endpoint: String,
    environment: String,
}

builder! {PartialUser
    id:            String,
    username:      String,
    discriminator: String,
    avatar:        String,
}
