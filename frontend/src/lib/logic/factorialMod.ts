export function jsFactorialMod(n: number): number {
  const MOD = 1_000_000_007n;

  let res = 1n;
  const bigN = BigInt(n);

  for (let i = 1n; i <= bigN; i++) {
    res = (res * i) % MOD;
  }

  return Number(res);
}
