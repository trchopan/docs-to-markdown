![](https://lh6.googleusercontent.com/fS5TzPP94P7LIsX66TGJA1400AT_6DJPvppPhttfvww9E69GOQYEdHDpu_Hk_RWu6QTEWXYFpBb-DqKSnvPswe6IikQ4IhOMagoV5_j-ZXZR1yPqYGah5dXZy4fawPysX8BI2BCU_Dj0manC0hHKRUrFOmmskQzT92GylEqLuzxTPadbOBh2x3Q)

Convert Google Docs to Markdown.

Docs to Markdown
---

Do you like the editing capability of Google Docs and wish I can edit a README.md with it?

This tool will help you convert a published Google Docs to Markdown easily. In fact, this README.md is from this [docs](https://docs.google.com/document/d/e/2PACX-1vTL6dR4i900OLQUeTDs40RD2nPI-yyLzkvMgcDxzNWOkJVlVzpprHqS8Qgp-LGccMOHVwCw76SEQXR5/pub).

Now you can have awesome spell check, grammar correction, and link insert, picture view, tables, and many more.

Supported features
---

- [x] Header
- [x] Link
- [x] Table
- [x] Image
- [x] List

Install
---

Using Rust `cargo` tool.

```bash
cargo install docs_to_markdown
```

How to use
---

### Create a Google Document and make your edits


![](https://lh6.googleusercontent.com/SRBlMUGwJZy8DhLsqXG60c9JB47Zp8cMtZnyER6kIptroBD1YNk5Q0YuBU45FvsFmyrr919ONsoi-4WLqmxPLJuSVn5-6ZnVsdmpeuemLr6u5AhxE-Edo9rn3OZM9ivE0IjlCScW0uVjG1qJJQvKqFISqPz9XpbVob_dSqUoQHOX25FhIPGbrwE)

### Publish the document


![](https://lh5.googleusercontent.com/KekGHks3O-OTD58yZYQ5T-91KdP9LxAKitt1vMoLlz2o4z7R9VpRRknr7qwvhUENelGlml7AgQcxBRoS6TTrCeAV81fRZ0Phr5Vx9Z6l5e0jVwgkPhoC6dzU45YxGCNMd8E3Dk9WcIb0hUG9g97rH5gRMQO_e-SlfNXRdHkyP0Od5awUkMTNAFY)

### Get the published link


![](https://lh6.googleusercontent.com/DRpmVzFJiRldXOV3rK0s6TbqRmpMRmsfzTJrIoNkKbdNQiOHSzgAE48x1E7lND7lW6PKlhMy7bkqWZcwPmERCB_nLwPVp-MQbtQQBfFXuFaqwx_W42VouXdFX42haQYvxTmamtq9on22QNFf0td1ojAu9PZ0HUFtvS3mcCNM00QYLmyXnqlnSKs)

### Exporting command


```bash
# Export to output
docs_to_markdown <PUBLISH_LINK>

# Export to README.md file
docs_to_markdown <PUBLISH_LINK> > README.md

# Export and copy to clipboard
docs_to_markdown <PUBLISH_LINK> | pbcopy
```

License
---

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2022-, Quang Tran.
