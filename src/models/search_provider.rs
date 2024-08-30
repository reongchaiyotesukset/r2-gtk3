use futures_channel::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
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
   pub fn new() -> (Self, Receiver<SearchProviderAction>) {
        let (sender, receiver) = futures_channel::mpsc::unbounded();
        (Self { sender }, receiver)
    }
}

impl SearchProviderImpl for SearchProvider {

}

pub async fn start(){
     let (search_provider, receiver) = SearchProvider::new();

    let path = "../../test/";
    let name = format!("{}.SearchProvider", "SearchProvider");

    RUNTIME.spawn(async move {
           // match SP::new(search_provider, name, path).await {

           //};

    });

    //Ok(receiver)
    println!("{:#?}",receiver);
}
