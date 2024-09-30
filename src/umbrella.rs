use fltk::{
    self, 
    app::{self, App, Scheme}, 
    enums::{Color, FrameType}, 
    prelude::{FltkError, GroupExt, WidgetBase, WidgetExt, WindowExt}, 
    window::Window
};


pub struct Umbrella {
    app: App,
    window: Window
}

impl Umbrella {
    pub fn new(scheme: Scheme, window_size: (i32, i32)) -> Self {
        let app = App::default().with_scheme(scheme);
        let window = Window::default().with_size(window_size.0, window_size.1);

        Umbrella {
            app,
            window
        }
    }

    pub fn get_app(&self) -> &App {
        &self.app
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn add_widget<T: WidgetBase + WidgetExt>(&mut self, widget: T) {
        self.window.begin();

        self.window.add(&widget);
        
        self.window.end();        
    }

    pub fn run(&mut self) -> Result<(), FltkError> {
        self.window.show();
        self.app.run()
    }
}
