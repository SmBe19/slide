/*!slide plugin_config
class UnionFind
var uf
size n
*/

class $class$ {
  public:
    vector<long long int> parent;
    vector<long long int> size;

    $class$ (long long int n) {
      parent.resize(n);
      iota(parent.begin(), parent.end(), 0);
      size.resize(n, 1);
    }

    long long int f(long long int el) {
      if (parent[el] == el) return el;
      parent[el] = f(parent[el]);
      return parent[el];
    }

    void u(long long int a, long long int b) {
      a = f(a);
      b = f(b);
      if (a != b) {
        if (size[a] > size[b]) {
          swap(a, b);
        }
        parent[a] = b;
        size[b] += size[a];
      }
    }
};

//!slide plugin_input
$class$ $var$($size$);
