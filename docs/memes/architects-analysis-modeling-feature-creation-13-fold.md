# Architects: Analysis and Modeling - Feature Creation - 13-Fold Division

This document applies a 13-fold division to the 'Feature Creation' facet of 'Feature Engineering' under the 'Architects' archetype, providing a deeper level of granularity for generating new features from raw data.

## 1. Polynomial Features

Creating new features by raising existing numerical features to a power or by multiplying them together to capture non-linear relationships.

## 2. Interaction Features

Combining two or more features to capture their synergistic or interdependent effects on the target variable.

## 3. Ratio Features

Creating new features by dividing one existing numerical feature by another, often revealing proportional relationships.

## 4. Difference Features

Creating new features by subtracting one existing numerical feature from another, highlighting changes or disparities.

## 5. Aggregation Features

Summarizing data (e.g., mean, sum, count, min, max, standard deviation) over a group, window, or time period.

## 6. Binning/Discretization

Converting continuous numerical features into categorical bins or intervals, simplifying relationships and handling outliers.

## 7. One-Hot Encoding (for new features)

Creating binary (0/1) features for each category of a nominal variable, often after binning or for new categorical features.

## 8. Date/Time Features

Extracting components like day of week, month, year, hour, minute, or holiday indicators from timestamp or date fields.

## 9. Geospatial Features

Creating features based on location data, such as distance to a landmark, area of a region, or density of points.

## 10. Text-Derived Features

Creating features from text data, such as word count, character count, average word length, sentiment score, or readability scores.

## 11. Image-Derived Features

Creating features from image data, such as color histograms, texture descriptors, edge detection counts, or object presence indicators.

## 12. External Data Enrichment

Augmenting existing data with new features obtained from external sources, APIs, or public datasets.

## 13. Custom Domain-Specific Features

Creating features based on unique insights, business rules, or expert knowledge derived from the specific problem domain.

---

## Visual Representation (Mermaid Diagram)

```mermaid
graph TD
    A[Architects: Feature Creation] --> B(1. Polynomial Features)
    A --> C(2. Interaction Features)
    A --> D(3. Ratio Features)
    A --> E(4. Difference Features)
    A --> F(5. Aggregation Features)
    A --> G(6. Binning/Discretization)
    A --> H(7. One-Hot Encoding (for new features))
    A --> I(8. Date/Time Features)
    A --> J(9. Geospatial Features)
    A --> K(10. Text-Derived Features)
    A --> L(11. Image-Derived Features)
    A --> M(12. External Data Enrichment)
    A --> N(13. Custom Domain-Specific Features)
```
