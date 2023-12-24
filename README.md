# SotoEstevez

TODO introduction

## SEimd

SEimd (SotoEstevez injectable Markdown) is a the backend muscle in the webpage. It's a Rust functionality capable of parsing and inyecting the content of a supersubset (I'll address this later) of Markdown in HTML and JS files. This is used to contain all the relevant content of the commands in simple files that can be easily updated.

SEimd supports and parses the following features of Markdown (that's why it's a subset):
- Headers
- Paragraphs
- Links
- Images
- Bold text
- Italic text
- Strikethrough text
- Single-line code
- Unordered and ordered lists

But it also supports the option to specify some metadata in the form of pairs of keys and values that can be later used with the different processings. For example, the provided HTML injecter can read some metadata to, for example, wrap the parsed HTML content into a specific container with defined classes. Let's see an example of a SEimd file:

```
---
+container: section
+container_classes: console, collapsible, about
---

# About

This is the description of the `about` command to be injected for use in <https://www.sotoestevez.dev>.
**Yay! Overcomplicated things!!**
```

And injecting this Mardown in the following HTML:

```html
<div>
    {{about.md}}
</div>
```

Would end up creating the following snippet:

```html
<div>
    <section class="console, collapsible, about">
        <h1>About</h1>
        <p>This is the description of the <code>about</code> command to be injected for use in <a href="https://www.sotoestevez.dev">https://www.sotoestevez.dev</a>. <strong>Yay! Overcomplated thingss!!</strong></p>
    </section>
</div>
```
