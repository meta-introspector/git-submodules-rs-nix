### C4 System Context Diagram

This diagram shows the Meta-Introspector Project as a whole, and its interactions with external users and systems.

```mermaid
graph TD
    A[Developer] -->|Uses| B(Meta-Introspector Project)
    C[AI Agent] -->|Interacts with| B
    B -->|Reads from| D[Git Repository]
    B -->|Processes| E[Markdown Files (CRQ, SOP)]
    B -->|Uses| F[Cargo/Rust Toolchain]

    subgraph Meta-Introspector Project
        B
    end
```

### C4 Container Diagram

This diagram zooms into the Meta-Introspector Project and shows the main applications/services (containers) within it, and their interactions.

```mermaid
graph TD
    subgraph Meta-Introspector Project
        subgraph Rust Applications
            subgraph submodules (Rust Library)
                L[CRQ Parser Module]
                M[Predicate Classifier Module]
            end
            C1[crq-parser-cli (Rust CLI)]
            C2[state-word-reporter (Rust CLI)]
        end
        G[Git (External Tool)]
        H[Cargo (External Tool)]
        I[Markdown Files (CRQ, SOP)]
    end

    C1 -->|Uses| L
    C2 -->|Uses| M
    C1 -->|Reads| I
    C2 -->|Reads| I
    C1 -->|Built by| H
    C2 -->|Built by| H
    H -->|Manages| G
```

### UML Class Diagram for Lattice Model

This diagram illustrates the key structs and traits involved in the lattice model, along with their relationships and important fields/methods.

```mermaid
classDiagram
    direction LR

    class ValueType {
        <<enum>>
        Bit
        ThreeValue
        FiveValue
        P7(u8)
        P11(u8)
        P13(u8)
        P17(u8)
        P19(u8)
        +count(): u8
        +zos_sequence(): Vec<ValueType>
    }

    class HasValueCount {
        <<trait>>
        +value_count(): u8
    }

    class Instance~T~ {
        +id: String
        +n_gram_size: u8
        +units: Vec<T>
        +new(id: &str, n_gram_size: u8, units: Vec<T>): Self
    }

    class LatticeLayer~T~ {
        +value_type: ValueType
        +instances: Vec<Instance<T>>
        +new(value_type: ValueType): Self
        +add_instance(instance: Instance<T>)
    }

    class LatticeLayerTrait {
        <<trait>>
        +describe_layer()
    }

    class Lattice {
        +name: String
        +layers: Vec<Box<dyn LatticeLayerTrait>>
        +new(name: &str): Self
        +add_layer(layer: LatticeLayer<T>)
    }

    class PredicateClassifier {
        -target_predicates: Vec<String>
        +new(predicates: Vec<&str>): Self
        +extract_word_predicates(text: &str): Vec<WordPredicate>
        +get_target_predicates(): &Vec<String>
    }

    class WordPredicate {
        +0: bool
        +new(value: bool): Self
    }

    ValueType --o LatticeLayer : contains
    HasValueCount <|.. Instance : T implements
    Instance --o LatticeLayer : contains
    LatticeLayer <|.. LatticeLayerTrait : implements
    Lattice --o LatticeLayerTrait : contains (Box<dyn>)
    PredicateClassifier ..> WordPredicate : creates
    PredicateClassifier ..> Instance : uses (indirectly via build_zos_lattice)
    WordPredicate --|> HasValueCount : implements (for bool)
```