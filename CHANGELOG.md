Tantivy 0.4.0
==========================
- Raise the limit of number of fields (previously 256 fields)
- Removed u32 fields. They are replaced by u64 and i64 fields (#65)
- Replacing rustc_serialize by serde. Kudos to @KodrAus and @lnicola
- QueryParser:
  - Explicit error returned when searched for a term that is not indexed
  - Searching for a int term via the query parser was broken `(age:1)`
  - Searching for a non-indexed field returns an explicit Error
  - Phrase query for non-tokenized field are not tokenized by the query parser.

Tantivy 0.3.1
==========================

- Expose a method to trigger files garbage collection


Tantivy 0.3
==========================


Special thanks to @Kodraus @lnicola @Ameobea @manuel-woelker @celaus
for their contribution to this release.

Thanks also to everyone in tantivy gitter chat 
for their advise and company :)

https://gitter.im/tantivy-search/tantivy


Warning:

Tantivy 0.3 is NOT backward compatible with tantivy 0.2 
code and index format.
You should not expect backward compatibility before 
tantivy 1.0.


New Features
------------

- Delete. You can now delete documents from an index.
- Support for windows (Thanks to @lnicola)


Various Bugfixes & small improvements
----------------------------------------

- Added CI for Windows (https://ci.appveyor.com/project/fulmicoton/tantivy)
Thanks to @KodrAus ! (#108)
- Various dependy version update (Thanks to @Ameobea) #76
- Fixed several race conditions in `Index.wait_merge_threads`
- Fixed #72. Mmap were never released.
- Fixed #80. Fast field used to take an amplitude of 32 bits after a merge. (Ouch!)
- Fixed #92. u32 are now encoded using big endian in the fst
  in order to make there enumeration consistent with
  the natural ordering.
- Building binary targets for tantivy-cli (Thanks to @KodrAus)
- Misc invisible bug fixes, and code cleanup.
- Use 




