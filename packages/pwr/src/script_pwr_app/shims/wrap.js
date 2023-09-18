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
