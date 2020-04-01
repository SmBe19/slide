/*!slide plugin_config
maxval 100002
class Primes
var primes
ty lli
*/

class $class$ {
  public:
    vector<$ty$> primes;

    $class$ ($ty$ maxval) {
      primes.push_back(2);
      for ($ty$ i = 3; i < maxval; i += 2) {
        bool is_prime = true;
        for ($ty$ other_prime : primes) {
          if (i % other_prime == 0) {
            is_prime = false;
            break;
          }
          if (other_prime*other_prime > i) {
            break;
          }
        }
        if (is_prime) {
          primes.push_back(i);
        }
      }
    }
};

//!slide plugin_input
$class$ $var$($maxval$);
