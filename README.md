# Rust Axum Server using Webpack [Hydration]

This is a proof of concept and wip project to show how to generate backend rendered HTML and then use Webpack to hydrate the frontend.

#### Packages being bundled by webpack

- Preact
  - I am using Preact to render and update DOM elements (hydration of server rendered HTML); I am a big fan of JSX and like using this workflow. Much cleaner looking that vanilla JS in my options. I am also using Typescript which allows for cleaner and safer code, plus better IDE support.
- Axios
  - I perfer axios over any other AJAX library; just my go to 😄

#### How to run

- `npm i`
- `npm run build`
- `cargo run`
- `Profit` 💰💰💰

#### Missing

- Frontend `DELETE` method call
- Rerender of frontend on state change
