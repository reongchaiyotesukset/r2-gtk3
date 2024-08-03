
//update 07/03/2024
use gtk::{
    gio,
    glib::{self,clone},   
    subclass::prelude::*,
    glib::Properties,
    prelude::*,
};
use crate::{
    widgets::{Window,PreferencesWindow},
    models::{ProvidersModel},
};
use crate::config;

mod imp {
  use std::cell::{Cell,RefCell};
  use super::*;
  //#[derive(Debug)]
  #[derive(Default,glib::Properties,Debug)]
  #[properties(wrapper_type = super::Application)]
  pub struct Application {
        pub window: RefCell<Option<glib::WeakRef<Window>>>,
        pub model: ProvidersModel,
        #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
    }
  #[glib::object_subclass]
   impl ObjectSubclass for Application {
   
         const NAME: &'static str = "Application";//
         type Type = super::Application;
         type ParentType = gtk::Application;
        // type Interfaces = ();
       }
     #[glib::derived_properties]
   impl ObjectImpl for Application { }
   
   impl ApplicationImpl for Application {
       
            fn startup(&self) {
              
                self.parent_startup();
                
                let app = self.obj();

                 
                
                    let preferences_action = gio::ActionEntry::builder("preferences")
                   .activate(|app: &Self::Type, _, _| {
                   
                    let model = &app.imp().model;
                    let window = app.active_window();
                    
                      let preferences = PreferencesWindow::new(model);
					      preferences.connect_restore_completed(clone!(@weak window =>move |_| {					  
					                println!("Connect_restore_completed!!!");
					      }));
                          preferences.present();
                      
                    }).build();
					
                    let lock_action = gio::ActionEntry::builder("lock")
				        .activate(|app: &Self::Type, _, _| app.set_is_locked(true))
					    .build(); 
            
                    app.add_action_entries([
                       
					   preferences_action,
					   lock_action,             			                
                     ]);  
            }
            
            fn activate(&self) {
        
                let app = self.obj();
                let window = Window::new(&self.model, &app);
                window.present();
                self.window.replace(Some(window.downgrade()));
            }
            fn open(&self, _files: &[gio::File], _hint: &str) 
            {
               //self.activate();
             }
            
          
   }
      impl GtkApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>) 
       @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    
    pub fn run() -> glib::ExitCode  {
    
     tracing::info!("Application ({})", config::APP_ID);
     tracing::info!("Version: {} ({})", config::VERSION, config::PROFILE);

        let app = glib::Object::builder::<Application>()
            .property("application-id", config::APP_ID)
            .property("flags", gio::ApplicationFlags::HANDLES_OPEN)
            .build();
            app.imp().model.load();
        app.run()
        
   
      
    }

  pub fn active_window(&self) -> Window {
        self.imp()
            .window
            .borrow()
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
    }


}
