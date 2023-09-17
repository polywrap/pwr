const axios = {
  get: (url, config) => {
    return Promise.resolve(__wrap_subinvoke("plugin/axios", "get", clean({ url, config })).value);
  },
  post: (url, data, config) => {
    return Promise.resolve(__wrap_subinvoke("plugin/axios", "post", clean({ url, data, config })).value);
  },
  put: (url, data, config) => {
    return Promise.resolve(__wrap_subinvoke("plugin/axios", "put", clean({ url, data, config })).value);
  },
  delete: (url, config) => {
    return Promise.resolve(__wrap_subinvoke("plugin/axios", "delete", clean({ url, config })).value);
  },
  head: (url, config) => {
    return Promise.resolve(__wrap_subinvoke("plugin/axios", "head", clean({ url, config })).value);
  },
};
