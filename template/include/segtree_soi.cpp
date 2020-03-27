// https://soi.ch/wiki/segtree/
class $class$ {
  public:
    long long int n;
    vector<vector<long long int>> tree;

    $class$(long long int size) {
      n = size;
      tree.push_back(vector<long long int>(n));
      init();
    }

    $class$(long long int size, vector<long long int>& orig_elements) {
      n = size;
      tree.push_back(orig_elements);
      init();
    }

    long long int combine(long long int left, long long int right) {
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
          tree.push_back(vector<long long int>(size));
      }
    }

    void modify(long long int pos, long long int value) {
      tree[0][pos] = value;
      long long int k = pos>>1;
      for(int l = 1; l < (int)tree.size(); l++){
        if(k >= tree[l].size()) break;
        tree[l][k] = combine(tree[l-1][2*k], tree[l-1][2*k+1]);
        k >>= 1;
      }
    }

    long long int query(long long int left, long long int right) {
      int a = left, b = right;
      long long int v_l = $neut$, v_r = $neut$;
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
