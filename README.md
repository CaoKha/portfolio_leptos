# Portfolio in WASM

Kha Nguyen's portfolio: Rust + Leptos (CSR), compiled to WebAssembly with Trunk and deployed to
GitHub Pages.

**Live site:** https://caokha.github.io/portfolio_leptos/

## Stack

- Rust / Leptos 0.6 / leptos_router / leptos-use
- Tailwind CSS
- Trunk (WASM bundler)

## Local development

```sh
npm install
trunk serve
```

`trunk serve` uses the site root. The deployed build is served from a repository sub-path, so the
workflow passes `--public-url "/<repo>/"`; the app reads that URL from the document's `<base>` tag
and prefixes its own links and assets.

## Deployment

Pushes to `master` run `.github/workflows/deploy.yml` and publish `dist/` to GitHub Pages.

The workflow calls `actions/configure-pages` with `enablement: true`, so it creates and enables
the Pages site (source: GitHub Actions) on the first run. No manual repository setup is required.

GitHub Pages has no SPA rewrite rule, so the workflow copies `index.html` to `404.html`; deep
links such as `/Projects/DlcMarkets` boot the router client-side.
