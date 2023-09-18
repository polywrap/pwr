const fs = {
  readFileSync: (path) => {
    return __wrap_subinvoke("plugin/fs", "readFileSync", clean({ path })).value;
  },
  readFile: (...args) => {
    const callback = args[args.length - 1];
    const result = __wrap_subinvoke("plugin/fs", "readFileSync", clean({ path: args[0] }));
    callback && callback(result.error ? new Error(result.error) : undefined, result.value);
  },
  writeFileSync: (path, data) => {
    return __wrap_subinvoke("plugin/fs", "writeFileSync", clean({ path, data })).value;
  },
  writeFile: (...args) => {
    const callback = args[args.length - 1];
    const result = __wrap_subinvoke("plugin/fs", "writeFileSync", clean({ path: args[0], data: args[1] }));
    callback && callback(result.error ? new Error(result.error) : undefined, result.value);
  },
  appendFileSync: (path, data) => {
    return __wrap_subinvoke("plugin/fs", "appendFileSync", clean({ path, data })).value;
  },
  appendFile: (...args) => {
    const callback = args[args.length - 1];
    const result = __wrap_subinvoke("plugin/fs", "appendFileSync", clean({ path: args[0], data: args[1] }));
    callback && callback(result.error ? new Error(result.error) : undefined, result.value);
  },
  existsSync: (path) => {
    return __wrap_subinvoke("plugin/fs", "existsSync", clean({ path })).value;
  },
  exists: (...args) => {
    const callback = args[args.length - 1];
    const result = __wrap_subinvoke("plugin/fs", "existsSync", clean({ path: args[0] }));
    callback && callback(result.error ? new Error(result.error) : undefined, result.value);
  },
  renameSync: (oldPath, newPath) => {
    return __wrap_subinvoke("plugin/fs", "renameSync", clean({ oldPath, newPath })).value;
  },
  rename: (...args) => {
    const callback = args[args.length - 1];
    const result = __wrap_subinvoke("plugin/fs", "renameSync", clean({ oldPath: args[0], newPath: args[1] }));
    callback && callback(result.error ? new Error(result.error) : undefined, result.value);
  },
  mkdirSync: (path) => {
    return __wrap_subinvoke("plugin/fs", "mkdirSync", clean({ path })).value;
  },
  mkdir: (...args) => {
    const callback = args[args.length - 1];
    const result = __wrap_subinvoke("plugin/fs", "mkdirSync", clean({ path: args[0] }));
    callback && callback(result.error ? new Error(result.error) : undefined, result.value);
  },
  readdirSync: (path) => {
    return __wrap_subinvoke("plugin/fs", "readdirSync", clean({ path })).value;
  },
  readdir: (...args) => {
    const callback = args[args.length - 1];
    const result = __wrap_subinvoke("plugin/fs", "readdirSync", clean({ path: args[0] }));
    callback && callback(result.error ? new Error(result.error) : undefined, result.value);
  },
};

const fsPromises = {
  readFile: (path) => {
    return Promise.resolve(fs.readFileSync(path));
  },
  writeFile: (path, data) => {
    return Promise.resolve(fs.writeFileSync(path, data));
  },
  appendFile: (path, data) => {
    return Promise.resolve(fs.appendFileSync(path, data));
  },
  exists: (path) => {
    return Promise.resolve(fs.existsSync(path));
  },
  rename: (oldPath, newPath) => {
    return Promise.resolve(fs.renameSync(oldPath, newPath));
  },
  mkdir: (path) => {
    return Promise.resolve(fs.mkdirSync(path));
  },
  readdir: (path) => {
    return Promise.resolve(fs.readdirSync(path));
  }
};