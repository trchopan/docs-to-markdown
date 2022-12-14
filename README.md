![](https://lh3.googleusercontent.com/kq20CwBwKdT5771DpRu-WLljK-13YT4zbYAJ2bITAkGHahLL8GtMZTOeVxzj5MHK1H9LeTjb7Cfu3algrx6m3R441LwCS_bMDFKffd4eSEO9tUdc9AQoBNQNINLiZI7TixFw0vHuBQDn7FL-n_yHis_9Y2q2oiREXp7iWl6bGAN3D4Y5ODpyR_4)

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


![](https://lh4.googleusercontent.com/WGvK8J6Qonmgarbadvd6pf2e_o_mCDMwCU7TuPSJr7z0L51U0v-stCuWc22qx9sefgwfNbVdMVibfYS6Mu8Ner59HP7uaHk9jAKNT0-NINonnPX2dZ8T1GWcfBkLUzwqzrvegS-kAv0M2nDXeC1Tdi8pBVdku8V6ehnEqPn8qa14FDB3MUYphtU)

### Publish the document


![](https://lh3.googleusercontent.com/qm0zj2INrJ151Xyjh65gPO5X9xbXAbFlNd3QbfoYlLU8ZYT-sIv1P0oYC_mF1NEvnMSMtb99ESA9hr94jOtomatei9XEsfdRfOG-S3rvsdyRmh3q_WMNv1iRWGX0bJ6KOiiRBFX1pPxXEQzARTXaouMUazSRexOswJApEVsNXpk_433_U66kLVQ)

### Get the published link


![](https://lh3.googleusercontent.com/mU3fwOvOKWFvLQRFkUmtbMcp1IuGoLTkHwxiHvc8aLtDmw-iqRbycm_Uhe9w2rCYJIu09BZ06wErhEZ3r0xTA1Etzu9JXTR57lDnirtKzvvIY3vetwIb8pffaT71Px3NyW_OxjfwXA8wRh3o1iXWJq2YVYcTMK0WVPIwk4ha6mkm54D_cksXDik)

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

Please use this endpoint sparingly. I’m running it on a free tier Cloudflare Worker. If you can, feel free to host your own for your convenient.

```bash
curl --request POST 'https://docs-to-markdown.chop.dev/' \
     --header 'Content-Type: application/json' \
     --data-raw '{"url": "<PUBLISH_LINK>"}'
```

License
---

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2022-, Quang Tran.
