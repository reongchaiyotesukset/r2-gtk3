//new
use once_cell::sync::Lazy;


mod providers;
mod provider;
mod settings;
mod search_provider; //new

//new
pub static RUNTIME: Lazy<tokio::runtime::Runtime> =
    Lazy::new(|| tokio::runtime::Runtime::new().unwrap());
    
pub use self::{
      providers::ProvidersModel,
      provider::Provider,
      search_provider::{start, SearchProviderAction},
};
