# Fractal project

Fractal is an innovative platform that creates an interconnected digital repository of knowledge, mapping information and its dependencies. It allows users to:

- Map their own knowledge onto a shared knowledge tree
- Get their expertise validated by others
- Visualize their skills and competencies

Fractal helps individuals understand their own capabilities while enabling organizations to better assess and leverage talent. By creating a "world tree of knowledge", Fractal aims to revolutionize how we organize, validate, and visualize human expertise.
Key features:

- Knowledge mapping and visualization
- Peer validation of skills
- Competency assessment for hiring and development
- Integration with educational institutions

Fractal has applications for professionals, businesses, recruiters, and academic institutions. Join us in building the future of knowledge management and skills assessment!

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
