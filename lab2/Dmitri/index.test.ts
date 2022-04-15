import {main} from "./index"

describe("Test code 5", () => {
    it("should return { code: 5 }", () => {
      expect(main(0, 0, 0, 0, 0, 0)).toEqual({ code: 5 });
    });
});

describe("Test code 4", () => {
    it.each([
        [0, 0, 0, 4, 0, 28, 7],
        [0, 4, 0, 0, 28, 0, 7],
        [0, 4, 0, 4,  0,  0, 0],
        [0, 4, 0, 4, 28, 28, 7],
        [0, 4, 0, 0, 28,  0, 7],
      ])(
        `should return { code: 4, y0: %f }`,
        (a:number, b:number, c:number, d:number, e:number, f:number, y0:number) => {
          expect(main(a, b, c, d, e, f)).toEqual({ code: 4, y0: y0 });
        }
      );
});

describe("Test code 3", () => {
    it.each([
        [0, 0, 3, 0, 0, 21, 7],
        [3, 0, 0, 0, 21, 0, 7],
        [3, 0, 3, 0,  0,  0, 0],
        [3, 0, 3, 0, 21, 21, 7],
      ])(
        `should return { code: 3, x0: %f }`,
        (a:number, b:number, c:number, d:number, e:number, f:number, x0:number) => {
          expect(main(a, b, c, d, e, f)).toEqual({ code: 3, x0: x0 });
        }
      );
});

describe("Test code 2", () => {
    it.each([
    [2, 0, 2, 2, 14, 14, 7, 0],
    [2, 2, 0, 2, 14, 14, 0, 7],
      ])(
        `should return { code: 2, x0: %f, y0: %f }`,
        (a:number, b:number, c:number, d:number, e:number, f:number, x0:number, y0:number) => {
          expect(main(a, b, c, d, e, f)).toEqual({ code: 2, x0: x0, y0: y0 });
        }
      );
});

describe("Test code 1", () => {
    it.each([
        [0, 0, -30, 10, 0, 20, 3, 2],
        [-30, 10, 0, 0, 20, 0, 3, 2],
        [0, 100, 10, -30, 0, 0, 3, 0],
        [10, -30, 0, 20, 0, 0, 3, 0],
        [9, 7, 19 * 9, 19 * 7, 3, 19 * 3, -9 / 7, 3 / 7],
      ])(
        `should return { code: 1, k: %f, n: %f }`,
        (a:number, b:number, c:number, d:number, e:number, f:number, k:number, n:number) => {
          expect(main(a, b, c, d, e, f)).toEqual({ code: 1, k: k, n: n });
        }
      );
});

describe("Test code 0", () => {
    it(`should return { code: 0 }`,
        () => {
          expect(main(0, 0, 0, 0, 5, 12)).toEqual({ code: 0});
        }
      );
});

