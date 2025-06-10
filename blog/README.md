# Clams Blog

This is the official repository for the Clams Blog. Visit [the Clams website](https://docs.clams.tech/) to learn more about the app.

## Installation

Run `npm install` to download the tailwind dependencies. The project is now ready to be used.

## Usage

Here's how you use the blog:

```zsh
# installs everything
# that you need
npm install

# build builds once,
# output in `./public`
npm run build

# starts a local server
# that watches/rebuilds
npm run serve

## You can also use `BUILD_OPTS` or `SERVE_OPTS`
## To specifiy options for those commands, e.g.
BUILTS_OPTS="--drafts" npm run build
SERVE_OPTS="--port 1234" npm run serve
```

For example, you can run `npm run serve` and then go to `localhost:1111` in your browser to see the website. As you make changes to the code or content, the website will be updated.

## Content

Content is stored in [markdown text](https://commonmark.org/help/) files located within the `content` directory. Files named `_index.md` are called "sections", and files by any other name ending in `.md` are called "pages". For more information you can read the [zola](https://www.getzola.org/documentation/content/overview/) documentation. The markdown is inserted into the html via the specified template, which is indicated at the top of the content's "front matter".

## Templating

Templates are html files stored in the `templates` directory. Their purpose is to decide where in the html the content goes. The content is accessible to the template as a variable named either `section.content` or `page.content`, depending on the context. Within the template file, you can use a templating language called [tera](http://tera.netlify.app/docs/).

## Styling

This website uses [tailwindcss](https://tailwindcss.com/) for most of its styling. Separate CSS can be added either in the `sass` directory or in static for just plain css. Configuration happens in `input.css`.

## Dependencies and Tools

* [zola](https://getzola.org) `brew install zola`
* node, npm
