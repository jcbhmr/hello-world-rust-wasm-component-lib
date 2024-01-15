export class PString {
  constructor(cb) {
    this.cb = cb;
  }
  call(a) {
    this.cb(a);
  }
}
export class RString {
  constructor(cb) {
    this.cb = cb;
  }
  call() {
    return this.cb();
  }
}
