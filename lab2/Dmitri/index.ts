// ax + by = e
// cx + dy = f

export const main=(a:number, b:number, c:number, d:number, e:number, f:number)=> {
    if (a == 0 && b == 0 && c == 0 && d == 0 && e == 0 && f == 0) {
      return { code: 5 };
    } else if (a * d - c * b != 0 && (e * d - b * f != 0 || a * f - c * e != 0)) {
      let y = (a * f - c * e) / (a * d - c * b);
      let x = (d * e - b * f) / (d * a - b * c);
      return { code: 2, x0: x, y0: y };
    } else if (
      (a * d - c * b == 0 && (e * d - b * f != 0 || a * f - c * e != 0)) ||
      (a == 0 && c == 0 && e / b != f / d) ||
      (b == 0 && d == 0 && e / a != f / c) ||
      (a == 0 && b == 0 && c == 0 && d == 0 && e / f > 0)
    ) {
      if (
        (a == 0 && b == 0 && e == 0 && d != 0 && c == 0) ||
        (c == 0 && d == 0 && f == 0 && b != 0 && a == 0)
      ) {
        let y:number;
        if (b == 0) y = f / d;
        else if (d == 0) y = e / b;
        else if (e == 0 || f == 0) y = 0;
        return { code: 4, y0: y };
      } else if (
        (a == 0 && b == 0 && e == 0 && c != 0 && d == 0) ||
        (c == 0 && d == 0 && f == 0 && a != 0 && b == 0)
      ) {
        let x:number;
        if (a == 0) x = f / c;
        else if (c == 0) x = e / a;
        else if (e == 0 || f == 0) x = 0;
        return { code: 3, x0: x };
      } else return { code: 0 };
    } else if (a == 0 && c == 0) {
      let y:number;
      if (e == 0) y = f / d;
      else if (f == 0) y = e / b;
      else y = e / b;
      return { code: 4, y0: y };
    } else if (b == 0 && d == 0) {
      let x:number;
      if (e == 0) x = f / c;
      else if (f == 0) x = e / a;
      else x = e / a;
      return { code: 3, x0: x };
    } else if (b == 0 && e == 0) {
      let k:number, n:number;
      k = -c / d;
      n = f / d;
      return { code: 1, k: k, n: n };
    } else if (d == 0 && f == 0) {
      let k:number, n:number;
      k = -a / b;
      n = e / b;
      return { code: 1, k: k, n: n };
    } else if (a == 0 && e == 0) {
      let k:number, n:number;
      k = -d / c;
      n = f / c;
      return { code: 1, k: k, n: n };
    } else if (c == 0 && f == 0) {
      let k, n;
      k = -b / a;
      n = e / a;
      return { code: 1, k: k, n: n };
    } else if (a / b == c / d) {
      let k:number, n:number;
      k = -c / d;
      n = f / d;
      return { code: 1, k: k, n: n };
    } else {
      return "Are you kidding me?";
    }
  }
  
  // ax + by = e
  // cx + dy = f
  
  console.log(main(4, -2, 6, 1, 22, 45));
  