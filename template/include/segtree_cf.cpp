// https://codeforces.com/blog/entry/18051
class $class$ {
  public:
    long long int n;
    vector<long long int> elements;

    $class$(long long int size) {
      n = size;
      elements.resize(n);
    }

    $class$(long long int size, vector<long long int>& orig_elements) {
      n = size;
      elements.resize(n);
      elements.insert(elements.end(), orig_elements.begin(), orig_elements.end());
      build();
    }

    long combine(long long int left, long long int right) {
      //!slide plugin_if op_func
      return $op$(left, right);
      //!slide plugin_end_if
      //!slide plugin_if !op_func
      return left $op$ right;
      //!slide plugin_end_if
    }

    void build() {
      for(int i = n-1; i > 0; i--) {
        elements[i] = combine(elements[i<<1], elements[i<<1|1]);
      }
    }

    void modify(long long int pos, long long int value) {
      pos += n;
      elements[pos] = value;
      for(; pos > 1; pos >>= 1) {
        elements[pos>>1] = combine(elements[pos], elements[pos^1]);
      }
    }

    long query(long long int left, long long int right) {
      long long int res = $neut$;
      for (left += n, right += n; left < right; left >>= 1, right >>= 1) {
        if (left&1) {
          res = combine(elements[left++], res);
        }
        if (right&1) {
          res = combine(res, elements[--right]);
        }
      }
      return res;
    }
};
