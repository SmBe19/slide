/*!slide plugin_config
fme fme
modinvers modinvers
ty lli
*/

// a^x % mod
$ty$ $fme$($ty$ a, $ty$ x, $ty$ mod) {
  $ty$ res = 1;
  while (x > 0) {
    if (x & 1) {
      res = (res * a) % mod;
    }
    x >>= 1;
    a = (a*a) % mod;
  }
  return res;
}

// res * a = 1 (MOD mod)
$ty$ $modinvers$($ty$ a, $ty$ mod) {
  return $fme$(a, mod-2, mod);
}
