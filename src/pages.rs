#![allow(non_snake_case)]
use leptos::*;
use crate::components::ExternalLink;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
      <h2> "Some Things I’ve Built" </h2>
      <p> "put some projects here in for loop or something" </p>
    }
}

#[component]
pub fn Work() -> impl IntoView{
    view! {
      "Work"
    }
}

#[component]
pub fn Contact() -> impl IntoView{
  let email = "email@davidhoeppner.ca";
  view! {
    <h2> "Get in Touch" </h2>
    <p> "I currently work at Workday as a Software Engineer in Vancouver, Canada as a part of the Search team. Don't hesitate to reach out to me if you have any questions or just want to say hi!" </p>
    <ExternalLink link_to=format!("mailto:{email}") text="Say Hello"/>
  }
}

#[component]
pub fn PageNotFound() -> impl IntoView {
  view! {
    <h1> "Page not found" </h1>
    <p> "We are terribly sorry, but the page you requested doesn't exist." </p>
  }
}

#[component]
pub fn Home() -> impl IntoView {
  let first = view! { <h1> "Hi, my name is" </h1>};
  let second = view! { <h2> "David Hoeppner" </h2>};
  let third = view! { <h3> "I build things for people that build things for the web." </h3>};
  let fourth = view! {
    <p>
      "I’m a Canadian full-stack software developer specializing in Javascript-focused full-stack engineering. Currently, I’m focused on building accessible, human-centered software at "
      <ExternalLink link_to="https://www.workday.com/" text="Workday" />
      "."
    </p>
  };

  let skills = vec![
    "JavaScript (ES6+)",
    "TypeScript",
    "React",
    "Kubernetes",
    "Webflux",
    "Docker",
    "Leptos",
    "Jenkins",
  ];

  view! {
    { first }
    { second }
    { third }
    { fourth }
    <p>
      "Hello! My name is David and I enjoy creating things that create other things. I had a meager start to web development back in 2013 when I experimented with editing some Tumblr themes, but I really started to get involved in 2018 when I interned for Intuit in Edmonton. Working on the QuickBooks Online devops team gave me a kickstart into tooling and operations."
    </p>
    <p>
      "Fast-forward to today, and I've had the privilege of working at "
      <ExternalLink link_to="https://www.intuit.com/" text="Intuit"/>
      " for 5 years, both as an intern and full time engineer. I currently work on the AppFabric Team, which provides cross-app support for 100s of Intuit applications, including QuickBooks Online, TurboTax Online,and Mint."
    </p>
    <p>
      "Here are a few technologies I've been working with recently:"
    </p>
    <ul>
      {skills.into_iter()
        .map(|skill| view! {<li>{skill}</li>})
        .collect_view()}
    </ul>
  }
}
