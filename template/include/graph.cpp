/*!slide plugin_config
dir false
weight false
list false
var g
class Graph
list_n n
list_ed ed
ty lli
*/

class $class$ {
  public:
    $ty$ nodes, edges;
    //!slide plugin_if weight
    vector<vector<pair<$ty$, $ty$>>> adj;
    //!slide plugin_end_if
    //!slide plugin_if !weight
    vector<vector<$ty$>> adj;
    //!slide plugin_end_if

    //!slide plugin_if !list
    void read() {
      cin >> nodes >> edges;
      adj.resize(nodes);
      for($ty$ i = 0; i < edges; i++) {
        //!slide plugin_if weight
        $ty$ a, b, c;
        cin >> a >> b >> c;
        adj[a].emplace_back(b, c);
        //!slide plugin_if !dir
        adj[b].emplace_back(a, c);
        //!slide plugin_end_if
        //!slide plugin_end_if
        //!slide plugin_if !weight
        $ty$ a, b;
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
    void from_list($ty$ n, vector<tuple<$ty$, $ty$, $ty$>>& list) {
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
    void from_list($ty$ n, vector<pair<$ty$, $ty$>>& list) {
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

    //!slide plugin_if dir
    vector<$ty$> toposort() {
      vector<$ty$> outDegree(nodes);
      for (auto& neighbors : adj) {
        for (auto& neighbor : neighbors) {
          outDegree[nodeIdx(neighbor)]++;
        }
      }
      queue<$ty$> zeroDegs;
      for ($ty$ i = 0; i < nodes; i++) {
        if (outDegree[i] == 0) {
          zeroDegs.push(i);
        }
      }
      vector<$ty$> res;
      while (!zeroDegs.empty()) {
        $ty$ ae = zeroDegs.front();
        zeroDegs.pop();
        res.push_back(ae);
        for (auto& neighbor : adj[ae]) {
          outDegree[nodeIdx(neighbor)]--;
          if (outDegree[nodeIdx(neighbor)] == 0) {
            zeroDegs.push(nodeIdx(neighbor));
          }
        }
      }
      return res;
    }
    //!slide plugin_end_if

    vector<$ty$> bfs($ty$ start) {
      queue<$ty$> q;
      vector<$ty$> dist(nodes, INF);
      q.push(start);
      dist[start] = 0;
      while (!q.empty()) {
        $ty$ ae = q.front();
        q.pop();
        for (auto neighbor : adj[ae]) {
          if (dist[nodeIdx(neighbor)] == INF) {
            dist[nodeIdx(neighbor)] = dist[ae] + 1;
            q.push(nodeIdx(neighbor));
          }
        }
      }
      return dist;
    }

    //!slide plugin_if weight
    vector<$ty$> dijkstra($ty$ start) {
      vector<$ty$> dist(nodes, INF);
      set<pair<$ty$, $ty$>> pq;
      pq.emplace(0, start);
      dist[start] = 0;
      while(!pq.empty()) {
        auto ae = *pq.begin();
        pq.erase(pq.begin());
        for (auto& neighbor : adj[ae.second]) {
          $ty$ newdist = neighbor.second + ae.first;
          if (newdist < dist[neighbor.first]) {
            auto oldElement = pq.find({neighbor.first, dist[neighbor.first]});
            if (oldElement != pq.end()) {
              pq.erase(oldElement);
            }
            dist[neighbor.first] = newdist;
            pq.emplace(newdist, neighbor.first);
          }
        }
      }
      return dist;
    }
    //!slide plugin_end_if

    //!slide plugin_if weight
    $ty$ mst() {
      $ty$ res = 0;
      vector<int> vis(nodes);
      vector<$ty$> dist(nodes, INF);
      dist[0] = 0;
      set<pair<$ty$, $ty$>> pq;
      pq.emplace(0, 0);
      while(!pq.empty()) {
        auto ae = *pq.begin();
        pq.erase(pq.begin());
        if (vis[ae.second]) {
          continue;
        }
        res += dist[ae.second];
        vis[ae.second]++;
        for (auto& neighbor : adj[ae.second]) {
          if (!vis[neighbor.first] && neighbor.second < dist[neighbor.first]) {
            auto oldElement = pq.find({neighbor.first, dist[neighbor.first]});
            if (oldElement != pq.end()) {
              pq.erase(oldElement);
            }
            dist[neighbor.first] = neighbor.second;
            pq.emplace(neighbor.second, neighbor.first);
          }
        }
      }
      for($ty$ d : dist) {
        if (d == INF) {
          return INF;
        }
      }
      return res;
    }
    //!slide plugin_end_if

  private:
    //!slide plugin_if weight
    $ty$ nodeIdx (pair<$ty$, $ty$> neighbor) {
      return neighbor.first;
    }
    //!slide plugin_end_if
    //!slide plugin_if !weight
    $ty$ nodeIdx ($ty$ neighbor) {
      return neighbor;
    }
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
