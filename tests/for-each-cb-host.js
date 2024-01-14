export class CbStringU32ListString {
  #cb;
  constructor(cb) {
    this.#cb = cb;
  }
  run(a, b, c) {
    (0, this.#cb)(a, b, c);
  }
}
export class CbU32U32ListU32 {
  #cb;
  constructor(cb) {
    this.#cb = cb;
  }
  run(a, b, c) {
    (0, this.#cb)(a, b, c);
  }
}
