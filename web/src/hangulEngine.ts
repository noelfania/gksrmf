// Rust src/hangul_engine.rs 기반 두벌식 조합 로직 포팅

const ENG_KEY = "rRseEfaqQtTdwWczxvgkoiOjpuPhynbml";
const KOR_KEY = "ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎㅏㅐㅑㅒㅓㅔㅕㅖㅗㅛㅜㅠㅡㅣ";
const CHO_DATA = "ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ";
const JUNG_DATA = "ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ";
const JONG_DATA = "ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ";

const KOR_KEY_ARR = Array.from(KOR_KEY);
const CHO_DATA_ARR = Array.from(CHO_DATA);
const JUNG_DATA_ARR = Array.from(JUNG_DATA);
const JONG_DATA_ARR = Array.from(JONG_DATA);

function charIndex(arr: string[], ch: string): number {
  return arr.indexOf(ch);
}

function charAt(arr: string[], idx: number): string {
  return arr[idx] ?? "\0";
}

function makeHangul(cho: number, jung: number, jong: number): string {
  const jongVal = jong < 0 ? 0 : jong + 1;
  const code = 0xac00 + cho * 21 * 28 + jung * 28 + jongVal;
  return String.fromCodePoint(code);
}

function flush(cho: number, jung: number, jong: number): string {
  if (cho >= 0) {
    if (jung >= 0) return makeHangul(cho, jung, jong);
    return charAt(CHO_DATA_ARR, cho);
  }
  if (jung >= 0) return charAt(JUNG_DATA_ARR, jung);
  if (jong >= 0) return charAt(JONG_DATA_ARR, jong);
  return "";
}

function tryCombineCho(cho: number, p: number): number | null {
  const key = `${cho},${p}`;
  const map: Record<string, number> = {
    "0,9": 2,
    "2,12": 4,
    "2,18": 5,
    "5,0": 8,
    "5,6": 9,
    "5,7": 10,
    "5,9": 11,
    "5,16": 12,
    "5,17": 13,
    "5,18": 14,
    "7,9": 17,
  };
  return map[key] ?? null;
}

function tryCombineJong(jong: number, p: number): number | null {
  const key = `${jong},${p}`;
  const map: Record<string, number> = {
    "0,9": 2,
    "3,12": 4,
    "3,18": 5,
    "7,0": 8,
    "7,6": 9,
    "7,7": 10,
    "7,9": 11,
    "7,16": 12,
    "7,17": 13,
    "7,18": 14,
    "16,9": 17,
  };
  return map[key] ?? null;
}

function tryCombineJung(jung: number, p: number): number | null {
  const key = `${jung},${p}`;
  const map: Record<string, number> = {
    "8,19": 9,
    "8,20": 10,
    "8,32": 11,
    "13,23": 14,
    "13,24": 15,
    "13,32": 16,
    "18,32": 19,
  };
  return map[key] ?? null;
}

function splitJongToCho(jong: number): [number, number] {
  const map: Record<number, [number, number]> = {
    2: [0, 9],
    4: [3, 12],
    5: [3, 18],
    8: [7, 0],
    9: [7, 6],
    10: [7, 7],
    11: [7, 9],
    12: [7, 16],
    13: [7, 17],
    14: [7, 18],
    17: [16, 9],
  };
  if (map[jong]) return map[jong];
  const ch = charAt(JONG_DATA_ARR, jong);
  return [-1, charIndex(CHO_DATA_ARR, ch)];
}

export function convertEngToKor(src: string): string {
  let res = "";
  let cho = -1;
  let jung = -1;
  let jong = -1;

  for (const ch of Array.from(src)) {
    const p = ENG_KEY.indexOf(ch);
    if (p === -1) {
      res += flush(cho, jung, jong);
      cho = -1;
      jung = -1;
      jong = -1;
      res += ch;
      continue;
    }

    if (p < 19) {
      if (jung >= 0) {
        if (cho < 0) {
          res += charAt(JUNG_DATA_ARR, jung);
          jung = -1;
          cho = charIndex(CHO_DATA_ARR, KOR_KEY_ARR[p]);
        } else if (jong < 0) {
          const jongCand = charIndex(JONG_DATA_ARR, KOR_KEY_ARR[p]);
          if (jongCand < 0) {
            res += makeHangul(cho, jung, -1);
            cho = charIndex(CHO_DATA_ARR, KOR_KEY_ARR[p]);
            jung = -1;
          } else {
            jong = jongCand;
          }
        } else {
          const newJong = tryCombineJong(jong, p);
          if (newJong != null) {
            jong = newJong;
          } else {
            res += makeHangul(cho, jung, jong);
            cho = charIndex(CHO_DATA_ARR, KOR_KEY_ARR[p]);
            jung = -1;
            jong = -1;
          }
        }
      } else if (cho < 0) {
        if (jong >= 0) {
          res += charAt(JONG_DATA_ARR, jong);
          jong = -1;
        }
        cho = charIndex(CHO_DATA_ARR, KOR_KEY_ARR[p]);
      } else {
        const combined = tryCombineCho(cho, p);
        if (combined != null) {
          cho = -1;
          jong = combined;
        } else {
          res += charAt(CHO_DATA_ARR, cho);
          cho = charIndex(CHO_DATA_ARR, KOR_KEY_ARR[p]);
        }
      }
    } else {
      if (jong >= 0) {
        const [keepJong, newCho] = splitJongToCho(jong);
        if (cho >= 0) {
          res += makeHangul(cho, jung, keepJong);
        } else if (keepJong >= 0) {
          res += charAt(JONG_DATA_ARR, keepJong);
        }
        cho = newCho;
        jung = -1;
        jong = -1;
      }

      if (jung < 0) {
        jung = charIndex(JUNG_DATA_ARR, KOR_KEY_ARR[p]);
      } else {
        const combined = tryCombineJung(jung, p);
        if (combined != null) {
          jung = combined;
        } else {
          if (cho >= 0) {
            res += makeHangul(cho, jung, -1);
            cho = -1;
          } else {
            res += charAt(JUNG_DATA_ARR, jung);
          }
          jung = charIndex(JUNG_DATA_ARR, KOR_KEY_ARR[p]);
        }
      }
    }
  }

  res += flush(cho, jung, jong);
  return res;
}

export interface ComposerStep {
  committed: string;
  composing: string;
}

export class HangulComposer {
  private keys = "";

  clear(): void {
    this.keys = "";
  }

  hasPending(): boolean {
    return this.keys.length > 0;
  }

  feedKey(key: string): ComposerStep {
    this.keys += key;
    return this.splitCommittedAndTail();
  }

  popKey(): string {
    this.keys = this.keys.slice(0, -1);
    return convertEngToKor(this.keys);
  }

  private splitCommittedAndTail(): ComposerStep {
    const text = convertEngToKor(this.keys);
    const chars = Array.from(text);
    if (chars.length <= 1) {
      return { committed: "", composing: text };
    }

    const last = chars.pop() ?? "";
    const committed = chars.join("");
    const composing = last;

    const keyChars = Array.from(this.keys);
    let suffix = "";
    for (let start = 0; start < keyChars.length; start += 1) {
      const cand = keyChars.slice(start).join("");
      if (convertEngToKor(cand) === composing) {
        suffix = cand;
        break;
      }
    }
    this.keys = suffix || this.keys;
    return { committed, composing };
  }
}
