/*!slide plugin_config
fme fme
modinvers modinvers
*/

// a^x % mod
long long int $fme$(long long int a, long long int x, long long int mod) {
  long long int res = 1;
  while (x > 0) {
    if (x & 1) {
      res = (res * a) % mod;
    }
    x >>= 1;
    x = (x*x) % mod;
  }
  return res;
}

// res * a = 1 (MOD mod)
long long int $modinvers$(long long int a, long long int mod) {
  return $fme$(a, mod-2, mod);
}
