const console = {
  log: function(...args) {
    __wrap_debug_log(args.map((arg) => typeof arg === "string" ? arg : JSON.stringify(arg, null, 2)).join(" "));
  },
};
