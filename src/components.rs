use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
  view! {
      <footer>
          "Footer"
      </footer>
  }
}

#[component]
pub fn Header() -> impl IntoView {
  view! {
      <header>
          <nav>
              <ul>
                  <li>
                    <a href="/"> "Home" </a>
                  </li>
                  <li>
                    <a href="/projects"> "Projects" </a>
                  </li>
                  <li>
                    <a href="/work"> "Work" </a>
                  </li>
                  <li>
                    <a href="/contact"> "Contact" </a>
                  </li>
              </ul>
          </nav>
      </header>
  }
}

#[component]
pub fn ExternalLink(link_to: impl Into<String>, text: impl Into<String>) -> impl IntoView {
  view! {
    <a
      href=link_to.into()
      target="_blank"
    >
      {text.into()}
    </a>

  }
}