/*!slide plugin_config
maxval 100002
class Primes
var primes
ty lli
*/

class $class$ {
  public:
    vector<$ty$> primes;
    vector<$ty$> factor;

    $class$ ($ty$ maxval) {
      factor.resize(maxval);
      for ($ty$ i = 2; i < maxval; i++) {
        if (!factor[i]) {
          primes.push_back(i);
          factor[i] = i;
        }
        for ($ty$ pr : primes) {
          if (i*pr >= maxval || pr > factor[i]) {
            break;
          }
          factor[i*pr] = pr;
        }
      }
    }
};

//!slide plugin_input
$class$ $var$($maxval$);
