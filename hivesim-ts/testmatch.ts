export class TestMatcher {
  suite: string;
  test: string;
  pattern: string;

  constructor(suite: string, test: string, pattern: string) {
    this.suite = suite;
    this.test = test;
    this.pattern = pattern;
  }

  static parseTestPattern(p: string): TestMatcher {
    const parts = TestMatcher.splitRegExp(p);
    const suite = (parts[0]);
    let test = ''
    if (parts.length > 1) {
      test = (parts.slice(1).join("/"));
    }
    const pattern = p;
    return new TestMatcher(suite, test, pattern);
  }

  // match checks whether the pattern matches suite and test name.
  match(suite: string, test: string): boolean {
    if (!RegExp(this.suite).test(suite)) {
      return false;
    }
    if (test !== "" && this.test !== undefined && !RegExp(this.test).test(test)) {
      return false;
    }
    return true;
  }

  // splitRegExp splits the expression s into /-separated parts.
  //
  static splitRegExp(s: string): string[] {
    const a: string[] = [];
    let cs = 0;
    let cp = 0;
    for (let i = 0; i < s.length;) {
      switch (s[i]) {
      case "[":
        cs++;
        break;
      case "]":
        if (cs-- === 0) { // An unmatched ']' is legal.
          cs = 0;
        }
        break;
      case "(":
        if (cs === 0) {
          cp++;
        }
        break;
      case ")":
        if (cs === 0) {
          cp--;
        }
        break;
      case "\\":
        i++;
        break;
      case "/":
        if (cs === 0 && cp === 0) {
          a.push(s.slice(0, i));
          s = s.slice(i + 1);
          i = 0;
          continue;
        }
        break;
      }
      i++;
    }
    a.push(s);
    return a;
  }
}