
const util = (function() {
  const exports = {};

  var getOwnPropertyDescriptors = 
    function getOwnPropertyDescriptors(obj) {
      var keys = Object.keys(obj);
      var descriptors = {};
      for (var i = 0; i < keys.length; i++) {
        descriptors[keys[i]] = Object.getOwnPropertyDescriptor(obj, keys[i]);
      }
      return descriptors;
    };
      
  var kCustomPromisifiedSymbol = undefined;

  exports.promisify = function promisify(original) {
    if (typeof original !== 'function')
      throw new TypeError('The "original" argument must be of type Function');

    if (kCustomPromisifiedSymbol && original[kCustomPromisifiedSymbol]) {
      var fn = original[kCustomPromisifiedSymbol];
      if (typeof fn !== 'function') {
        throw new TypeError('The "util.promisify.custom" argument must be of type Function');
      }
      Object.defineProperty(fn, kCustomPromisifiedSymbol, {
        value: fn, enumerable: false, writable: false, configurable: true
      });
      return fn;
    }

    function fn() {
      var promiseResolve, promiseReject;
      var promise = new Promise(function (resolve, reject) {
        promiseResolve = resolve;
        promiseReject = reject;
      });

      var args = [];
      for (var i = 0; i < arguments.length; i++) {
        args.push(arguments[i]);
      }
      args.push(function (err, value) {
        if (err) {
        promiseReject(err);
        } else {
        promiseResolve(value);
        }
      });

      try {
        original.apply(this, args);
      } catch (err) {
        promiseReject(err);
      }

      return promise;
    }

    Object.setPrototypeOf(fn, Object.getPrototypeOf(original));

    if (kCustomPromisifiedSymbol) Object.defineProperty(fn, kCustomPromisifiedSymbol, {
      value: fn, enumerable: false, writable: false, configurable: true
    });
    return Object.defineProperties(
      fn,
      getOwnPropertyDescriptors(original)
    );
  }

  return exports;
})();