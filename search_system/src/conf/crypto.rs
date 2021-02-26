use color_eyre::Result;
use std::sync::Arc;
use argonautica::Hasher;
use eyre::eyre;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>
}

impl CryptoService {
    pub fn hash_password(&self, password: String) -> Result<String> {
        Hasher::default()
            .with_secret_key(&*self.key)
            .with_password(password)
            .hash()
            .map_err(|err| eyre!("Hahsing error: {}", err))
    }

    pub fn crypto_service(key:String) -> Self {
        Self {
            key: Arc::new(key.clone())
        }
    }
    
}