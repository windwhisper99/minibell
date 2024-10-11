htmx.defineExtension("json-enc", {
  onEvent: function (name, evt) {
    if (name === "htmx:configRequest") {
      evt.detail.headers["Content-Type"] = "application/json";
    }
  },

  encodeParameters: function (xhr, parameters, elt) {
    xhr.overrideMimeType("text/json");
    return JSON.stringify(transform(elt));
  },
});

/**
 * Assign a value to an object at a given path.
 * Path can be object key or array index.
 * If is array index, it must be a string.
 * @param {object} obj
 * @param {string[]} path
 * @param {*} value
 */
function setValue(obj, path, value) {
  let ref = obj;
  for (let i = 0; i < path.length; i++) {
    const key = path[i];

    if (i === path.length - 1) {
      ref[key] = value;
    } else {
      if (!(key in ref)) {
        if (isNaN(path[i + 1])) {
          ref[key] = {};
        } else {
          ref[key] = [];
        }
      }

      ref = ref[key];
    }
  }
}

/**
 * Get value from an input element.
 * If input string is empty, return null.
 * If input type is datetime-local, return value as second timestamp number.
 * @param {HTMLElement} ele
 */
function getValue(ele) {
  if (ele.value === "") return null;
  if (ele.type === "datetime-local") {
    return new Date(ele.value).getTime() / 1000;
  }

  // If input have [data-array] attribute, split value by comma
  if (ele.dataset.array) {
    return ele.value.split(",").map((v) => v.trim());
  }

  return ele.value;
}

/**
 * @param {HTMLFormElement} form
 * @returns
 */
function transform(form) {
  // Map of form elements
  const elements = form.elements;
  const elementMap = {};
  for (let i = 0; i < elements.length; i++) {
    const element = elements[i];
    const name = element.name;

    if (element.name) {
      elementMap[name] = element;
    }
  }

  // Build json object
  const jsonObj = {};
  for (const key in elementMap) {
    const path = key.split(".");
    const value = getValue(elementMap[key]);

    if (value !== null) {
      setValue(jsonObj, path, value);
    }
  }

  return jsonObj;
}
