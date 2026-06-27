# Writing Design Documents

## Two-Phase Design Process

### Phase 1: System Design
- Focus on **requirements, behavior, and logical completeness**.
- **NO code snippets** of any kind. Describe what the system does, not how it's implemented.
- Include: user stories, functional requirements, non-functional requirements, architecture diagrams (Mermaid), data flow, error scenarios.

### Phase 2: Code Design
- Translate system design into implementation specifications.
- Define: data contracts (structs, enums), module boundaries, function signatures, error types.
- Use the terminology established in system design.
- Include: directory structure, module responsibilities, API contracts, test strategy.

## Document Format

### YAML Frontmatter (required on all design docs)
```yaml
---
system: excel-to-json
sub-system: <subsystem name in Chinese>
part: <后端|前端|流程图|数据模型>
description: <one-line description in Chinese>
version: 1.0
last_updated: YYYY-MM-DD
---
```

### Directory Structure
```
docs/design/
  NN.system-name/              # Level 1: system
    NN.NN.sub-system/          # Level 2: sub-system
      NN.NN.descriptive_{type}.md  # Level 3: document
```

### Document Type Suffixes
| Suffix | Purpose |
|--------|---------|
| `_backend` | Backend/API specifications |
| `_frontend` | Frontend/UI specifications |
| `_process_diagram` | Mermaid flow/sequence/state diagrams |
| `_data_model` | Data model definitions |
| `_design` | Comprehensive design document |

## Conventions
- **Language**: Document body in Chinese. Technical identifiers (API paths, type names) in English.
- **Diagrams**: Use Mermaid (`flowchart TD`, `sequenceDiagram`, `stateDiagram-v2`).
- **Tables**: Use Markdown tables for structured data (config constants, API parameters, error codes).
- **Change log**: Include a change log table at the bottom of each document.
