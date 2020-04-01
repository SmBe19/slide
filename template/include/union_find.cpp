/*!slide plugin_config
class UnionFind
var uf
size n
ty lli
*/

class $class$ {
  public:
    vector<$ty$> parent;
    vector<$ty$> size;

    $class$ ($ty$ n) {
      parent.resize(n);
      iota(parent.begin(), parent.end(), 0);
      size.resize(n, 1);
    }

    $ty$ f($ty$ el) {
      if (parent[el] == el) return el;
      parent[el] = f(parent[el]);
      return parent[el];
    }

    void u($ty$ a, $ty$ b) {
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
