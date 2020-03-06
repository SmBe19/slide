class Graph {
  public:
    long nodes, edges;
    vector<vector<long>> adj;

    void read() {
      cin >> nodes >> edges;
      adj.resize(nodes);
      for(long i = 0; i < edges; i++) {
        long a, b;
        cin >> a >> b;
        adj[a].push_back(b);
      }
    }
};

//!slide plugin_input
Graph g;
g.read();
