# Data Access

You can browse the archive on the [website](https://openstreetlifting.org) or use
the public API to access the data in your own projects. The data is dedicated to
the public domain under CC0, so you can freely use, modify and share it. See
[Licensing](./LICENSING.md) for details.

You do not have to give credit, but a mention of OpenStreetlifting would make me
happy.

## Public API

The API provides competition results, athlete profiles and rankings as JSON. It
is read-only and requires no account or API key.

The base URL is:

```text
https://api.openstreetlifting.org/api/v1
```

For example, this request returns a page of competitions:

```sh
curl 'https://api.openstreetlifting.org/api/v1/competitions?page=1&page_size=20'
```

Lists are split into pages. Use `page` to choose a page, starting at `1`, and
`page_size` to request between `1` and `100` results. The response includes
pagination details so you can request the remaining pages.

For available requests, filters and response fields, see the
[API reference](https://api.openstreetlifting.org/swagger-ui/). You can also find
the [OpenAPI specification](https://github.com/openstreetlifting/openstreetlifting/blob/main/backend/openapi.json)
and [Bruno request examples](https://github.com/openstreetlifting/openstreetlifting/tree/main/osl-bruno)
on GitHub.

## Bulk download

A download of the full archive is coming soon. It is not available yet; this page
will be updated when it is ready.

## Getting help

If you need help accessing the data, [contact me](https://openstreetlifting.org/contact)
and describe what you would like to use it for.
