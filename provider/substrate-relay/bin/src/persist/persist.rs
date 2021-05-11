use std::path::PathBuf;

use rand::{distributions::Alphanumeric, Rng};

use crate::error;
use crate::persist::{Chain, Generic, Persist, Token};

impl Persist {
    async fn init_file(config_file: &PathBuf) -> error::Result<()> {
        if !config_file.display().to_string().ends_with(".toml") {
            return Err(error::CliError::ConfigPathNotToml)?;
        }
        if !config_file.exists() {
            if let Some(parent) = config_file.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            async_fs::write(&config_file, "").await?;
        }
        if !config_file.is_file() {
            return Err(error::CliError::ConfigPathNotFile)?;
        }
        Ok(())
    }

    pub async fn load_from_file(config_file: PathBuf) -> error::Result<Self> {
        Persist::init_file(&config_file).await?;
        let toml_config = std::fs::read_to_string(&config_file)?;
        let mut persist: Persist = toml::from_str(&toml_config)?;
        let generic: &mut Generic = persist.generic_mut();
        generic.set_config_file(config_file);
        Ok(persist)
    }

    pub async fn store(&self) -> error::Result<&Self> {
        Persist::init_file(&self.generic.config_file).await?;

        // // reminder: https://github.com/alexcrichton/toml-rs/issues/142
        // // | error: values must be emitted before tables
        // // | if not have an change
        // let toml_text = toml::to_string_pretty(&self)?;
        // std::fs::write(&self.generic.config_file, toml_text)?;

        let json = serde_json::to_string(&self)?;
        let v: toml::Value = serde_json::from_str(&json)?;
        let toml_text: String = v.to_string();
        async_fs::write(&self.generic.config_file, toml_text).await?;

        Ok(self)
    }
}

// chain
impl Persist {
    pub async fn chain_list(&self) -> &Vec<Chain> {
        &self.chains
    }
    pub async fn chain_add(&mut self, chain: Chain) -> error::Result<&Self> {
        let chains = &mut self.chains;
        if chains.iter().any(|item| item.name == chain.name) {
            return Err(error::CliError::ChainNameExists)?;
        }
        chains.push(chain);
        self.store().await
    }

    pub async fn chain_update(&mut self, chain: Chain) -> error::Result<&Self> {
        let chains = &mut self.chains;
        if let Some(saved_chain) = chains.iter_mut().find(|ref item| item.name == chain.name) {
            saved_chain.host = chain.host;
            saved_chain.port = chain.port;
            saved_chain.signer = chain.signer;
            return self.store().await;
        }
        Err(error::CliError::ChainNotFound)?
    }

    pub fn chain_exists<T: AsRef<str>>(&self, name: T) -> bool {
        self.chains.iter().any(|item| item.name == name.as_ref())
    }

    pub async fn chain_remove<T: AsRef<str>>(&mut self, chain_name: T) -> error::Result<&Self> {
        if !self.chain_exists(&chain_name) {
            return Err(error::CliError::ChainNotFound)?;
        }
        let chains = &mut self.chains;
        chains.retain(|item| &item.name != chain_name.as_ref());
        self.store().await
    }
}

// token
impl Persist {
    fn new_token(&self) -> String {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let mut m = sha1::Sha1::new();
        m.update(s.as_ref());
        m.digest().to_string()
    }

    pub async fn token_generate(&mut self, remark: Option<String>) -> error::Result<Token> {
        let token_value = self.new_token();
        let token = Token::builder()
            .remark(Some(remark.unwrap_or("".to_string())))
            .value(token_value)
            .build();
        self.tokens.push(token.clone());
        self.store().await?;
        Ok(token)
    }

    pub async fn token_list(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn token_exists<T: AsRef<str>>(&self, token: T) -> bool {
        self.tokens.iter().any(|item| item.value == token.as_ref())
    }

    pub async fn token_remove<T: AsRef<str>>(&mut self, token: T) -> error::Result<&Self> {
        if !self.token_exists(&token) {
            return Err(error::CliError::TokenNotFound)?;
        }
        let tokens = &mut self.tokens;
        tokens.retain(|item| &item.value != token.as_ref());
        self.store().await
    }
}
