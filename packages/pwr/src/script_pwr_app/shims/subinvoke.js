//HACK: remove this once codegen is fixed
const __old_subinvoke = __wrap_subinvoke;
__wrap_subinvoke = function (plugin, method, args) {
  if (Array.isArray(args)) {
    return __old_subinvoke(plugin, method, args);
  } else {
    return __old_subinvoke(plugin, method, clean(args));
  }
};

function clean(obj, root = true) {
  const x = JSON.stringify(Array.from(encode(obj)));
  return JSON.parse(x);
}
