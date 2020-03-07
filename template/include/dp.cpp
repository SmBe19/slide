/*!slide plugin_config
inp i
out i
class DP
var dp
map true
maxval n
neut -1
*/

// implement £ty:out£ $class$::solve_one(£tyvar:inp£)
class $class$ {
  public:
    //!slide plugin_if map
    map<tuple<£ty:inp£>, £ty:out£> memo;
    //!slide plugin_end_if
    //!slide plugin_if !map
    vector<£ty:out£> memo;

    $class$(long maxval) {
      memo.resize(maxval);
    }
    //!slide plugin_end_if

    £ty:out£ solve_one(£tyvar:inp£);

    £ty:out£ dp(£tyvar:inp£) {
      //!slide plugin_if map
      tuple<£ty:inp£> state = {£var:inp£};
      if (memo.find(state) == memo.end()) {
        memo[state] = solve_one(£var:inp£);
      }
      return memo[state];
      //!slide plugin_end_if
      //!slide plugin_if !map
      if (memo[£var:inp£] == $neut$) {
        memo[£var:inp£] = solve_one(£var:inp£);
      }
      return memo[£var:inp£];
      //!slide plugin_end_if
    }
};

//!slide plugin_input
//!slide plugin_if map
$class$ $var$;
//!slide plugin_end_if
//!slide plugin_if !map
$class$ $var$($maxval$);
//!slide plugin_end_if
