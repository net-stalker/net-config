use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(NetConfig)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    // Create a new ident for the new struct name.
    // let new_ident = syn::Ident::new("MyNewStruct", ident.span());

    let output = quote! {
        pub use net_file;

        const CONFIG_DIR: &str = ".config";
        const PKG_NAME: &str = std::env!("CARGO_PKG_NAME");

        impl #ident {
            pub fn new(path: &str) -> NetConfigBuilder {
                NetConfigBuilder::default().with_config_dir(path.to_string())
            }

            pub fn builder() -> NetConfigBuilder {
                NetConfigBuilder::default()
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", to_string(self).unwrap())
            }
        }

        #[derive(std::fmt::Debug)]
        pub struct NetConfigBuilder {
            config_path: std::path::PathBuf,
        }

        #[cfg(debug_assertions)]
        impl Default for NetConfigBuilder {
            fn default() -> Self {
                NetConfigBuilder { config_path: std::path::PathBuf::new().join(PKG_NAME).join(CONFIG_DIR) }
            }
        }

        #[cfg(not(debug_assertions))]
        impl Default for NetConfigBuilder {
            fn default() -> Self {
                if env::var("NET_CONFIG_DIR").is_ok() {
                    return NetConfigBuilder { config_path: std::path::PathBuf::from(&env::var("NET_CONFIG_DIR").unwrap()) };
                }

                let base_dir = Self::get_base_dir().unwrap();
                NetConfigBuilder { config_path: std::path::PathBuf::from(base_dir.home_dir()).join(CONFIG_DIR).join(PKG_NAME) }
            }
        }

        impl NetConfigBuilder {
            pub(crate) fn with_config_dir(mut self, config_path: String) -> Self {
                self.config_path = std::path::PathBuf::from(config_path);
                self
            }

            #[cfg(not(debug_assertions))]
            fn get_base_dir() -> Option<directories::BaseDirs> {
                directories::BaseDirs::new()
            }
        }

        impl<'de> NetConfigBuilder {
            pub fn build(&self) -> Result<#ident, config::ConfigError> {
                log::debug!("{:?}", self);

                let config_files = net_file::file::files::Files::find_files(&self.config_path, "toml");
                log::debug!("found config files {:?}", config_files);

                match Self::create_config(config_files) {
                    Ok(config) => { config.try_deserialize::<'de, #ident>() }
                    Err(e) => { Err(e) }
                }
            }

            fn create_config(config_files: Vec<std::path::PathBuf>) -> Result<config::Config, config::ConfigError> {
                use std::ops::Deref;

                let mut builder = config::Config::builder();

                for i in 0..config_files.len() {
                    let path_buf = config_files.get(i).unwrap().deref();
                    builder = builder.add_source(config::File::from(path_buf));
                }

                builder = builder.add_source(config::Environment::with_prefix("net"));
                builder = builder.add_source(config::Environment::with_prefix("net").separator("DOT"));
                let config = builder.build();
                log::info!("{:?}", config);

                config

            }
        }
    };

    output.into()
}