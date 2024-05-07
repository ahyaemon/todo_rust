# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload --port 3000
npx tailwindcss -i ./assets/input.css -o ./assets/tailwind.css --watch
```

- Open the browser to http://localhost:3000

Run on production:
```bash
dx build --release
cd dist
miniserve --spa --index index.html -p 3000
```
