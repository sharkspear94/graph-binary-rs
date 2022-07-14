FROM openjdk:8-jdk


ARG VERSION=0.6.2

RUN apt-get update && \
    apt-get install -y wget unzip htop && \
    mkdir /workspace && \
    cd /workspace && \
    wget https://github.com/JanusGraph/janusgraph/releases/download/v$VERSION/janusgraph-full-$VERSION.zip && \
    unzip janusgraph-full-$VERSION.zip && \
    rm janusgraph-full-$VERSION.zip && \
    mv janusgraph-* janusgraph


WORKDIR /workspace/janusgraph

COPY janusgraph/janusgraph-config.properties conf/
COPY air-routes.graphml data/
COPY janusgraph/air-routes.properties conf/
COPY janusgraph/startup.groovy scripts/
COPY janusgraph/gremlin-server-graph-binary.yaml conf/
CMD ["bin/janusgraph-server.sh","console","gremlin-server-graph-binary.yaml"]