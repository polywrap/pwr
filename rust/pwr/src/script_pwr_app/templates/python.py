class Module:
  def hello(self, args):
    a = args["a"]
    b = args["b"]

    return a + b

# To invoke another wrap use __wrap_subinvoke
# e.g. __wrap_subinvoke("script/hello.py", "add", { "a": 1, "b": 2 })
# e.g. __wrap_subinvoke("ens/wraps.eth:http@1.1.0", "get", { "url": "http://google.com" })

getattr(Module(), __wrap_method)(__wrap_args)
