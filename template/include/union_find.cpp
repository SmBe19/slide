/*!slide plugin_config
class UnionFind
var uf
size n
*/

class $class$ {
  public:
    vector<long> parent;
    vector<long> size;

    $class$ (long n) {
      parent.resize(n);
      iota(parent.begin(), parent.end(), 0);
      size.resize(n, 1);
    }

    long f(long el) {
      if (parent[el] == el) return el;
      parent[el] = f(parent[el]);
      return parent[el];
    }

    void u(long a, long b) {
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
