# Example PWR CLI usage:

## Migration
After running `pwrup`, you have to run `pwr migrate` to migrate your configuration to the latest version.

## Running pwr apps

### From wrapscan.io
To run a pwr app from wrapscan.io, run:
```bash
pwr wrapscan.io/pwr/say-hello
```

### From wrappers.dev
To run a pwr app from wrappers.dev, run:
```bash
pwr @pwr/say-hello
```
```bash
pwr https/wrappers.dev/u/pwr/say-hello
```

### Local directory
To run a pwr app from a local directory (e.g. `./build`), run:
```bash
pwr ./build
```
```bash
pwr fs/build
```

### From IPFS
To run a pwr app from IPFS, run:
```bash
pwr ipfs/Qm...
```
```bash
pwr Qm...
```
```bash
pwr ipfs://Qm...
```

### From ENS
To run a pwr app from ENS, run:
```bash
pwr ens/say-hello.eth
```
```bash
pwr say-hello.eth
```

## Passing arguments to pwr apps
To pass arguments to a pwr app add them after the WRAP URI, e.g.:
```bash
pwr ./build arg1 arg2
```