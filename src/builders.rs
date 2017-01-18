use args::*;

// For when you don't need <'a>. Haven't tested this yet so it may go wrong...
macro_rules! builder {
    ( $sstruct:expr; $( $name:ident, $ttype:ty ),* ) => {
        $(
            pub fn $name(mut self, $name: $ttype) -> $sstruct {
                self.$name = Some($name);
                self
            }
        )*
    }
}

macro_rules! buildera {
    ( $sstruct:ident; $( $name:ident, $ttype:ty );* ) => {
        $(
            pub fn $name(mut self, $name: $ttype) -> $sstruct<'a> {
                self.$name = Some($name);
                self
            }
        )*
    }
}

impl<'a> GetUpdates<'a> {
    pub fn new() -> GetUpdates<'a> {
        GetUpdates {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    buildera!(GetUpdates; offset, i64; limit, i64; timeout, i64;
                          allowed_updates, &'a [&'a str]);
}

impl<'a> SetWebhook<'a> {
    pub fn new(url: &'a str) -> SetWebhook<'a> {
        SetWebhook {
            url: url,
            certificate: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    buildera!(SetWebhook; certificate, &'a str; max_connections, i64;
                          allowed_updates, &'a [&'a str]);
}