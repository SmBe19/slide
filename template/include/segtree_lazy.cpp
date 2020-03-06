/*!slide plugin_config
class SegTree
sum_add false
min_set false
var tree
size n
*/

// https://soi.ch/wiki/advanced-segtree/
class $class$ {
public:
  //!slide plugin_if sum_add
  struct Value { long x, k; };
  Value identity_value() { return { 0, 0 }; }
  Value combine(Value a, Value b) { return { a.x+b.x, a.k+b.k }; }

  struct Update { long v; };
  Update identity_update() { return { 0 }; }
  Value apply_update(Update a, Value x) { return {x.x + a.v*x.k}; }
  Update combine_updates(Update a, Update b) { return { a.v + b.v }; }
  //!slide plugin_end_if
  //!slide plugin_if min_set
  struct Value { long x; };
  Value identity_value() { return { (long)1e9+1 }; }
  Value combine(Value a, Value b) { return { min(a.x, b.x) }; }

  struct Update { long v; };
  Update identity_update() { return { -1 }; }
  Value apply_update(Update a, Value x) {
    if (a.v == -1) return x;
    return { a.v };
  }
  Update combine_updates(Update a, Update b) {
    if (a.v == -1) return b;
    return a;
  }
  //!slide plugin_end_if

  long n;
  vector<$class$::Value> tree;
  vector<$class$::Update> lazy;

  // build segtree of size at least min_n
  $class$(long min_n)
    : n(next_power_of_two(min_n)),
      tree(2*n, identity_value()),
      lazy(2*n, identity_update()) {}

  // build segtree on an array of initial values
  $class$(vector<Value> const& base)
    : n(next_power_of_two(base.size())),
      tree(2*n, identity_value()),
      lazy(2*n, identity_update()) {
    for (long i=0; i<(long)base.size(); ++i)
      tree[n+i] = base[i];
    build(1, 0, n);
  }

  // combines all values in range [l, r)
  Value query(long l, long r) {
    assert(0 <= l && l < r && r <= n);
    return query_(l, r, 1, 0, n);
  }

  long next_power_of_two(unsigned x) { return 1<<__lg(x-1)+1; }

  // updates all values in range [l, r)
  void update(long l, long r, Update a) {
    assert(0 <= l && l < r && r <= n);
    return update_(a, l, r, 1, 0, n);
  }

private:
  // applies the update to the current node
  void apply(long pos, Update a) {
    tree[pos] = apply_update(a, tree[pos]);
    lazy[pos] = combine_updates(a, lazy[pos]);
  }

  // recomputes the value of position "pos", assuming lazy[pos]==identity_update()
  void recompute(long pos) {
    tree[pos] = combine(tree[2*pos], tree[2*pos+1]);
  }

  // pushes lazy values down the tree
  void propagate(long pos) {
    apply(2*pos, lazy[pos]);
    apply(2*pos+1, lazy[pos]);
    lazy[pos] = identity_update();
  }

  // build segtree assuming only leaf nodes are correct
  void build(long pos, long l, long r){
    if (r - l == 1) // leaf: do nothing
      return;
    long m = (l+r)/2;
    build(2*pos, l, m);
    build(2*pos+1, m, r);
    recompute(pos);
  }

  Value query_(long ql, long qr, long pos, long l, long r) {
    // completely contained: return value
    if (ql <= l && r <= qr) { return tree[pos]; }
    // not overlapping: return nothing
    if (qr <= l || r <= ql) { return identity_value(); }
    // otherwise: recurse
    propagate(pos);
    long m = (l+r)/2;
    Value ans_l = query_(ql, qr, 2*pos, l, m);
    Value ans_r = query_(ql, qr, 2*pos+1, m, r);
    return combine(ans_l, ans_r);
  }

  void update_(Update a, long ql, long qr, long pos, long l, long r) {
    // completely contained: update lazy
    if (ql <= l && r <= qr) { apply(pos, a); return; }
    // not overlapping: do nothing
    if (qr <= l || r <= ql) { return; }
    // otherwise: recurse
    propagate(pos);
    long m = (l+r)/2;
    update_(a, ql, qr, 2*pos, l, m);
    update_(a, ql, qr, 2*pos+1, m, r);
    recompute(pos);
  }
};

//!slide plugin_input
$class$ $var$($size$);
