use crate::error_template::{AppError};
use crate::pages::*;
use crate::components::{Header, Footer};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/davidhoeppnerleptos.css"/>

        // sets the document title
        <Title text="davidhoeppner.ca"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <PageNotFound/>
            }
            .into_view()
        }>
            <Header/>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/projects" view=Projects/>
                    <Route path="/contact" view=Contact/>
                    <Route path="/work" view=Work/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
