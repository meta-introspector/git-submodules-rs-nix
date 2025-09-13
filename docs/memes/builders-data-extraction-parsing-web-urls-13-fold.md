# Builders: Data Extraction and Parsing - Web URLs - 13-Fold Division

This document applies a 13-fold division to the 'Web URLs' facet of 'Source Identification' under the 'Builders' archetype, providing a deeper level of granularity for identifying and accessing data from web sources.

## 1. Protocols

Specifying the communication protocol used to access the web resource (e.g., HTTP, HTTPS, FTP, WebSocket), defining the transport mechanism.

## 2. Domain Names

The registered name of a website or server, serving as a human-readable address for web resources.

## 3. Subdomains

Divisions of a domain name that can be used to organize content or services (e.g., `blog.example.com`, `api.example.com`).

## 4. Paths

The specific location of a resource on the server, indicating the hierarchical structure within a domain.

## 5. Query Parameters

Key-value pairs appended to the URL after a question mark (`?`), used to pass dynamic content or filter results.

## 6. Fragments/Anchors

Sections within a document, indicated by a hash symbol (`#`), allowing direct linking to specific parts of a web page.

## 7. URL Encoding

Handling special characters in URLs by converting them into a format that can be transmitted over the internet without ambiguity.

## 8. URL Shortening

Using services or techniques to create shorter, more manageable URLs, often for sharing or tracking purposes.

## 9. Canonical URLs

The preferred version of a web page for search engines, used to prevent duplicate content issues.

## 10. Redirects

How URLs are redirected to different locations, including permanent (301) and temporary (302) redirects.

## 11. Robots.txt and Sitemaps

Files that guide web crawlers on which parts of a website to crawl or index (robots.txt) and provide a list of all available pages (sitemaps).

## 12. Authentication in URLs

Embedding credentials (e.g., username and password) directly within the URL, which is generally considered less secure.

## 13. Internationalized Domain Names (IDNs)

URLs that contain non-ASCII characters, allowing domain names to be expressed in local languages and scripts.

---

## Visual Representation (Mermaid Diagram)

```mermaid
graph TD
    A[Builders: Web URLs] --> B(1. Protocols)
    A --> C(2. Domain Names)
    A --> D(3. Subdomains)
    A --> E(4. Paths)
    A --> F(5. Query Parameters)
    A --> G(6. Fragments/Anchors)
    A --> H(7. URL Encoding)
    A --> I(8. URL Shortening)
    A --> J(9. Canonical URLs)
    A --> K(10. Redirects)
    A --> L(11. Robots.txt and Sitemaps)
    A --> M(12. Authentication in URLs)
    A --> N(13. Internationalized Domain Names (IDNs))
```
