// zeroOrOne = (n, s) => { };
//   let max_by_col = new Array();
//   for (i = 0; i < n; ++i) {
//     let sum = 0;
//     for (j = 0; j < n; ++j) {
//       sum += s[j][i];
//     }
//     max_by_col.push(sum > n / 2 ? 1 : 0);
//   }
//   return res;
// };

zeroOrOne = (n, s) => {
  m = [];
  for (i = 0; i < s[0].length; ++i) {
    x = 0;
    for (j = 0; j < n; ++j) x += s[j][i];

    m.push(x > n / 2 ? 1 : 0);
  }
  return m;
};

zeroOrOne = (e, r) =>
  r[0].map((O, a) => (r.reduce((e, r) => e + r[a], 0) > e / 2 ? 1 : 0));

console.log(zeroOrOne(1, [[1, 1, 0, 1]]));
