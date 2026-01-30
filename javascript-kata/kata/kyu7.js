const swap = (s) => {
  return [...s]
    .map((c) => ("aeiou".contains(c) ? c.toUpperCase() : c))
    .join("");
};

const nbDig = (n, d) => {
  return [...Array(n).keys()]
    .map((x) => x * x)
    .join("")
    .match(new RegExp(d, "g")).length;
};
