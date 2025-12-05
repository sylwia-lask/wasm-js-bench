export function jsMatmulSum(n: number): number {
    const MOD = 1_000_000_007;
    let sum = 0;

    for (let i = 0; i < n; i++) {
      for (let k = 0; k < n; k++) {
        const a = (i + k) % 10;
        for (let j = 0; j < n; j++) {
          const b = (k + j) % 10;
          sum += a * b;
          if (sum >= MOD) {
            sum = sum % MOD;
          }
        }
      }
    }

    return sum % MOD;
  }