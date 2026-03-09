document.addEventListener("alpine:init", () => {
  const counterForm = () => ({
    counter: 0,
    date: "Pick a date",

    increment() {
      this.counter++;
    },
    decrement() {
      this.counter--;
    },
    onDateChanged(date) {
      this.date = date;
    },
    get textDisplay() {
      return `The current count is ${this.counter}`;
    },
  });

  Alpine.data("counterForm", counterForm);
});
