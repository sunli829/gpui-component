window.jsBridge = {
  __internal: {
    call(method, args) {
      var request = {
        method: method,
        args,
      };
      return new Promise((resolve, reject) => {
        window.cefQuery({
          request: JSON.stringify(request),
          persistent: false,
          onSuccess: function (response) {
            resolve(JSON.parse(response));
          },
          onFailure: (error_code, error_message) => reject(error_message),
        });
      });
    },
  },
};
