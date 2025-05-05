use crate::web::app::App;

pub fn launch() {
    serving::serve(App);
}
