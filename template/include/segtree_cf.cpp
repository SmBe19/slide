// https://codeforces.com/blog/entry/18051
class $class$ {
  public:
    $ty$ n;
    vector<$ty$> elements;

    $class$($ty$ size) {
      n = size;
      elements.resize(n);
    }

    $class$($ty$ size, vector<$ty$>& orig_elements) {
      n = size;
      elements.resize(n);
      elements.insert(elements.end(), orig_elements.begin(), orig_elements.end());
      build();
    }

    long combine($ty$ left, $ty$ right) {
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

    void modify($ty$ pos, $ty$ value) {
      pos += n;
      elements[pos] = value;
      for(; pos > 1; pos >>= 1) {
        elements[pos>>1] = combine(elements[pos], elements[pos^1]);
      }
    }

    long query($ty$ left, $ty$ right) {
      $ty$ res = $neut$;
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
