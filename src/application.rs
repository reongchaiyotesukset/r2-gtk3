use gtk::{
    gio,
    glib::{self,clone},
    subclass::prelude::*,
    glib::Properties,
    prelude::*,
};
use crate::{
    widgets::{Window,PreferencesWindow},
    models::{ProvidersModel,start as start_search_provider},
    utils::{spawn}, //new
};
use crate::config;

//new
use std::rc::Rc;

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

                                 
                    let button1_action = gio::ActionEntry::builder("button1")
                   .activate(|app: &Self::Type, _, _| {           
                           
                           if app.is_locked(){
							   println!("if");
						   }else{
							   println!("else");
						   }
                    }).build();
                
                   
                    app.add_action_entries([
                       button1_action,
                     			   			               
                     ]);  
                     
                   let button1_action = app.lookup_action("button1").unwrap();
                     
                    app.bind_property("is_locked", &button1_action, "enabled")
					.invert_boolean()
					.sync_create()
					.build();


              //new
              spawn(clone!(
              @strong
                 app => async move {

                    app.start_search_provider().await;
                }
            ));

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

   async fn start_search_provider(&self) {
   /*
          let mut receiver = match start_search_provider().await{
                Err(err) => {
                tracing::error!("Failed to start search provider {err}");
                return;
            }
            Ok(receiver) => receiver,
                         _ => todo!(),

        };
  */
    let mut receiver = match start_search_provider().await{

     _ => todo!(),

    };
   }

}
