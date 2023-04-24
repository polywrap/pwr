# Polywrap Wrapper Runner (PWR)

## Description
Polywrap Wrapper Runner (PWR) is a CLI application for running Polywrap wrappers.
It uses the PolywrapClient to execute any wrapper that implements the PwrApp interface wrap://ens/pwr-app.eth
```graphql
type Module {
  main(args: [String!]!): UInt8!
}
```

## Installation
`npm i -g @nerfzael/pwr`

## Examples: 
Input: `pwr wrap://ens/wrap-echo.eth Hello world!`
Output: `Hello world!`

Input: `pwr ens/wrap-echo.eth Hello world!`
Output: `Hello world!`

Input: `pwr wrap-echo.eth Hello world!`
Output: `Hello world!`

Input: `pwr wrap://ipfs/QmebzauKAXoYbywLAYdBvKyPhWsDoHfkUC4wffWvjighKT Hello world!`
Output: `Hello world!`

Input: `pwr ipfs/QmebzauKAXoYbywLAYdBvKyPhWsDoHfkUC4wffWvjighKT Hello world!`
Output: `Hello world!`

Input: `pwr ipfs://QmebzauKAXoYbywLAYdBvKyPhWsDoHfkUC4wffWvjighKT Hello world!`
Output: `Hello world!`

Input: `pwr QmebzauKAXoYbywLAYdBvKyPhWsDoHfkUC4wffWvjighKT Hello world!`
Output: `Hello world!`

## Rust version
### Installation 
Run the following command in the terminal:
```bash
curl -L https://raw.githubusercontent.com/polywrap/pwr/main/pwrup/install | bash
```
Which will install `pwrup`.

Then, to install `pwr`, run:
```bash
pwrup
```
