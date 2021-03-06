
// an init script that returns a Map allows explicit setting of global bindings.
def globals = [:]

// defines a sample LifeCycleHook that prints some output to the Gremlin Server console.
// note that the name of the key in the "global" map is unimportant.
globals << [hook : [
        onStartUp: { ctx ->
            ctx.logger.info("Executed once at startup of Gremlin Server.")

            // See https://github.com/experoinc/gremlin-lang-intro for further details.
            ctx.logger.info("Loading Air Routes data set into graph [airlines] from data/air-routes.graphml. Use TraversalSource: [g]")
            airlines.io(graphml()).readGraph('data/air-routes.graphml')
            airlines.tx().commit()
        },
        onShutDown: { ctx ->
            ctx.logger.info("Executed once at shutdown of Gremlin Server.")
        }
] as LifeCycleHook]

// Statically defined graphs (gremlin-server.yaml)
globals << [g : airlines.traversal()]

// Dynamically defined graphs (ConfiguredGraphFactory)
// def getGraphs() {
//     def graphNames = ConfiguredGraphFactory.getGraphNames();
//     def graphMaps = [:];
//     for (graphName in graphNames) {
//         def g = ConfiguredGraphFactory.open(graphName);
//         graphMaps.put(graphName, g.traversal());
//     }
//     return graphMaps;
// }

// globals << getGraphs()