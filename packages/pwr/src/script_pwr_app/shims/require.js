function require(lib) {
  function wrapLib(objName, obj) {
    const origin = {};
    return new Proxy(origin, {
      get(_, name) {
        if (obj[name]) {
          return obj[name];
        } else {
          throw new Error(`No method ${name} in ${objName}`);
        }
      },
    });
  }

  switch (lib) {
    case "fs":
      return wrapLib("fs", {
        ...fs,
        promises: fsPromises, 
      });
    case "util":
      return wrapLib("util", util);
    case "axios":
      return wrapLib("axios", axios);
    default:
      throw new Error(`Cannot do require('${lib}'), '${lib}' is an unknown import.`);
  }
}
