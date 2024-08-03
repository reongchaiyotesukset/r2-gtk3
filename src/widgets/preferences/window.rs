use gtk::{
    gio,
    glib::{self,clone},   
    subclass::prelude::*,
    glib::Properties,
    prelude::*,
};
use crate::{
    models::{ProvidersModel},
};

mod imp {
    use std::cell::{Cell,RefCell};
  use super::*;
  use gtk::prelude::GtkWindowExt;

  
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]    
    #[template(file = "../../../ui/preferences.ui")]
    #[properties(wrapper_type = super::PreferencesWindow)]
 
  pub struct PreferencesWindow {
     #[property(get, set, construct)]
        pub is_locked: Cell<bool>, 
    }
  
  #[glib::object_subclass]
   impl ObjectSubclass for PreferencesWindow {
   
         const NAME: &'static str = "PreferencesWindow";
         type Type = super::PreferencesWindow;
         type ParentType = gtk::ApplicationWindow;
         //type Interfaces = (gio::Initable,);
         
          fn class_init(klass: &mut Self::Class) {
               Self::bind_template(klass);     
            }
            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                obj.init_template();
            }
            
   }

     impl WidgetImpl for PreferencesWindow {}
   
     #[glib::derived_properties]
    impl ObjectImpl for PreferencesWindow {
         fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.setup_actions();
            obj.setup_widget();
        }
    
    }
   
		impl  WindowImpl for PreferencesWindow {}
        impl  ApplicationWindowImpl for PreferencesWindow {}
        impl  BinImpl for PreferencesWindow {}
        impl  ContainerImpl for PreferencesWindow {}
        impl BoxImpl for PreferencesWindow{}
        impl InitableImpl for PreferencesWindow {
             fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
               Ok(())
             }
        }
  
}


glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
        @extends gtk::Widget,gtk::Container, gtk::Window;

}


impl PreferencesWindow {
     pub fn new(model: &ProvidersModel) -> Self {

        glib::Object::builder().property("model", model).build()
      }
	   pub fn connect_restore_completed<F>(&self, callback: F) -> glib::SignalHandlerId
		where
			F: Fn(&Self) + 'static,
		{
			self.connect_local(
				"restore-completed",
				false,
				clone!(@weak self as win => @default-return None, move |_| {
					callback(&win);
					None
				}),
			)
		}
		fn setup_actions(&self) {
           let imp = self.imp();
		}
		fn setup_widget(&self) {
	       let imp = self.imp();
		}
}
