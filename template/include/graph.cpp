/*!slide plugin_config
dir false
weight false
var g
class Graph
*/

class $class$ {
  public:
    long nodes, edges;
    //!slide plugin_if weight
    vector<vector<pair<long, long>> adj;
    //!slide plugin_end_if
    //!slide plugin_if !weight
    vector<vector<long>> adj;
    //!slide plugin_end_if

    void read() {
      cin >> nodes >> edges;
      adj.resize(nodes);
      for(long i = 0; i < edges; i++) {
        //!slide plugin_if weight
        long a, b, c;
        cin >> a >> b >> c;
        adj[a].emplace_back(b, c);
        //!slide plugin_if !dir
        adj[b].emplace_back(a, c);
        //!slide plugin_end_if
        //!slide plugin_end_if
        //!slide plugin_if !weight
        long a, b;
        cin >> a >> b;
        adj[a].push_back(b);
        //!slide plugin_if !dir
        adj[b].push_back(a);
        //!slide plugin_end_if
        //!slide plugin_end_if
      }
    }
};

//!slide plugin_input
$class$ $var$;
$var$.read();
