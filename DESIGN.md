# Boring RPC

## Design Goals

### Strictly Typed

1. Schema Based

2. Union types

3. No Subtyping

### Friendly with status quo

1. Compatible with JSON, easy to read & write and debug by human.
2. Similar to TypeScript, easy to learn by most Web devs.

### Versioning

1. Automatically decide if two versions of the API are compatible

### Meta-programmable

## Example Syntax

TODO: codeblocks should be tagged with `br`

```ts
// Scalar Types
// Scalars are `JSONValue`s that have an atomic semantic and its mapping with the actual language is user-provided.
// e.g. you can encode `Date` in the format of `"2024-01-01"` or `"20240101"` in string, or even `{ "year": 2024, "month": 1, "day": 1 }`
import { Date, URL } from 'my-scalars'

// Metadata
// Metadatas are also user-provided piece of info that can be attached to each level of the syntax
// Users can write plugins in the using language against them, e.g. limiting the length of strings.
import { maxLength } from 'my-metadata'

// Struct Type
type TextMessage = {
    id: i64,

    @maxLength(20)
    text: string,
    created: Date,

    @label('Is Public?')
    isPublic: boolean,
}

type FileMessage = {
    id: i64,
    fileUrl: URL,
    fileSize: u64,
    fileType: FileType,
    created: Date,
}

// Enums
enum FileType {
    Image,
    Audio,
    Unknown,
}

// Union Types
type Message = TextMessage | FileMessage;

// Procedure Macros
//
// Programmatic manipulation of `type`s and `interface`s are possible
// Here it equals to `{ text: TextMessage['text'], isPublic: TextMessage['isPublic'] }`
// Executed after initial parsing and before validation, having access to purely syntax information.
//
// Can be attached to type and fields.
//
// User bring their own using wasm/JavaScript?
//
#pick(TextMessage, ["text", "isPublic"])
type TextMessageCreate = {}

type ReportTicket = {
    @primaryKey('Message')
    messageId: i64;

    #default_maxlen
    reason: string;
}

// Type Reference
type FileMessageCreate = {
    fileUrl: FileMessage.fileUrl
}

// RPC interface definition
service Chat {
    getMessageById(id: i64): Message;
    listMessages(): Message[];
}
```

## References

1. Rust Analyzer Syntax: https://rust-analyzer.github.io/blog/2020/10/24/introducing-ungrammar.html
    1. Syntax
        1. https://github.com/rust-lang/rust-analyzer/blob/master/xtask/src/codegen/grammar.rs
        2. https://github.com/rust-lang/rust-analyzer/blob/master/crates/syntax/rust.ungram
        3. https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/syntax.md
    4. Parser: 
        1. Pratt Parser: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
        1. https://github.com/rust-lang/rust-analyzer/blob/master/crates/parser/src/event.rs

2. Rust Analyzer Architecture
    1. https://github.com/rust-lang/rust-analyzer/blob/d10cdd25e7a5509252aa2af077b66c679e313ba0/docs/dev/architecture.md

3. Linked List
    1. https://rust-unofficial.github.io/too-many-lists/

4. TypeScript
    1. checker
        1. checking: 
            1. checkSourceElement
                1. checkVariableStatement
                2. checkIdentifier
                    1. getResolvedSymbol
                        1. resolveName <- createNameResolver
        2. Public API: 
            1. getSuggestionDiagnostics
                1. checkSourceFileWithEagerDiagnostics
    2. utilities
        1. createNameResolver