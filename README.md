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
## Script WRAPS (JS and Python)
{lang} = js | py

### REPL
`pwr {lang} repl`
Runs a REPL where you can type scripts.

`pwr {lang} repl -f {name_of_file}`
Runs the repl over a file.
It will read and execute the file first (and create it if it doesn't exist).
It will also store all CLI commands you type inside of it (after evaluating them).
Press enter in the CLI to re-run the whole file (useful if you want to code in the file instead of the CLI).

`pwr {lang} repl -f {name_of_file} -w`
It will read and execute the file first (and create it if it doesn't exist).
Then it will execute the file every time you save it.
It will not listen to CLI input if you use this option ("-w").

### New 
`pwr {lang} new -f {name_of_file}`
Creates a new file of the specified name for the script wrap.
It uses a template for that language.

### Build
`pwr {lang} build -f {name_of_file}`
Builds the target script file.

### Deploy
`pwr deploy`
Deploys the build directory.

`pwr {lang} deploy`
Deploys the build directory. (Same as `pwr deploy`)

`pwr {lang} deploy -f {name_of_file}`
Builds and then deploys the script file.

## TODO: Examples PWR app usage: 
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


