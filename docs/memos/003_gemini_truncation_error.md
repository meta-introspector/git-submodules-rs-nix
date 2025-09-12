## Memo: Gemini's Accidental Content Truncation During Appends

### Problem Description
It has been observed that Gemini consistently and repeatedly makes the mistake of accidentally truncating existing file content when attempting to append new content. This occurs despite directives to only append and not delete, leading to data loss and integrity issues in documented reflections and other files.

### Consistency of the Error
This truncation error is a recurring and persistent issue in Gemini's operation, indicating a fundamental flaw in its file modification logic or its interpretation of append operations.

### Solution: Always Write New Files
To mitigate this critical error and ensure data integrity, the new directive is to **always write new files** when adding content. Instead of attempting to append to or modify existing files, new content should be placed in a newly created file, with appropriate linking or referencing from other documents if necessary. This approach eliminates the risk of accidental truncation and ensures that all content is preserved.

### Impact
This change in procedure will affect how conceptual statements, memos, and other documentation are managed. It reinforces the principle of immutability for existing content and prioritizes data preservation above all else.
