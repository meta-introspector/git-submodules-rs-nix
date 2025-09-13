# Builders: Data Extraction and Parsing - Extraction Mechanisms - 11-Fold Division

This document applies an 11-fold division to the 'Extraction Mechanisms' facet of 'Data Extraction and Parsing' under the 'Builders' archetype, providing a deeper level of granularity for the methods used to pull data from various sources.

## 1. Regular Expressions (Regex)

Pattern matching for extracting specific text sequences from unstructured or semi-structured text-based data.

## 2. XPath/CSS Selectors

Navigating and selecting specific elements or attributes from XML or HTML documents, commonly used in web scraping.

## 3. API Calls

Programmatic access to data through defined interfaces (REST, SOAP, GraphQL), sending requests and receiving structured responses.

## 4. Web Scraping Libraries

Tools and frameworks specifically designed for automating the extraction of data from websites, handling HTTP requests, and parsing HTML.

## 5. Database Queries

Using SQL or NoSQL query languages to retrieve structured data from various types of databases.

## 6. Event Listeners/Hooks

Capturing data as events occur in streaming systems, message queues, or real-time data pipelines.

## 7. File Parsers

Libraries or custom code designed to read and interpret specific file formats (e.g., PDF, Excel, CSV, JSON, XML) and extract their content.

## 8. Optical Character Recognition (OCR)

Extracting text from images, scanned documents, or PDFs that are not natively text-searchable.

## 9. Natural Language Processing (NLP)

Techniques for extracting entities, relationships, sentiments, or other structured information from unstructured text data.

## 10. Change Data Capture (CDC)

Identifying and capturing changes (inserts, updates, deletes) in a database in real-time or near real-time, often for data replication or warehousing.

## 11. Manual Extraction

Human intervention for extracting data from highly complex, unstructured, or unique sources where automation is not feasible.

---

## Visual Representation (Mermaid Diagram)

```mermaid
graph TD
    A[Builders: Extraction Mechanisms] --> B(1. Regular Expressions (Regex))
    A --> C(2. XPath/CSS Selectors)
    A --> D(3. API Calls)
    A --> E(4. Web Scraping Libraries)
    A --> F(5. Database Queries)
    A --> G(6. Event Listeners/Hooks)
    A --> H(7. File Parsers)
    A --> I(8. Optical Character Recognition (OCR))
    A --> J(9. Natural Language Processing (NLP))
    A --> K(10. Change Data Capture (CDC))
    A --> L(11. Manual Extraction)
```
