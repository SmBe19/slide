/*!slide plugin_config
dir false
weight false
list false
var g
class Graph
list_n n
list_ed ed
*/

class $class$ {
  public:
    long long int nodes, edges;
    //!slide plugin_if weight
    vector<vector<pair<long, long>> adj;
    //!slide plugin_end_if
    //!slide plugin_if !weight
    vector<vector<long>> adj;
    //!slide plugin_end_if

    //!slide plugin_if !list
    void read() {
      cin >> nodes >> edges;
      adj.resize(nodes);
      for(long long int i = 0; i < edges; i++) {
        //!slide plugin_if weight
        long long int a, b, c;
        cin >> a >> b >> c;
        adj[a].emplace_back(b, c);
        //!slide plugin_if !dir
        adj[b].emplace_back(a, c);
        //!slide plugin_end_if
        //!slide plugin_end_if
        //!slide plugin_if !weight
        long long int a, b;
        cin >> a >> b;
        adj[a].push_back(b);
        //!slide plugin_if !dir
        adj[b].push_back(a);
        //!slide plugin_end_if
        //!slide plugin_end_if
      }
    }
    //!slide plugin_end_if

    //!slide plugin_if list
    //!slide plugin_if weight
    void from_list(long long int n, vector<tuple<long long int, long long int, long long int>>& list) {
      adj.resize(n);
      for (auto& edge : list) {
        adj[get<0>(edge)].emplace_back(get<1>(edge), get<2>(edge));
        //!slide plugin_if !dir
        adj[get<1>(edge)].emplace_back(get<0>(edge), get<2>(edge));
        //!slide plugin_end_if
      }
    }
    //!slide plugin_end_if
    //!slide plugin_if !weight
    void from_list(long long int n, vector<pair<long long int, long long int>>& list) {
      adj.resize(n);
      for (auto& edge : list) {
        adj[edge.first].push_back(edge.second);
        //!slide plugin_if !dir
        adj[edge.second].push_back(edge.first);
        //!slide plugin_end_if
      }
    }
    //!slide plugin_end_if
    //!slide plugin_end_if
};

//!slide plugin_input
$class$ $var$;
//!slide plugin_if list
$var$.from_list($list_n$, $list_ed$);
//!slide plugin_end_if
//!slide plugin_if !list
$var$.read();
//!slide plugin_end_if
