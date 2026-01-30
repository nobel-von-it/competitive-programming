const mult3and5 = (n) => {
  return n >= 0
    ? 0
    : Array.from(Array(n).keys())
        .filter((x) => x % 3 == 0 || x % 5 == 0)
        .reduce((acc, x) => acc + x);
};

const getCount = (str) => {
  const vow = "aeiou";
  return [...str].filter((c) => vow.includes(c)).length;
};

const spinWords = (s) => {
  return s
    .split(" ")
    .map((w) => {
      if (w.length > 4) {
        w = [...w].reverse().join("");
      }
      return w;
    })
    .join(" ");
};

const findOdd = (a) => {
  let nums = new Map();
  a.forEach((x) => {
    nums.has(x) ? nums.set(x, nums.get(x) + 1) : nums.set(x, 1);
  });
  for (const [k, v] of nums) {
    if (v % 2 != 0) {
      return k;
    }
  }
};

const countBits = (n) => {
  return [...n.toString(2)].filter((c) => c == "1").length;
};

const duplicateCount = (t) => {
  let counter = new Map();
  [...t.toLowerCase()].forEach((c) => {
    counter.has(c) ? counter.set(c, counter.get(c) + 1) : counter.set(c, 1);
  });
  return counter.reduce((acc, [k, v]) => {
    if (v > 1) acc += 1;
  });
};

const alphabetPosition = (t) => {
  return [...t]
    .filter((c) => c.match(/[a-z]/i))
    .map((c) => parseInt(c, 36) - 10);
};

const toCamelCase = (s) => {
  return s
    .split(/_|-/)
    .map(
      (w, i) => (i > 0 ? w.charAt(0).toUpperCase() : w.charAt(0)) + w.slice(1),
    )
    .join("");
};

const tribonacci = (sig, n) => {
  if (n === 0) return [];
  else if (n > 0 && n < 4) return sig.slice(0, n);
  else {
    for (i = 0; i < n - 3; i++) {
      sig.push(sig[i] + sig[i + 1] + sig[i + 2]);
    }
    return sig;
  }
};

const isPangram = (s) => {
  let cs = new Set();
  [...s.toLowerCase()]
    .filter((c) => c.match(/[a-z]/g))
    .forEach((c) => cs.add(c));
  return cs.size == 26;
};

const parse = (d) => {
  let res = new Array();
  [...d].reduce((mem, o) => {
    switch (o) {
      case "i":
        mem += 1;
        break;
      case "d":
        mem -= 1;
        break;
      case "s":
        mem *= mem;
        break;
      case "o":
        res.push(mem);
        break;
    }
  }, 0);
  return res;
};
