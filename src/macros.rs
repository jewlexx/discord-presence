macro_rules! builder_func_doc {
    [ $type:tt ] => {
        concat!(
            "Instantiates the current struct with the given ",
            stringify!($type),
            " value."
        )
    };
}

macro_rules! builder_func {
    [ $name:ident, $type:tt func ] => {
        #[doc = builder_func_doc!($type)]
        #[must_use]
        pub fn $name<F>(mut self, func: F) -> Self
            where F: FnOnce($type) -> $type
        {
            self.$name = Some(func($type::default())); self
        }
    };

    [ $name:ident, String ] => {
        #[doc = builder_func_doc!(Stringish)]
        #[must_use]
        pub fn $name<S>(mut self, value: S) -> Self
            where S: Into<String>
        {
            self.$name = Some(value.into()); self
        }
    };

    [ $name:ident, $type:ty ] => {
        #[doc = builder_func_doc!($type)]
        #[must_use]
        pub fn $name(mut self, value: $type) -> Self {
            self.$name = Some(value); self
        }
    };
}

macro_rules! into_error {
    [ $opt:expr, $msg:expr ] => {
        match $opt {
            Some(v) => Ok(v),
            None => Err(crate::error::DiscordError::NoneError($msg)),
        }
    };

    [ $opt:expr ] => {
        into_error!($opt, String::from("Option unwrapped to None"))
    };
}

macro_rules! builder {
    [ @st ( $name:ident $field:tt: $type:tt alias = $alias:tt, $($rest:tt)* ) -> ( $($out:tt)* ) ] => {
        builder![ @st
            ( $name $($rest)* ) -> (
                $($out)*
                #[doc = concat!("Optional " , stringify!($field), " field")]
                #[serde(skip_serializing_if = "Option::is_none", rename = $alias)]
                pub $field: Option<$type>,
            )
        ];
    };

    [ @st ( $name:ident $field:tt: $type:tt func, $($rest:tt)* ) -> ( $($out:tt)* ) ] => {
        builder![ @st ( $name $field: $type, $($rest)* ) -> ( $($out)* ) ];
    };

    [ @st ( $name:ident $field:ident: $type:ty, $($rest:tt)* ) -> ( $($out:tt)* ) ] => {
        builder![ @st
            ( $name $($rest)* ) -> (
                $($out)*
                #[doc = concat!("Optional " , stringify!($field), " field")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $field: Option<$type>,
            )
        ];
    };

    [ @st ( $name:ident ) -> ( $($out:tt)* ) ] => {
        #[doc = concat!(stringify!($name), " struct")]
        #[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize, Hash, Eq)]
        pub struct $name { $($out)* }
    };

    [ @im ( $name:ident $field:ident: $type:tt func, $($rest:tt)* ) -> ( $($out:tt)* ) ] => {
        builder![ @im ( $name $($rest)* ) -> ( builder_func![$field, $type func]; $($out)* ) ];
    };

    [ @im ( $name:ident $field:ident: $type:tt alias = $modifier:tt, $($rest:tt)* ) -> ( $($out:tt)* ) ] => {
        builder![ @im ( $name $field: $type, $($rest)* ) -> ( $($out)* ) ];
    };

    [ @im ( $name:ident $field:ident: $type:tt, $($rest:tt)* ) -> ( $($out:tt)* ) ] => {
        builder![ @im ( $name $($rest)* ) -> ( builder_func![$field, $type]; $($out)* ) ];
    };

    [ @im ( $name:ident ) -> ( $($out:tt)* ) ] => {
        impl $name {
            #[doc = concat!("Instantiates the `", stringify!($name), "` struct using the `Default` implementation")]
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            $($out)*
        }
    };

    [ $name:ident $($body:tt)* ] => {
        builder![@st ( $name $($body)* ) -> () ];
        builder![@im ( $name $($body)* ) -> () ];
    }
}
