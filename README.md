# Crabsburger Example

Welcome to the Crabsburger example food landing page site for the [Dioxus community](https://dioxuslabs.com/)!

<div align="center">
  <h3>  
    <a href="https://crabsburger.netlify.app/"> Website </a>
  </h3>
</div>

## About This Example

This example showcases the new syntax introduced in Dioxus 0.5 and utilizes signals as the state management approach. It's designed to demonstrate best practices and provide a reference for building your own Dioxus applications.

### Features

- Utilizes the new syntax of Dioxus 0.5
- State management with signals
- Styling with Tailwind CSS v3.4.1

## **How to Run Locally:**

Clone the repository:

`git clone https://github.com/DioxusGrow/crabsburger`

Comment out this parameter in `Dioxus.toml` as it is intended only for redirection in web:

```
[web.app]
# base_path = "."
```

Run the command:
`dx serve`

If you want to change the Tailwind CSS in a different terminal run:
`npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch`

## **How to Run in the Web:**

Uncomment the parameter in `Dioxus.toml` as it is intended only for redirection in web:

```
[web.app]
base_path = "."
```

Run the command:
`dx build --release`

The release or the ready website is located in the `webapp` folder.

You can rename the folder in the `Dioxus.toml` parameter. For example, for use in GitHub Pages, the folder name should be `docs`

```
# `build` & `serve` dist path
pathout_dir = "webapp"
```

However, the site with GitHub Pages will only work on the main domain, such as `https://dioxuslabs.com`
It will work on a subdomain, but with a greater number of issues:
[Routing errors and the use of Manganis if the site is located in a subdirectory of the domain, as on GitHub Pages. ](https://github.com/DioxusLabs/dioxus/issues/2093)


## Roadmap

- [X] Create a button that appears when scrolling the page
- [X] Highlight the top menu when scrolling
- [X] Implement light and dark mode toggling and set the mode in the HTML tag as required by Tailwind CSS
- [X] Save the color scheme to the browser's local storage
- [ ] Retrieve the browser client's language
- [ ] Change the language in the HTML tag when switching languages

## Browser JS Interaction

Please note that the interaction with the browser's JavaScript needs to be refined in this example.

## Contributing

We welcome contributions! If you have suggestions or want to contribute to the roadmap, please feel free to open an issue or submit a pull request.

## License

This project is open source and available under the MIT License.
