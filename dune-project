(lang dune 3.7)
(using menhir 2.0)

(name simpl)

(generate_opam_files true)

(source
 (github muqiuhan/simpl))

(authors "Muqiu Han")

(maintainers "Muqiu Han")

(license MIT)

(documentation https://github.com/muqiuhan/simpl)

(package
 (name simpl)
 (synopsis "A simple programming language with simple type system")
 (description "A simple programming language with simple type system")
 (depends ocaml dune alcotest)
 (tags
  (type language)))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/dune-files.html#dune-project
