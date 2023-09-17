# Polywrap Wrap Runner (PWR)

## Description
Polywrap Wrap Runner (PWR) is a CLI application for running Polywrap wraps.
It uses the PolywrapClient to execute any wrap that implements the PwrApp interface wrap://ens/pwr-app.eth
```graphql
type Module {
  main(args: [String!]!): UInt8!
}
```

## Installation
Run the following command in the terminal:
```bash
curl -L https://raw.githubusercontent.com/polywrap/pwr/main/pwrup/install | bash
```
Which will install `pwrup`.

Then, to install `pwr`, run:
```bash
pwrup
```

[Example PWR Usage](./docs/pwr-usage.md)

[Example PWR Apps](./docs/pwr-apps.md)

[Script WRAPS Guide](./docs/script-wraps.md)
