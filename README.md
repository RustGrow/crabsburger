# Crabsburger Example

Welcome to the Crabsburger example food landing page site for the [Dioxus community](https://dioxuslabs.com/)!

<div align="center">
  <h3>  
    <a href="https://crabsburger.netlify.app/"> Website </a>
  </h3>
</div>

## About This Example

This example showcases the new syntax introduced in Dioxus 0.6 and utilizes signals as the state management approach. It's designed to demonstrate best practices and provide a reference for building your own Dioxus applications.

### Features

- Utilizes the new syntax of Dioxus 0.6.3
- State management with signals
- Styling with Tailwind CSS v3.4

## Roadmap

- [X] Create a button that appears when scrolling the page
- [X] Highlight the top menu when scrolling
- [X] Implement light and dark mode toggling and set the mode in the HTML tag as required by Tailwind CSS
- [X] Save the color scheme to the browser's local storage
- [X] Retrieve the browser client's language
- [X] Change the language in the HTML tag when switching languages

### Important. This project uses the web platform
# Quick start
1. Reinstall the CLI to the git version.
For Windows users need to install the [Netwide Assembler (NASM)](https://www.nasm.us/pub/nasm/releasebuilds/2.16.03/win64/). On startup it will open the shell and inside execute this command.
```bash
cargo install --git https://github.com/dioxuslabs/dioxus dioxus-cli --locked --force
```
or you can install the CLI (instead of building from source) use:

```bash
cargo binstall dioxus-cli --force
```

2. Clone this repository
```bash
https://github.com/DioxusGrow/crabsburger.git
```
and 👇

# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve
```

- Open the browser to http://localhost:8080

# How to use the Dioxus cli version from github
It often happens that a necessary update is released or fix bugs in a minor version of a project. In this case you can use the github version of cli.

1. Reinstall the CLI:
```bash
cargo install --git https://github.com/dioxuslabs/dioxus dioxus-cli --locked --force
```

2. Check Dioxus cli version
```bash
dx -V
// dioxus 0.6.3 (bdeedc1)
```

3. Add dependencies to the Cargo.toml file. The Dioxus version from github must match the cli version. In this case it is rev = "bdeedc1"

```toml
[dependencies]
dioxus = { git = "https://github.com/DioxusLabs/dioxus", rev = "bdeedc1" features = ["router"] }
dioxus-logger = "0.5.1"

[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
```

4. For existing projects, to synchronize with cli, you must also run the command
`cargo update`

5. Start the Tailwind CSS compiler and the Dioxus dev server in different terminals:
```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```
You can start web version by default:
`dx serve`
or with platform
`dx serve --platform web`

And you can start desktop version:
`dx serve --platform desktop`




# If you need a local stylesheet for custom styles inside input.css.
1. Insert your custom styles inside input.css:
```css
@layer components {
  p {
    @apply p-10 bg-yellow-600;
  }
  .red {
    @apply bg-red-600;
  }
  .yellow {
    @apply bg-yellow-600;
  }
  .blue {
    @apply bg-blue-600;
  }
}
```
2. Insert custom classes into the page:
```rust
rsx!{
    p { "I" }
    div { class: "red", "want to" }
    div { class: "yellow", "burger" }
    div { class: "blue", "burger" }
}
```
3. Rebuild the app:

button r on terminal 

or 

```bash
dx serve
```

# How to make a release

1. Make sure you add the languages folder to the monitoring in the Dioxus.toml file:
```toml
# which files or dirs will be watcher monitoring
watch_path = ["src", "assets", "lang"]
```
2. Use the `dx check` command to check that there are no errors in the logic for using the signals.
```bash
dx check
//output No issues found.
```
3. Make a release for web:
```bash
dx build --release --platform web
```
for desktop:
```bash
dx build --release --platform desktop
```

4. The release is inside `/target/dx/fustfood/release/web/public` folder is by default the main project folder where the finished site is located.

# Netlify deployment

Deploying an application in netlify requires a special settings file to be uploaded to the assets folder so that it is automatically loaded when the project is built. In this repository it is already in the assets folder.

netlify.toml
```toml
[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

## License

This project is open source and available under the MIT License.
