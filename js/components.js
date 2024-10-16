import Alpine from "alpinejs";

Alpine.data("switcher", (checked = false) => ({
  checked,

  toggle() {
    this.checked = !this.checked;
  },
  root: {
    [":class"]() {
      return this.checked ? "active" : "";
    },
    ["@click"]() {
      this.toggle();
    },
  },
}));
