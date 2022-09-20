use serde::{Deserialize, Serialize};

/*
{
        "cmd": String("AUTHENTICATE"),
        "data": Object {
            "access_token": String("<concealed by 1Password>"),
            "application": Object {
                "bot_public": Bool(true),
                "bot_require_code_grant": Bool(false),
                "description": String("A Discord overlay that just works™"),
                "flags": Number(0),
                "hook": Bool(true),
                "icon": String("0dc7afd6805e34348161dc1295c6aae5"),
                "id": String("905987126099836938"),
                "name": String("overlayed"),
                "privacy_policy_url": String("https://overlayed.dev/privacy"),
                "summary": String(""),
                "tags": Array [
                    String("discord"),
                    String("electron"),
                    String("overlay"),
                    String("react"),
                    String("utility"),
                ],
                "terms_of_service_url": String("https://overlayed.dev/tos"),
                "type": Null,
                "verify_key": String("e00fe2fe5bedebc07bd3bcf6f7bead178e615c2b6049b4829a4c5c9ce949e6ba"),
            },
            "expires": String("2022-09-23T17:10:17.229000+00:00"),
            "scopes": Array [
                String("messages.read"),
                String("rpc"),
            ],
            "user": Object {
                "avatar": String("bf6a5dae7b434918ef5c1224f8194f3b"),
                "avatar_decoration": Null,
                "discriminator": String("1984"),
                "id": String("378293909610037252"),
                "public_flags": Number(64),
                "username": String("Hacksore"),
            },
        },
        "evt": Null,
        "nonce": String("6c1df072-4267-4296-8f19-737db2cc8cb4"),
    }, */
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyConfig {
  pub api_endpoint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyData {
  pub config: ReadyConfig
}
