class GraphW {
  public:
    long nodes, edges;
    vector<vector<pair<long, long>> adj;

    void read() {
      cin >> nodes >> edges;
      adj.resize(nodes);
      for(long i = 0; i < edges; i++) {
        long a, b, c;
        cin >> a >> b >> c;
        adj[a].emplace_back(b, c);
        adj[b].emplace_back(a, c);
      }
    }
};

//!slide plugin_input
GraphW g;
g.read();
