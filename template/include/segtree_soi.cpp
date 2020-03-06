// https://soi.ch/wiki/segtree/
class $class$ {
  public:
    long n;
    vector<vector<long>> tree;

    $class$(long size) {
      n = size;
      tree.push_back(vector<long>(n));
      init();
    }

    $class$(long size, vector<long>& orig_elements) {
      n = size;
      tree.push_back(orig_elements);
      init();
    }

    long combine(long left, long right) {
      //!slide plugin_if op_func
      return $op$(left, right);
      //!slide plugin_end_if
      //!slide plugin_if !op_func
      return left $op$ right;
      //!slide plugin_end_if
    }

    void init() {
      for(int i = 1; tree[i-1].size() > 1; i++){
          int size = tree[i-1].size()/2;
          tree.push_back(vector<long>(size));
      }
    }

    void modify(long pos, long value) {
      tree[0][pos] = value;
      long k = pos>>1;
      for(int l = 1; l < (int)tree.size(); l++){
        if(k >= tree[l].size()) break;
        tree[l][k] = combine(tree[l-1][2*k], tree[l-1][2*k+1]);
        k >>= 1;
      }
    }

    long query(long left, long right) {
      int a = left, b = right;
      long v_l = $neut$, v_r = $neut$;
      for(int i = 0; a <= b; i++){
        if(a%2 == 1){
          v_l = combine(v_l, tree[i][a]);
          a++;
        }
        if(b%2 == 0){
          v_r = combine(tree[i][b], v_r);
          b--;
        }
        a >>= 1, b >>= 1;
      }
      return combine(v_l, v_r);
    }
};
