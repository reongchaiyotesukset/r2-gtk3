use gtk::{
    gio,
    glib::{self},
    subclass::prelude::*,
    prelude::*,
};
use crate::{
  models::{ProvidersModel},
};

mod imp {
  //add new
  use std::{
    cell::{OnceCell},
  };
 //end list add new
use super::*;


 #[derive(Default)]
  pub struct PreferencesWindow {
   //add new

    pub model: OnceCell<ProvidersModel>,
   //end list add new
  }

  #[glib::object_subclass]
   impl ObjectSubclass for PreferencesWindow {
   
       const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
      

        fn new() -> Self {
          
           Self 
            {
              //add new
                model: OnceCell::default(),
              //end list add new
            }

        }
        //add new
        fn class_init(klass: &mut Self::Class) {
          Self::bind_template(klass);

        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
          obj.init_template();
        }

        //end list add new
   }
   impl ObjectImpl for PreferencesWindow {
       
   }
    impl WidgetImpl for PreferencesWindow {}


    
   impl ApplicationImpl for PreferencesWindow {

      
   }
  
}
glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>) 
       @extends gio::Application, gtk::Application,gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}
impl PreferencesWindow {
     
  pub fn new(model: &ProvidersModel) -> Self {
    glib::Object::builder().property("model", model).build()
  }
}  


