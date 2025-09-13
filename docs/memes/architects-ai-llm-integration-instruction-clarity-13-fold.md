# Architects: AI and LLM Integration - Instruction Clarity - 13-Fold Division

This document applies a 13-fold division to the 'Instruction Clarity' facet of 'Prompt Engineering' under the 'Architects' archetype, providing a deeper level of granularity for ensuring prompts are unambiguous and easy to understand.

## 1. Conciseness

Using the fewest words necessary to convey the meaning, avoiding unnecessary filler or redundant phrases.

## 2. Specificity

Avoiding vague language and providing precise, concrete details about the desired output or task.

## 3. Unambiguity

Ensuring there is only one possible interpretation of the instruction, eliminating any potential for misreading or confusion.

## 4. Action-Oriented Verbs

Using strong, clear verbs that directly indicate the desired action or behavior from the LLM.

## 5. Target Audience Awareness

Tailoring the language, complexity, and level of detail to the LLM's capabilities, training data, and intended purpose.

## 6. Avoidance of Jargon

Using plain and universally understood language, unless technical terms are explicitly defined or are part of the expected domain.

## 7. Logical Flow

Structuring instructions in a clear, sequential, step-by-step, or hierarchical manner to guide the LLM's processing.

## 8. Consistency in Terminology

Using the same terms and phrases for the same concepts throughout the prompt to avoid confusion and reinforce meaning.

## 9. Positive Phrasing

Stating what the LLM *should do* or *include* rather than what it *should not do* or *exclude*, guiding it towards desired outcomes.

## 10. Examples (within prompt)

Providing short, illustrative examples directly within the prompt to clarify complex instructions or demonstrate desired output patterns.

## 11. Separation of Concerns

Clearly delineating different parts of the instruction (e.g., the main task, contextual information, output format requirements) for better organization.

## 12. Error Handling Instructions

Specifying how the LLM should respond to errors, ambiguities, or situations where it cannot fulfill the request, providing fallback guidance.

## 13. Iterative Refinement (of instructions)

Continuously improving the clarity and effectiveness of instructions based on analysis of LLM responses and desired outcomes.

---

## Visual Representation (Mermaid Diagram)

```mermaid
graph TD
    A[Architects: Instruction Clarity] --> B(1. Conciseness)
    A --> C(2. Specificity)
    A --> D(3. Unambiguity)
    A --> E(4. Action-Oriented Verbs)
    A --> F(5. Target Audience Awareness)
    A --> G(6. Avoidance of Jargon)
    A --> H(7. Logical Flow)
    A --> I(8. Consistency in Terminology)
    A --> J(9. Positive Phrasing)
    A --> K(10. Examples (within prompt))
    A --> L(11. Separation of Concerns)
    A --> M(12. Error Handling Instructions)
    A --> N(13. Iterative Refinement (of instructions))
```
