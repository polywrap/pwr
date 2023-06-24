const http = wrap("ens/wraps.eth:http@1.1.0");
const fs = wrap("ens/wraps.eth:file-system@1.0.0");

class Module {
  add(args) {
    return args.a + args.b;
  }
}

// To invoke another wrap e.g. http write the following:
// const response = http.get({ url: "http://google.com" }).unwrap();
  
function wrap(uri) {
  const origin = {};
  return new Proxy(origin, {
    get(_, name) {
      return function (args) {
        const result = __wrap_subinvoke(uri, name, args);

        return {
          ...result,
          unwrap: () => {
            if (!result.ok) {
              throw result.error;
            }
            return result.value;
          }
        }
      };
    },
  });
}

new Module()[__wrap_method](__wrap_args);