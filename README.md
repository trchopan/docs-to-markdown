![](https://lh4.googleusercontent.com/lpu7gvVra5dOzvga7hyaIpQ0o8DUssWsq1aC45wguXiCAzalhKkigchrGP2Pd6bine4HKLkf96Fwe6RVZGIspjQ6Rfay6MywKXh_c-KSZrB5-otAy1u2fUeAvTAQKvAz3IGKv0wJKneyvUui)



Convert Google Docs to Markdown.

# Docs to Markdown

Do you like the editing capability of Google Docs and wish I can edit a README.md with it?

This tool will help you convert a published Google Docs to Markdown easily. In fact, this README.md is from this [docs](https://docs.google.com/document/d/e/2PACX-1vTL6dR4i900OLQUeTDs40RD2nPI-yyLzkvMgcDxzNWOkJVlVzpprHqS8Qgp-LGccMOHVwCw76SEQXR5/pub).

Now you can have awesome spell check, grammar correction, and link insert, picture view, tables, and many more.

# Supported features

- [x] Header
- [x] Link
- [x] Table
- [x] Image
- [x] List

# Install

Using Rust `cargo` tool.

```bash
cargo install docs_to_markdown
```

# How to use

### Create a Google Document and make your edits

![](https://lh4.googleusercontent.com/oQ4zaonCAy55ZH0ABSmEGx5ndoPjAXJ4IN_B7mrphjRmS4Hqd06auyvt0keb4x_DKOCpyThJIVFdPk7M4fMLEQCpEYiMgEEJn9bmQf5gxR6cipRntfucuyWbtgDDW3KexxV7i9aJ7OmGc7Jp)

### Publish the document

![](https://lh4.googleusercontent.com/i07GH67fmWleSfRNPTIgqGQ6CnC3K8i2jVLJrhZFO-q-Xb26KlX2lTiDQQLfQhMSlPuhVWPaST24rNvyWbLeoHVjKuzV00SnmomcqLaRGbFYwByqwPrcXTSO3OSj0g-Lm7tqFFJbkxlT5ZFf)

### Get the published link

![](https://lh5.googleusercontent.com/ddE5kYhMeRi3BgW8_LL4_76Ygij098QYnkc6HpyFQlEuXOJpUyhLY-bERGynDL-Pq-lWYl4V3HxgnvjKtK7eFczd44Baf3iQDyRs55JFdB1WMdozOdf4T6WPNBBle3bulae86I_jFNlL1DHV)

### Using cli

```bash
# Export to output
docs_to_markdown <PUBLISH_LINK>

# Export to README.md file
docs_to_markdown <PUBLISH_LINK> > README.md

# Export and copy to clipboard
docs_to_markdown <PUBLISH_LINK> | pbcopy
```

### Using Cloudflare Worker

Please use this endpoint sparingly. I’m running it on a free tier Cloudflare Worker. If you can, feel free to host your own for your convenience.

```bash
curl --request POST 'https://docs-to-markdown.chop.dev/' \
     --header 'Content-Type: application/json' \
     --data-raw "{\"url\": \"$PUBLISH_LINK\"}"
```

# License

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2022-, Quang Tran.
