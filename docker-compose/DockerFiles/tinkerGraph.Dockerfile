FROM tinkerpop/gremlin-server:3.6

COPY air-routes.graphml data/
COPY tinkergraph/air-routes.properties conf/
#COPY load-air-routes-graph.groovy scripts/
COPY tinkergraph/gremlin-server-graph-binary.yaml conf/


# CMD [ "conf/gremlin-server-graph-binary.yaml" ]