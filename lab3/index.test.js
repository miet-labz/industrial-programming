const main = require("./index");

describe("All tests", () => {
  describe("Any pair of numbers (x, y) is a solution", () => {
    it("should return { code: 5 }", () => {
      expect(main(0, 0, 0, 0, 0, 0)).toEqual({ code: 5 });
    });
  });

  describe("One unique solution", () => {
    it.each([
      [7, 0, 2, 0, 2, 2, 14, 14],
      [0, 7, 2, 2, 0, 2, 14, 14],
    ])(
      `should return { code: 2, x0: %f, y0: %f }`,
      (x0, y0, a, b, c, d, e, f) => {
        expect(main(a, b, c, d, e, f)).toEqual({ code: 2, x0: x0, y0: y0 });
      }
    );
  });
  describe("No solutions", () => {
    it("should return { code: 0 }", () => {
      expect(main(2, 3, 4, 6, 2, 9)).toEqual({ code: 0 });
    });
  });

  describe("Infinitely many solutions", () => {
    describe("y = k * x + n", () => {
      it.each([
        [3, 2, 0, 0, -30, 10, 0, 20],
        [3, 2, -30, 10, 0, 0, 20, 0],
        [3, 0, 0, 100, 10, -30, 0, 0],
        [3, 0, 10, -30, 0, 20, 0, 0],
        [-9 / 7, 3 / 7, 9, 7, 19 * 9, 19 * 7, 3, 19 * 3],
      ])(
        `should return { code: 1, k: %f, n: %f }`,
        (k, n, a, b, c, d, e, f) => {
          expect(main(a, b, c, d, e, f)).toEqual({ code: 1, k: k, n: n });
        }
      );
    });

    describe("x = x0, y - any", () => {
      it.each([
        [7, 0, 0, 3, 0, 0, 21],
        [7, 3, 0, 0, 0, 21, 0],
        [0, 3, 0, 3, 0, 0, 0],
        [7, 3, 0, 3, 0, 21, 21],
      ])(`should return { code: 3, x0: %f }`, (x0, a, b, c, d, e, f) => {
        expect(main(a, b, c, d, e, f)).toEqual({ code: 3, x0: x0 });
      });
    });
    describe("y = y0, x - any", () => {
      it.each([
        [7, 0, 0, 0, 4, 0, 28],
        [7, 0, 4, 0, 0, 28, 0],
        [0, 0, 4, 0, 4, 0, 0],
        [7, 0, 4, 0, 4, 28, 28],
        [7, 0, 4, 0, 0, 28, 0],
      ])(`should return { code: 4, y0: %f }`, (y0, a, b, c, d, e, f) => {
        expect(main(a, b, c, d, e, f)).toEqual({ code: 4, y0: y0 });
      });
    });
  });
});
