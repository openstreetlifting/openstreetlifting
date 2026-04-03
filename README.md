A strong README for this kind of project covers these sections in this order:

What it is and why it exists

One sharp paragraph. What is OpenStreetLifting, what problem does it solve, why does it exist compared to existing solutions. This is what convinces someone to care in the first 10 seconds.

Live link and screenshot or data preview

A table of recent meet results or a screenshot of the interface. Data projects need to show the data immediately.

Data coverage

Which federations are currently supported, which are in progress, which are not planned and why. This is the most common question contributors will have.

How the data pipeline works

A short prose explanation of bronze to silver to gold. Where data comes from, how it gets normalized, how often it updates. This is unique to your project and sets expectations.

Running it locally

Exact commands, no ambiguity:

git clone <https://github.com/your-org/openstreetlifting>
docker compose up -d
cd backend && cargo run --bin osl_api

Prerequisites listed explicitly: Rust stable, Node 22, pnpm 10, Docker.

Project structure

A brief tree with one line explaining each major directory. People want to orient themselves before reading code.

Contributing

Point to CONTRIBUTING.md but summarize the two or three most wanted contributions: new federation parsers, data corrections, frontend improvements.

Data sources and licensing

Where the raw data comes from, what license applies to the aggregated data versus the code. This is legally important and communities care about it.

License

One line with the license name and a link to LICENSE.

What you do not need

Badges beyond CI status are noise. A lengthy roadmap belongs in GitHub Projects or Issues, not the README. Installation instructions for production deployment belong in docs/, not the README.
