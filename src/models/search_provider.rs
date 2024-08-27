use futures_channel::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
//pub use search_provider::{SearchProvider as SP, SearchProviderImpl};
pub use search_provider::{SearchProvider as SP};

use super::RUNTIME;
use super::search_provider;
//use glib::Receiver;
pub struct SearchProvider {
    sender: Sender<SearchProviderAction>,
}
pub enum SearchProviderAction {
   LaunchSearch(Vec<String>),
}

pub trait SearchProviderImpl {
    // ...
}

impl SearchProvider {

}

impl SearchProviderImpl for SearchProvider {

}

pub async fn start(){
    
}
