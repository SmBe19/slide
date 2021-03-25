#include <bits/stdc++.h>

#define FOR(i,n) for(int i = 0; i < (n); i++)
#define FORS(i,s,n) for(int i = (s); i < (n); i++)
#define IN(...) lli __VA_ARGS__; input(__VA_ARGS__)
#define INT(T, ...) T __VA_ARGS__; input(__VA_ARGS__)
#define INV(v, n) vi v(n); FOR(iii, n) { cin >> v[iii]; } while(0)
#define ALL(x) x.begin(), x.end()
#define OUT(...) output(cout, __VA_ARGS__); cout << "\n"
#define DEB(...) output(cerr, __VA_ARGS__); cerr << endl
#define DEBS(x) cerr << x << endl
#define DEBV(v) cerr << #v << ": "; cerr << (v) << endl

using namespace std;

using lli=long long int;
template <typename T> using v=vector<T>;
using pi=pair<lli, lli>;
using vi=v<lli>;
using vvi=v<vi>;
using vp=v<pi>;
using vvp=v<vp>;

constexpr lli INF = numeric_limits<lli>::max();

void input() {}
template <typename T, typename... Ts>
void input(T &arg1, Ts&... args) {
  cin >> arg1;
  input(args...);
}


template <typename ST>
void output(ST &strm) {
  (void)strm; // unused
}
template <typename ST, typename T, typename... Ts>
void output(ST &strm, const T &arg1, const Ts&... args) {
  strm << arg1 << " ";
  output(strm, args...);
}

template<class T>
ostream& operator<<(ostream& stream, const vector<T>& values) {
    copy(begin(values), end(values), ostream_iterator<T>(stream, " "));
    return stream;
}

template <typename T, typename S>
void maxx(T &a, const S &b) {
  a = a > b ? a : b;
}

template <typename T, typename S>
void minn(T &a, const S &b) {
  a = a < b ? a : b;
}
