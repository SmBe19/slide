/*!slide plugin_config
maxval 100002
class Primes
var primes
*/

class $class$ {
  public:
    vector<long long int> primes;

    $class$ (long long int maxval) {
      primes.push_back(2);
      for (long long int i = 3; i < maxval; i += 2) {
        bool is_prime = true;
        for (long long int other_prime : primes) {
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
