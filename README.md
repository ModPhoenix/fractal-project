
# Fractal project

Fractal is an innovative platform that creates a context-based interconnected digital repository of knowledge, mapping information and its dependencies. It allows users to:

- Explore the context-based interconnected knowledge graph
- Create and share knowledge trees
- Create educational content
- Create snapshots (sub-graphs) of knowledge areas (Frontend, Backend, DevOps, Sales, Marketing, etc.)
- Map their own knowledge onto a shared knowledge tree
- Compare their knowledge with others
- Get their expertise validated by others
- Visualize their skills and competencies
- [Potential] for creating a real-time history platform of our reality, similar to Wikipedia
- [Potential] for social news network for current things happening in the world

Fractal helps individuals understand their own capabilities while enabling organizations to better assess and leverage talent. By creating a context-based interconnected knowledge graph, Fractal aims to revolutionize how we organize, explore, learn, visualize, and validate knowledge, events, history, and human expertise.

Key features:

- Explore interconnected knowledge graph
- Knowledge mapping and visualization
- Peer validation of skills
- Competency assessment for hiring and development
- Educational content creation and sharing

Fractal has applications for individuals, professionals, businesses, recruiters, and academic institutions. Join us in building the future of knowledge management and skills assessment!

## WIP Cypher schema representation

```
[Fractal]
   |
   |-- HAS_CHILD --> [Fractal]
   |
   |-- PROVIDES_CONTEXT_FOR --> [Fractal]
   |
   |-- HAS_KNOWLEDGE --> [Knowledge]
                             |
                             |-- IN_CONTEXT --> [Fractal]
```

Example schema in Cypher:

```cypher
// Define constraints
CREATE CONSTRAINT ON (f:Fractal) ASSERT f.id IS UNIQUE;
CREATE CONSTRAINT ON (f:Fractal) ASSERT f.name IS UNIQUE;
CREATE CONSTRAINT ON (k:Knowledge) ASSERT k.id IS UNIQUE;

// Define indexes
CREATE INDEX ON :Fractal(name);
CREATE INDEX ON :Knowledge(id);

// Nodes
(:Fractal {
  id: UUID,
  name: String,
  createdAt: DateTime,
  updatedAt: DateTime
})

(:Knowledge {
  id: UUID,
  content: String
})

// Relationship structure:
(:Fractal)-[:HAS_CHILD]->(:Fractal)
(:Fractal)-[:PROVIDES_CONTEXT_FOR]->(:Fractal)
(:Fractal)-[:HAS_KNOWLEDGE]->(:Knowledge)
(:Knowledge)-[:IN_CONTEXT]->(:Fractal)
```

## Project Status

Fractal is currently in the research and development phase, focusing on creating a proof of concept:

- Exploring and refining the core concepts of the context-based interconnected knowledge graph
- Developing initial prototypes for key features
- Investigating optimal technical solutions for scalability and performance
- Conducting user research to validate assumptions and gather feedback

We are not yet at a stage for public release or wide-scale testing. However, we're excited about the potential of Fractal and are working diligently to bring this innovative platform to life.
